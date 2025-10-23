// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::io::Write;
#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use super::TestUtils;
#[cfg(not(windows))] // Related functions not exposed on Windows
use crate::env;

#[must_use]
pub(crate) struct BinBuilder<'a> {
    utils: &'a TestUtils,
    name: &'a str,
    #[cfg(not(windows))] // Windows does not have file modes
    mode: Option<u32>,
    stdout: Option<String>,
    stderr: Option<String>,
    exit: Option<i32>,
    expected_args: Option<&'a [&'a str]>,
    /// If true test that all expected args and no additional ones are present.
    /// If false only check expected args are present and in order,
    /// but there can be additional arguments before, between, or after.
    expected_args_strict: bool,
    dest: BinaryDestination,
    program: &'static str,
}

impl<'a> BinBuilder<'a> {
    pub(super) fn new(utils: &'a TestUtils, name: &'a str) -> Self {
        Self {
            utils,
            name,
            #[cfg(not(windows))]
            mode: None,
            stdout: None,
            stderr: None,
            exit: None,
            expected_args: None,
            expected_args_strict: true,
            dest: BinaryDestination::Sysroot,
            program: BIN_PROGRAM,
        }
    }

    #[cfg(not(windows))] // Windows does not have file modes
    pub(crate) fn mode(mut self, mode: u32) -> Self {
        self.mode = Some(mode);
        self
    }

    pub(crate) fn stdout(mut self, stdout: &str) -> Self {
        self.stdout = Some(stdout.into());
        self
    }

    #[cfg(not(windows))] // Tests using this are excluded on Windows
    pub(crate) fn stderr(mut self, stderr: &str) -> Self {
        self.stderr = Some(stderr.into());
        self
    }

    pub(crate) fn exit(mut self, exit: i32) -> Self {
        self.exit = Some(exit);
        self
    }

    #[cfg(not(windows))] // Tests using this are excluded on Windows
    #[allow(non_snake_case)]
    pub(crate) fn behaves_like_vV(self) -> Self {
        let stdout = format!(
            "release: {}
            host: {}
            commit-hash: {}\n",
            env::CFG_RELEASE,
            env::SELFTEST_TARGET,
            env::SELFTEST_RUST_HASH.unwrap_or("unknown")
        );
        self.stdout(&stdout).stderr("").exit(0).expected_args(&["-vV"])
    }

    pub(crate) fn for_target(mut self, target: &str) -> Self {
        if let BinaryDestination::Sysroot = self.dest {
            self.dest = BinaryDestination::Target(target.into());
            self
        } else {
            panic!("called for_target() when another destination was already set");
        }
    }

    pub(crate) fn expected_args(mut self, args: &'a [&'a str]) -> Self {
        self.expected_args = Some(args);
        self
    }

    pub(crate) fn expected_args_strict(mut self, strict: bool) -> Self {
        self.expected_args_strict = strict;
        self
    }

    pub(crate) fn program_source(mut self, source: &'static str) -> Self {
        self.program = source;
        self
    }

    pub(crate) fn create(self) -> PathBuf {
        let bin_root = match self.dest {
            BinaryDestination::Sysroot => self.utils.sysroot.path().join("bin"),
            BinaryDestination::Target(target) => {
                self.utils.sysroot.path().join("lib").join("rustlib").join(target).join("bin")
            }
        };
        let bin = bin_root.join(self.name);
        // self.name might have included a sub-directory, so re-fetch the
        // containing directory's path
        let bin_dir = bin.parent().expect("path should have a parent");

        std::fs::create_dir_all(bin_dir).unwrap();
        if bin.exists() {
            std::fs::remove_file(&bin).unwrap();
        }

        let mut rustc_path = PathBuf::from(env!("CARGO"));
        rustc_path.pop();
        rustc_path.push("rustc");

        let mut rustc = Command::new(rustc_path);
        rustc.stdin(Stdio::piped());
        rustc.arg("-");
        rustc.arg("-o").arg(&bin);
        rustc.args(["--edition", "2021"]);
        if let Some(stdout) = self.stdout {
            rustc.env("OVERRIDE_STDOUT", stdout);
        }
        if let Some(stderr) = self.stderr {
            rustc.env("OVERRIDE_STDERR", stderr);
        }
        if let Some(exit) = self.exit {
            rustc.env("OVERRIDE_EXIT", exit.to_string());
        }
        if let Some(expected_args) = self.expected_args {
            rustc.env("EXPECTED_ARGS", expected_args.join("\t"));
        }
        rustc.env("EXPECTED_ARGS_STRICT", self.expected_args_strict.to_string());

        let mut rustc = rustc.spawn().unwrap();
        let stdin = rustc.stdin.as_mut().unwrap();
        stdin.write_all(self.program.as_bytes()).unwrap();
        let res = rustc.wait_with_output().unwrap();
        assert!(res.status.success(), "stdout: \n{}", String::from_utf8_lossy(&res.stdout));

        #[cfg(not(windows))]
        if let Some(mode) = self.mode {
            let mut perms = bin.metadata().unwrap().permissions();
            perms.set_mode(mode);
            std::fs::set_permissions(&bin, perms).unwrap();
        }

        bin
    }
}

enum BinaryDestination {
    Sysroot,
    Target(String),
}

const BIN_PROGRAM: &str = r#"
fn main() {
    if let Some(expected_args) = option_env!("EXPECTED_ARGS") {
        let expected = expected_args.split("\t").collect::<Vec<_>>();
        let mut found = std::env::args().skip(1).collect::<Vec<_>>();
        match option_env!("EXPECTED_ARGS_STRICT") {
            Some("true") => assert_eq!(expected, found),
            Some("false") => {
                // Validate each of the args is present in the order provided,
                // ignore the rest.
                for item in expected {
                    let val = found.iter().enumerate().find(|(_, v)| *v == &item.to_string());
                    assert!(val.is_some(), "Did not find argument");
                    // Get rid of all the previous args so that we can validate the expected
                    // args are in-order.
                    found = found.split_off(val.unwrap().0);
                }
            },
            _ => panic!("Invalid env passed to `EXPECTED_ARGS_STRICT`, should be 'true' or 'false'"),
        }
    }
    if let Some(stdout) = option_env!("OVERRIDE_STDOUT") {
        print!("{stdout}");
    }
    if let Some(stderr) = option_env!("OVERRIDE_STDERR") {
        eprint!("{stderr}");
    }
    if let Some(code) = option_env!("OVERRIDE_EXIT") {
        std::process::exit(code.parse().unwrap());
    }
}
"#;

#[test]
fn test_missing_args() {
    let utils = TestUtils::new();
    let expected = ["--waffle", "--pancake", "-lefse", "stroopwaffel"];
    let strict_bin = utils
        .bin("strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(true)
        .create();

    let non_strict_bin = utils
        .bin("non-strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(false)
        .create();

    assert_eq!(
        Command::new(strict_bin).args(["--waffle"]).output().unwrap().status.success(),
        false
    );
    assert_eq!(
        Command::new(non_strict_bin).args(["--waffle"]).output().unwrap().status.success(),
        false
    );
}

#[test]
fn test_expected_args() {
    let utils = TestUtils::new();
    let expected = ["--waffle", "--pancake", "-lefse", "stroopwaffel"];
    let strict_bin = utils
        .bin("strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(true)
        .create();

    let non_strict_bin = utils
        .bin("non-strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(false)
        .create();

    assert_eq!(Command::new(strict_bin).args(expected).output().unwrap().status.success(), true);
    assert_eq!(
        Command::new(non_strict_bin).args(expected).output().unwrap().status.success(),
        true
    );
}

#[test]
fn test_args_wrong_order() {
    let utils = TestUtils::new();
    let expected = ["--waffle", "--pancake", "-lefse", "stroopwaffel"];
    let strict_bin = utils
        .bin("strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(true)
        .create();

    let non_strict_bin = utils
        .bin("non-strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(false)
        .create();

    let passed = ["--waffle", "--pancake", "stroopwaffel", "-lefse"];
    assert_eq!(Command::new(strict_bin).args(passed).output().unwrap().status.success(), false);
    assert_eq!(Command::new(non_strict_bin).args(passed).output().unwrap().status.success(), false);
}

#[test]
fn test_args_extra_args() {
    let utils = TestUtils::new();
    let expected = ["--waffle", "--pancake", "-lefse", "stroopwaffel"];
    let strict_bin = utils
        .bin("strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(true)
        .create();

    let non_strict_bin = utils
        .bin("non-strict-flapjack-analogs")
        .expected_args(&expected)
        .expected_args_strict(false)
        .create();

    let passed = ["--waffle", "--pancake", "-lefse", "stroopwaffel", "--dogs"];
    assert_eq!(Command::new(strict_bin).args(passed).output().unwrap().status.success(), false);
    assert_eq!(Command::new(non_strict_bin).args(passed).output().unwrap().status.success(), true);
}
