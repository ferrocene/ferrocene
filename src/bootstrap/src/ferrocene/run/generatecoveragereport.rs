// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::UNIX_EPOCH;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::utils::exec::BootstrapCommand;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct GenerateCoverageReport;

impl Step for GenerateCoverageReport {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Generating coverage report");

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
            dir_entry.ok().and_then(|dir_entry| {
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
                Some((file_path.to_owned(), duration))
            })
        })
        .max_by(|(_, duration_1), (_, duration_2)| duration_1.cmp(duration_2))
        .map(|(path, _)| path);

    core_tests_binary_file_path
}
