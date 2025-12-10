// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::channel;

use camino::{Utf8Path, Utf8PathBuf};

use crate::common::{Config, TestMode, TestPaths};
use crate::{TestHandler, find_tests_in_dir};

const BULK_ANNOTATIONS_FILE_NAME: &str = "ferrocene-annotations";

#[derive(serde::Serialize)]
struct Output<'a> {
    bulk_annotations_file_name: &'a str,
    tests: &'a [TestFile],
}

#[derive(serde::Serialize)]
struct TestFile {
    file: String,
    annotations: Vec<Annotation>,
}

#[derive(serde::Serialize, Clone, Debug)]
struct Annotation {
    id: String,
    file: PathBuf,
}

pub fn maybe_collect_and_exit() {
    if std::env::var("FERROCENE_COLLECT_ANNOTATIONS") != Ok("1".into()) {
        // Let compiletest continue its normal execution if we're not collecting annotations.
        return;
    }

    let dest: PathBuf = env("FERROCENE_DEST");
    let config = sample_config();

    let mut collector = Collector::new(Arc::new(config));
    collector.collect();
    let found = collector.write(&dest);

    println!("collected {found} tests with annotations");

    std::process::exit(0);
}

struct Collector {
    config: Arc<Config>,
    tests: Vec<TestFile>,
    directory_annotations: HashMap<Utf8PathBuf, Vec<Annotation>>,
}

impl Collector {
    fn new(config: Arc<Config>) -> Self {
        Self { config, tests: Vec::new(), directory_annotations: HashMap::new() }
    }

    fn collect(&mut self) {
        let src_base = self.config.src_test_suite_root.clone();
        let (tx, rx) = channel();
        find_tests_in_dir(
            self.config.clone(),
            &src_base,
            &Utf8PathBuf::new(),
            &Vec::new(),
            TestHandler::Sender(tx),
        )
        .unwrap();

        for test in rx.iter() {
            if let Some(t) = self.collect_test(&test) {
                self.tests.push(t);
            }
        }
    }

    fn collect_test(&mut self, paths: &TestPaths) -> Option<TestFile> {
        let path = if self.config.mode == TestMode::RunMake {
            paths.file.join("rmake.rs")
        } else {
            paths.file.clone()
        };
        let contents = std::fs::read_to_string(&path).expect(&format!("failed to read {path}"));
        let mut annotations = self.collect_annotations(&path, &contents);
        self.append_directory_annotations(paths, &mut annotations);

        if annotations.is_empty() {
            None
        } else {
            Some(TestFile { file: paths.file.to_string(), annotations })
        }
    }

    fn append_directory_annotations(&mut self, paths: &TestPaths, extend: &mut Vec<Annotation>) {
        if let Some(parent) = paths.file.parent() {
            if let Some(annotations) = self.directory_annotations.get(parent) {
                extend.extend_from_slice(&annotations);
            } else {
                let file = parent.join(BULK_ANNOTATIONS_FILE_NAME);
                let mut annotations = match std::fs::read_to_string(&file) {
                    Ok(contents) => self.collect_annotations(&file, &contents),
                    Err(err) if err.kind() == std::io::ErrorKind::NotFound => Vec::new(),
                    Err(err) => panic!("failed to load {file}: {err}"),
                };
                self.directory_annotations.insert(parent.into(), annotations.clone());
                extend.append(&mut annotations);
            }
        }
    }

    fn collect_annotations(&self, path: &Utf8Path, contents: &str) -> Vec<Annotation> {
        let mut found = Vec::new();
        for line in contents.lines() {
            let prefix = if path.extension() == Some("rs")
                || path.file_name() == Some(BULK_ANNOTATIONS_FILE_NAME)
            {
                "// "
            } else {
                panic!("unknown type of file encountered: {path}");
            };
            let remaining = if let Some(remaining) = line.strip_prefix(prefix) {
                remaining
            } else {
                continue;
            };
            if let Some(remaining) = remaining.strip_prefix("ferrocene-annotations: ") {
                if !remaining.is_empty() {
                    found.push(Annotation { id: remaining.into(), file: path.into() });
                }
            } else if remaining.starts_with("ferrocene-annotation: ") {
                // Prevent common typos
                panic!(
                    "{path}: attribute is called 'ferrocene-annotations', not 'ferrocene-annotation'"
                );
            }
        }
        found
    }

    fn write(self, dest: &Path) -> usize {
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let mut file = BufWriter::new(File::create(dest).unwrap());
        serde_json::to_writer(
            &mut file,
            &Output { bulk_annotations_file_name: BULK_ANNOTATIONS_FILE_NAME, tests: &self.tests },
        )
        .unwrap();
        file.write_all(b"\n").unwrap();
        file.flush().unwrap();

        self.tests.len()
    }
}

fn sample_config() -> Config {
    Config {
        color: crate::ColorConfig::NeverColor,
        mode: env("FERROCENE_MODE"),
        src_root: env("FERROCENE_SRC_ROOT"),
        src_test_suite_root: env("FERROCENE_SRC_TEST_SUITE_ROOT"),
        build_root: env("FERROCENE_BUILD_ROOT"),
        build_test_suite_root: env("FERROCENE_BUILD_TEST_SUITE_ROOT"),
        suite: env("FERROCENE_SUITE"),
        default_codegen_backend: crate::common::CodegenBackend::Llvm,

        // Dummy values
        edition: Default::default(),
        bless: Default::default(),
        fail_fast: Default::default(),
        host_compile_lib_path: Utf8PathBuf::default(),
        target_run_lib_path: Utf8PathBuf::default(),
        rustc_path: Utf8PathBuf::default(),
        cargo_path: Default::default(),
        stage0_rustc_path: Default::default(),
        rustdoc_path: Default::default(),
        coverage_dump_path: Default::default(),
        python: Default::default(),
        jsondocck_path: Default::default(),
        jsondoclint_path: Default::default(),
        llvm_filecheck: Default::default(),
        llvm_bin_dir: Default::default(),
        run_clang_based_tests_with: Default::default(),
        sysroot_base: Utf8PathBuf::default(),
        stage: Default::default(),
        stage_id: String::default(),
        debugger: Default::default(),
        run_ignored: Default::default(),
        with_rustc_debug_assertions: Default::default(),
        with_std_debug_assertions: Default::default(),
        filters: Default::default(),
        skip: Default::default(),
        filter_exact: Default::default(),
        force_pass_mode: Default::default(),
        run: Default::default(),
        runner: Default::default(),
        host_rustcflags: Default::default(),
        target_rustcflags: Default::default(),
        rust_randomized_layout: Default::default(),
        optimize_tests: Default::default(),
        target: Default::default(),
        host: Default::default(),
        cdb: Default::default(),
        cdb_version: Default::default(),
        gdb: Default::default(),
        gdb_version: Default::default(),
        lldb_version: Default::default(),
        llvm_version: Default::default(),
        system_llvm: Default::default(),
        android_cross_path: Default::default(),
        adb_path: Default::default(),
        adb_test_dir: Default::default(),
        adb_device_status: Default::default(),
        lldb: Default::default(),
        verbose: Default::default(),
        remote_test_client: Default::default(),
        compare_mode: Default::default(),
        rustfix_coverage: Default::default(),
        has_html_tidy: Default::default(),
        has_enzyme: Default::default(),
        channel: Default::default(),
        git_hash: Default::default(),
        cc: Default::default(),
        cxx: Default::default(),
        cflags: Default::default(),
        cxxflags: Default::default(),
        ar: Default::default(),
        target_linker: Default::default(),
        host_linker: Default::default(),
        llvm_components: Default::default(),
        nodejs: Default::default(),
        force_rerun: Default::default(),
        only_modified: Default::default(),
        target_cfgs: Default::default(),
        builtin_cfg_names: Default::default(),
        supported_crate_types: Default::default(),
        capture: Default::default(),
        nightly_branch: Default::default(),
        git_merge_commit_email: Default::default(),
        profiler_runtime: Default::default(),
        diff_command: Default::default(),
        minicore_path: Default::default(),
        query_rustc_path: Default::default(),
        bypass_ignore_backends: Default::default(),
        override_codegen_backend: Default::default(),
    }
}

fn env<T>(var: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    if let Ok(contents) = std::env::var(var) {
        contents.parse().expect(&format!("failed to parse {var}"))
    } else {
        panic!("missing variable {var}");
    }
}
