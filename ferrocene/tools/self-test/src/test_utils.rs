// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::cell::RefCell;
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use tempfile::TempDir;

use crate::env::{self, Env};
use crate::error::Error;
use crate::report::Reporter;

pub(crate) struct TestUtils {
    env: Env,
    sysroot: TempDir,
    reports: ReportsCollector,
}

impl TestUtils {
    pub(crate) fn new() -> Self {
        let sysroot = TempDir::new().unwrap();

        Self {
            env: Env { path: Some(std::env::join_paths([&sysroot.path().join("bin")]).unwrap()) },
            sysroot,
            reports: ReportsCollector { reports: RefCell::new(Vec::new()) },
        }
    }

    pub(crate) fn sysroot(&self) -> &Path {
        self.sysroot.path()
    }

    pub(crate) fn bin<'a>(&'a self, name: &'a str) -> BinBuilder<'a> {
        BinBuilder {
            utils: self,
            name,
            mode: None,
            stdout: None,
            stderr: None,
            exit: None,
            expected_args: None,
            dest: BinaryDestinaton::Sysroot,
            program: BIN_PROGRAM,
        }
    }

    pub(crate) fn target<'a>(&'a self, name: &'a str) -> TargetBuilder<'a> {
        TargetBuilder { utils: self, name, libraries: Vec::new() }
    }

    pub(crate) fn reporter(&self) -> &dyn Reporter {
        &self.reports
    }

    pub(crate) fn env(&self) -> &Env {
        &self.env
    }

    #[track_caller]
    pub(crate) fn assert_report_success(&self, message: &str) {
        assert_eq!(
            Report::Success(message.into()),
            self.reports.reports.borrow_mut().pop().expect("no reports left")
        );
    }

    #[track_caller]
    pub(crate) fn assert_report_skipped(&self, message: &str) {
        assert_eq!(
            Report::Skipped(message.into()),
            self.reports.reports.borrow_mut().pop().expect("no reports left")
        );
    }

    #[track_caller]
    pub(crate) fn assert_no_reports(&self) {
        assert!(self.reports.reports.borrow_mut().is_empty());
    }
}

#[must_use]
pub(crate) struct BinBuilder<'a> {
    utils: &'a TestUtils,
    name: &'a str,
    mode: Option<u32>,
    stdout: Option<String>,
    stderr: Option<String>,
    exit: Option<i32>,
    expected_args: Option<&'a [&'a str]>,
    dest: BinaryDestinaton,
    program: &'static str,
}

impl<'a> BinBuilder<'a> {
    #[must_use]
    pub(crate) fn mode(mut self, mode: u32) -> Self {
        self.mode = Some(mode);
        self
    }

    #[must_use]
    pub(crate) fn stdout(mut self, stdout: &str) -> Self {
        self.stdout = Some(stdout.into());
        self
    }

    #[must_use]
    pub(crate) fn stderr(mut self, stderr: &str) -> Self {
        self.stderr = Some(stderr.into());
        self
    }

    #[must_use]
    pub(crate) fn exit(mut self, exit: i32) -> Self {
        self.exit = Some(exit);
        self
    }

    #[allow(non_snake_case)]
    #[must_use]
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

    #[must_use]
    pub(crate) fn for_target(mut self, target: &str) -> Self {
        if let BinaryDestinaton::Sysroot = self.dest {
            self.dest = BinaryDestinaton::Target(target.into());
            self
        } else {
            panic!("called for_target() when another destination was already set");
        }
    }

    #[must_use]
    pub(crate) fn expected_args(mut self, args: &'a [&'a str]) -> Self {
        self.expected_args = Some(args);
        self
    }

    #[must_use]
    pub(crate) fn program_source(mut self, source: &'static str) -> Self {
        self.program = source;
        self
    }

    pub(crate) fn create(self) -> PathBuf {
        let bin_root = match self.dest {
            BinaryDestinaton::Sysroot => self.utils.sysroot.path().join("bin"),
            BinaryDestinaton::Target(target) => {
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

enum BinaryDestinaton {
    Sysroot,
    Target(String),
}

pub(crate) struct CliVersionContent<'a> {
    pub(crate) release: &'a str,
    pub(crate) host: &'a str,
    pub(crate) commit_hash: &'a str,
}

impl Default for CliVersionContent<'_> {
    fn default() -> Self {
        Self {
            release: env::CFG_RELEASE,
            host: env::SELFTEST_TARGET,
            commit_hash: env::SELFTEST_RUST_HASH.unwrap_or("unknown"),
        }
    }
}

impl CliVersionContent<'_> {
    pub(crate) fn serialize(&self) -> String {
        format!(
            "release: {}
            host: {}
            commit-hash: {}\n",
            self.release, self.host, self.commit_hash
        )
    }
}

#[must_use]
pub(crate) struct TargetBuilder<'a> {
    utils: &'a TestUtils,
    name: &'a str,
    libraries: Vec<(&'a str, &'a str)>,
}

impl<'a> TargetBuilder<'a> {
    #[must_use]
    pub(crate) fn lib(mut self, name: &'a str, hash: &'a str) -> Self {
        self.libraries.push((name, hash));
        self
    }

    pub(crate) fn create(self) {
        let target_dir = self.utils.sysroot().join("lib").join("rustlib").join(self.name);
        std::fs::create_dir_all(&target_dir).unwrap();

        let lib_dir = target_dir.join("lib");
        std::fs::create_dir_all(&lib_dir).unwrap();

        for (name, hash) in self.libraries {
            std::fs::write(lib_dir.join(format!("lib{name}-{hash}.rlib")), b"").unwrap();
        }
    }
}

struct ReportsCollector {
    reports: RefCell<Vec<Report>>,
}

impl Reporter for ReportsCollector {
    fn success(&self, message: &str) {
        self.reports.borrow_mut().push(Report::Success(message.into()));
    }

    fn skipped(&self, message: &str) {
        self.reports.borrow_mut().push(Report::Skipped(message.into()));
    }

    fn note(&self, message: &str) {
        self.reports.borrow_mut().push(Report::Note(message.into()));
    }

    fn info(&self, message: &str) {
        self.reports.borrow_mut().push(Report::Info(message.into()));
    }

    fn error(&self, _: &Error) {
        self.reports.borrow_mut().push(Report::Error);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Report {
    Success(String),
    Skipped(String),
    Note(String),
    Info(String),
    Error,
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
