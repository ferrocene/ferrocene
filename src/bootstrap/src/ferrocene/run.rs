// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::Tool;
use crate::core::config::{FerroceneTraceabilityMatrixMode, TargetSelection};
use crate::ferrocene::doc::{Specification, SphinxMode, UserManual};
use crate::ferrocene::test_outcomes::TestOutcomesDir;
use crate::utils::exec::BootstrapCommand;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::UNIX_EPOCH;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct TraceabilityMatrix {
    pub(crate) target: TargetSelection,
}

impl Step for TraceabilityMatrix {
    type Output = PathBuf;
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/traceability-matrix")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TraceabilityMatrix { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let test_annotations_base =
            builder.out.join(self.target.triple).join("ferrocene").join("test-annotations");

        let specification = builder.ensure(Specification {
            mode: SphinxMode::Html,
            target: self.target,
            fresh_build: false,
        });
        let user_manual = builder.ensure(UserManual {
            mode: SphinxMode::Html,
            target: self.target,
            fresh_build: false,
        });

        let compiletest = builder.tool_exe(Tool::Compiletest);
        for (suite, mode) in &[("tests/ui", "ui"), ("tests/run-make", "run-make")] {
            builder.info(&format!("Loading test annotations from {suite}"));

            let dest = test_annotations_base.join(format!("{}.json", suite.replace('/', "-")));
            builder.run(
                Command::new(&compiletest)
                    .env("FERROCENE_COLLECT_ANNOTATIONS", "1")
                    .env("FERROCENE_DEST", dest)
                    .env("FERROCENE_SRC_BASE", builder.src.join(suite))
                    .env("FERROCENE_MODE", mode)
                    .env("FERROCENE_SUITE", suite),
            );
        }

        let html_output = builder
            .out
            .join(self.target.triple)
            .join("doc")
            .join("qualification")
            .join("traceability-matrix.html");
        builder.create_dir(html_output.parent().unwrap());

        let (spec_url, user_manual_url, src_url);
        match builder.config.ferrocene_traceability_matrix_mode {
            FerroceneTraceabilityMatrixMode::Local => {
                spec_url = format!("file://{}", specification.display());
                user_manual_url = format!("file://{}", user_manual.display());
                src_url = format!("file://{}", builder.src.display());
            }
            FerroceneTraceabilityMatrixMode::Ci => {
                spec_url = "../specification".into();
                user_manual_url = "../user-manual".into();
                src_url = format!(
                    "https://github.com/ferrocene/ferrocene/blob/{}",
                    builder.rust_sha().unwrap_or("main")
                );
            }
        }

        let mut cmd = builder.tool_cmd(Tool::FerroceneTraceabilityMatrix);
        cmd.env("TRACEABILITY_MATRIX_FLS_IDS", specification.join("paragraph-ids.json"))
            .env("TRACEABILITY_MATRIX_FLS_URL", spec_url)
            .env("TRACEABILITY_MATRIX_UM_IDS", user_manual.join("traceability-ids.json"))
            .env("TRACEABILITY_MATRIX_UM_URL", user_manual_url)
            .env("TRACEABILITY_MATRIX_ANNOTATIONS", test_annotations_base)
            .env("TRACEABILITY_MATRIX_HTML_OUT", &html_output)
            .env("TRACEABILITY_MATRIX_SRC_BASE", &builder.src)
            .env("TRACEABILITY_MATRIX_SRC_URL", src_url);

        if let Some(dir) = builder.ensure(TestOutcomesDir) {
            cmd.env("TRACEABILITY_MATRIX_TEST_OUTCOMES_DIR", dir);
        }

        builder.run(&mut cmd);
        html_output
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct GenerateCoverageReport;

impl Step for GenerateCoverageReport {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info(&"Generating coverage report");

        let coverage_report_data_dir =
            env_llvm_profile_data_dir().unwrap_or_else(|| builder.out.join("coverage"));

        let coverage_report_out_dir = builder.out.join("coverage_report");

        let coverage_src_path = builder.src.join("library/core").canonicalize().unwrap();

        if builder.config.dry_run() {
            return;
        }

        let core_tests_binary_path =
            get_test_binary_path(builder).expect("Could not find tests binary");

        if coverage_report_data_dir.exists() {
            let mut files = coverage_report_data_dir.read_dir().expect("Failed to read dir");

            let has_profraw_files = files.any(|file| {
                if let Ok(file) = file {
                    file.path().extension().map(|ext| ext.to_str().unwrap()) == Some("profraw")
                } else {
                    false
                }
            });

            if !has_profraw_files {
                panic!("No profraw files found in build/coverage directory");
            }
        }

        if coverage_report_out_dir.exists() {
            builder.remove_dir(&coverage_report_out_dir);
            builder.info(&format!("Removed previous report at {:?}", coverage_report_out_dir));
        }
        let rustc_path = builder.out.join(builder.config.build.triple).join("stage1/bin/rustc");

        let grcov_installed = Command::new("grcov").arg("--version").output().is_ok();

        if !grcov_installed {
            panic!("grcov not installed, to install it run:\n cargo install grcov");
        }

        const COVERAGE_EXCLUDE_LINE: &str = "COVR_EXCL_LINE";

        assert!(coverage_report_data_dir.exists());
        assert!(!coverage_report_out_dir.exists());
        assert!(coverage_src_path.exists());
        assert!(core_tests_binary_path.exists());

        let mut cmd = Command::new("grcov");
        cmd.arg(coverage_report_data_dir)
            .arg("-s")
            .arg(coverage_src_path)
            .arg("--ignore")
            .arg("tests/*")
            .arg("--ignore")
            .arg("benches/*")
            .arg("--binary-path")
            .arg(core_tests_binary_path)
            .arg("-t")
            .arg("html")
            .arg("--branch")
            .arg("--ignore-not-existing")
            .arg("--excl-line")
            .arg(COVERAGE_EXCLUDE_LINE)
            .arg("-o")
            .arg(coverage_report_out_dir)
            // RUSTC environment variable is required by grcov to locate the right version of the llvm-cov.
            // If the variable is not set grcov uses the the rustc in the host.
            // This version mismatch of llvm-cov leads to empty report.
            // So, we need the specific version of rustc that we used to create the profraw files.
            // grcov uses cargo-binutils to invoke llvm-cov. cargo-binutils looks for RUSTC env variable to get the path for llvm-cov.
            // https://github.com/rust-embedded/cargo-binutils/blob/5c38490e1abf91af51d0a345bb581e37facd28ff/src/rustc.rs#L8.
            .env("RUSTC", rustc_path);

        let bootstrap_cmd = BootstrapCommand::from(&mut cmd);

        builder.run_cmd(bootstrap_cmd);
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("generate-coverage-report")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(GenerateCoverageReport);
    }
}
fn env_llvm_profile_data_dir() -> Option<PathBuf> {
    // If the LLVM_PROFILE_FILE environment variable is set, get the directory
    match std::env::var("LLVM_PROFILE_FILE") {
        Ok(path) => {
            let file_path = Path::new(&path);
            let directory = file_path.parent().unwrap_or(Path::new("./"));
            Some(directory.to_owned())
        }
        Err(_err) => None,
    }
}
fn get_test_binary_path(builder: &Builder<'_>) -> Option<PathBuf> {
    let test_name = "coretests";

    let core_tests_binary_dir_path = builder
        .out
        .join(builder.config.build.triple)
        .join("stage1-std")
        .join(builder.config.build.triple)
        .join("release")
        .join("deps/");

    let core_tests_binary_file_path = core_tests_binary_dir_path
        .read_dir()
        .expect("read_dir call failed")
        .filter_map(|dir_entry| {
            if let Ok(dir_entry) = dir_entry {
                let file_path = dir_entry.path();
                let file_name = dir_entry.file_name();
                let file_name_str = file_name.to_str().expect("Failed to convert file name to str");
                if !file_name_str.starts_with(test_name) {
                    return None;
                }
                let extension = file_path.extension();
                let is_executable =
                    if let Some(extension) = extension { extension == "exe" } else { true };
                if !is_executable {
                    return None;
                }
                let metadata = file_path.metadata().expect("Failed to get metadata");
                let modified_time = metadata.modified().expect("Failed to get modified time");
                let duration = modified_time
                    .duration_since(UNIX_EPOCH)
                    .expect("Failed to get duration since epoch");
                return Some((file_path.to_owned(), duration));
            } else {
                None
            }
        })
        .max_by(|(_, duration_1), (_, duration_2)| duration_1.cmp(duration_2))
        .map(|(path, _)| path);

    core_tests_binary_file_path
}
