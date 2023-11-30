// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::common::{Config, Mode, TestPaths};
use crate::header::EarlyProps;
use crate::{files_related_to_test, find_tests_in_dir};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use test::ColorConfig;

const BULK_ANNOTATIONS_FILE_NAME: &str = "ferrocene-annotations";

#[derive(serde::Serialize)]
struct Output<'a> {
    bulk_annotations_file_name: &'a str,
    tests: &'a [TestFile],
}

#[derive(serde::Serialize)]
struct TestFile {
    sha256: HashMap<String, String>,
    file: String,
    annotations: Vec<Annotation>,
}

#[derive(serde::Serialize, Clone, Debug)]
struct Annotation {
    annotation: String,
    file: PathBuf,
}

pub(crate) fn maybe_collect_and_exit() {
    if std::env::var("FERROCENE_COLLECT_ANNOTATIONS") != Ok("1".into()) {
        // Let compiletest continue its normal execution if we're not collecting annotations.
        return;
    }

    let dest: PathBuf = env("FERROCENE_DEST");
    let config = sample_config();

    let mut collector = Collector::new(config);
    collector.collect();
    let found = collector.write(&dest);

    println!("collected {found} tests with annotations");

    std::process::exit(0);
}

struct Collector {
    config: Config,
    tests: Vec<TestFile>,
    directory_annotations: HashMap<PathBuf, Vec<Annotation>>,
}

impl Collector {
    fn new(config: Config) -> Self {
        Self { config, tests: Vec::new(), directory_annotations: HashMap::new() }
    }

    fn collect(&mut self) {
        let src_base = self.config.src_base.clone();
        find_tests_in_dir(self.config.mode, &src_base, &PathBuf::new(), &mut |test| {
            if let Some(t) = self.collect_test(test) {
                self.tests.push(t);
            }
        })
        .unwrap();
    }

    fn collect_test(&mut self, paths: &TestPaths) -> Option<TestFile> {
        let path = if self.config.mode == Mode::RunMake {
            paths.file.join("Makefile")
        } else {
            paths.file.clone()
        };
        let contents =
            std::fs::read_to_string(&path).expect(&format!("failed to read {}", path.display()));
        let mut annotations = self.collect_annotations(&path, &contents);
        self.append_directory_annotations(paths, &mut annotations);

        if annotations.is_empty() {
            None
        } else {
            let props = EarlyProps::from_reader(&self.config, &path, contents.as_bytes());
            Some(TestFile {
                sha256: self.hash_test(paths, &props),
                file: paths.file.to_str().unwrap().into(),
                annotations,
            })
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
                    Err(err) => panic!("failed to load {}: {err}", file.display()),
                };
                self.directory_annotations.insert(parent.into(), annotations.clone());
                extend.append(&mut annotations);
            }
        }
    }

    fn collect_annotations(&self, path: &Path, contents: &str) -> Vec<Annotation> {
        let mut found = Vec::new();
        for line in contents.lines() {
            let prefix = if path.file_name() == Some(OsStr::new("Makefile")) {
                "# "
            } else if path.extension() == Some(OsStr::new("rs"))
                || path.file_name() == Some(OsStr::new(BULK_ANNOTATIONS_FILE_NAME))
            {
                "// "
            } else {
                panic!("unknown type of file encountered: {}", path.display());
            };
            let remaining = if let Some(remaining) = line.strip_prefix(prefix) {
                remaining
            } else {
                continue;
            };
            if let Some(remaining) = remaining.strip_prefix("ferrocene-annotations: ") {
                if !remaining.is_empty() {
                    found.push(Annotation { annotation: remaining.into(), file: path.into() });
                }
            } else if remaining.starts_with("ferrocene-annotation: ") {
                // Prevent common typos
                panic!(
                    "{}: attribute is called 'ferrocene-annotations', not 'ferrocene-annotation'",
                    path.display()
                );
            }
        }
        found
    }

    fn hash_test(&self, paths: &TestPaths, props: &EarlyProps) -> HashMap<String, String> {
        let mut shas = HashMap::new();
        for path in files_related_to_test(&self.config, paths, props, None) {
            if !path.is_file() {
                continue;
            }

            let mut file = BufReader::new(File::open(&path).unwrap());
            let mut sha256 = Sha256::new();
            std::io::copy(&mut file, &mut sha256).unwrap();

            shas.insert(
                path.to_str().unwrap().to_string(),
                hex::encode(sha256.finalize().as_slice()),
            );
        }

        shas
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
        src_base: env("FERROCENE_SRC_BASE"),
        mode: env("FERROCENE_MODE"),
        suite: env("FERROCENE_SUITE"),

        // We don't care about most of the configuration, as we only need to extract headers from
        // test files.
        adb_device_status: default(),
        adb_path: default(),
        adb_test_dir: default(),
        android_cross_path: default(),
        ar: default(),
        bless: default(),
        build_base: default(),
        sysroot_base: default(),
        cc: default(),
        cdb: default(),
        cdb_version: default(),
        cflags: default(),
        channel: default(),
        color: ColorConfig::NeverColor,
        compare_mode: default(),
        compile_lib_path: default(),
        cxx: default(),
        cxxflags: default(),
        debugger: default(),
        edition: default(),
        filter_exact: default(),
        filters: default(),
        force_pass_mode: default(),
        force_rerun: default(),
        force_valgrind: default(),
        gdb: default(),
        gdb_native_rust: default(),
        gdb_version: default(),
        has_tidy: default(),
        host: default(),
        host_rustcflags: default(),
        jsondocck_path: default(),
        jsondoclint_path: default(),
        linker: default(),
        lldb_native_rust: default(),
        lldb_python_dir: default(),
        lldb_version: default(),
        llvm_bin_dir: default(),
        llvm_components: default(),
        llvm_filecheck: default(),
        llvm_version: default(),
        logfile: default(),
        nodejs: default(),
        npm: default(),
        optimize_tests: default(),
        python: default(),
        format: test::OutputFormat::Terse,
        remote_test_client: default(),
        run: default(),
        run_clang_based_tests_with: default(),
        run_ignored: default(),
        run_lib_path: default(),
        runtool: default(),
        rust_demangler_path: default(),
        rustc_path: default(),
        rustdoc_path: default(),
        rustfix_coverage: default(),
        skip: default(),
        stage_id: default(),
        system_llvm: default(),
        target: default(),
        target_cfgs: default(),
        target_rustcflags: default(),
        valgrind_path: default(),
        verbose: default(),
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

fn default<T: Default>() -> T {
    T::default()
}
