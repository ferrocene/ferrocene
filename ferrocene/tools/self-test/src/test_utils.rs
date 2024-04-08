// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod bin_builder;

use std::cell::RefCell;
use std::path::Path;

use tempfile::TempDir;

use self::bin_builder::BinBuilder;
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
            reports: ReportsCollector::new(),
        }
    }

    pub(crate) fn sysroot(&self) -> &Path {
        self.sysroot.path()
    }

    pub(crate) fn bin<'a>(&'a self, name: &'a str) -> BinBuilder<'a> {
        BinBuilder::new(self, name)
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

impl ReportsCollector {
    fn new() -> Self {
        Self { reports: RefCell::new(Vec::new()) }
    }
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
