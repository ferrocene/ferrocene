// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use super::TestUtils;
use crate::env;

#[must_use]
pub(crate) struct BinBuilder<'a> {
    utils: &'a TestUtils,
    name: &'a str,
    mode: Option<u32>,
    stdout: Option<String>,
    stderr: Option<String>,
    exit: Option<i32>,
    expected_args: Option<&'a [&'a str]>,
    dest: BinaryDestination,
    program: &'static str,
}

impl<'a> BinBuilder<'a> {
    pub(super) fn new(utils: &'a TestUtils, name: &'a str) -> Self {
        Self {
            utils,
            name,
            mode: None,
            stdout: None,
            stderr: None,
            exit: None,
            expected_args: None,
            dest: BinaryDestination::Sysroot,
            program: BIN_PROGRAM,
        }
    }

    pub(crate) fn mode(mut self, mode: u32) -> Self {
        self.mode = Some(mode);
        self
    }

    pub(crate) fn stdout(mut self, stdout: &str) -> Self {
        self.stdout = Some(stdout.into());
        self
    }

    pub(crate) fn stderr(mut self, stderr: &str) -> Self {
        self.stderr = Some(stderr.into());
        self
    }

    pub(crate) fn exit(mut self, exit: i32) -> Self {
        self.exit = Some(exit);
        self
    }

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

        let mut rustc = rustc.spawn().unwrap();
        let stdin = rustc.stdin.as_mut().unwrap();
        stdin.write_all(self.program.as_bytes()).unwrap();
        assert!(rustc.wait().unwrap().success());

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
        let found = std::env::args().skip(1).collect::<Vec<_>>();
        assert_eq!(expected, found);
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
