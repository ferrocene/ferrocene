// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::common::{Config, Mode, TestPaths};
use crate::find_tests_in_dir;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

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
    directory_annotations: HashMap<PathBuf, Vec<Annotation>>,
}

impl Collector {
    fn new(config: Arc<Config>) -> Self {
        Self { config, tests: Vec::new(), directory_annotations: HashMap::new() }
    }

    fn collect(&mut self) {
        let src_base = self.config.src_base.clone();
        find_tests_in_dir(
            self.config.clone(),
            &src_base,
            &PathBuf::new(),
            &mut Default::default(),
            &Vec::new(),
            &mut |test| {
                if let Some(t) = self.collect_test(test) {
                    self.tests.push(t);
                }
            },
        )
        .unwrap();
    }

    fn collect_test(&mut self, paths: &TestPaths) -> Option<TestFile> {
        let path = if self.config.mode == Mode::RunMake {
            let path = paths.file.join("Makefile");
            if path.exists() { path } else { paths.file.join("rmake.rs") }
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
            Some(TestFile { file: paths.file.to_str().unwrap().into(), annotations })
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
                    found.push(Annotation { id: remaining.into(), file: path.into() });
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
        color: test::ColorConfig::NeverColor,
        format: test::OutputFormat::Json,
        mode: env("FERROCENE_MODE"),
        src_base: env("FERROCENE_SRC_BASE"),
        suite: env("FERROCENE_SUITE"),
        ..Config::default()
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
