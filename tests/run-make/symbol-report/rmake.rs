use std::path::PathBuf;

/// Tests the symbol-report binary, which makes part of the coverage pipeline.
///
/// We have several testcases that run symbol-report over an input file and compare its output
/// against an expected symbol report file. Each testcase has its own directory inside
/// `tests/run-make/symbol-report/testcases`, inside this directory there must be two files:
///
/// - An `input.rs` file that will be passed to the symbol-report binary.
/// - A `expected.json` file that contains the expected symbol report, serialized as JSON. This file
/// has all its paths relative to the testcases folder.
///
/// Then this run-make test will automatically traverse each testcase directory and check that the
/// output of the symbol-report command coincides with the expected symbol report.
use run_make_support::cmd;
use run_make_support::path_helpers::{build_root, source_root};
use run_make_support::symbol_report::{SymbolReport, serde_json};

fn main() {
    let build_path = build_root();
    let rustc_path: PathBuf = std::env::var_os("RUSTC").expect("RUSTC env var is not set").into();

    let rel_rustc_path = rustc_path.strip_prefix(&build_path).unwrap().to_str().unwrap();

    let stage = if rel_rustc_path.contains("stage1") {
        1
    } else if rel_rustc_path.contains("stage2") {
        2
    } else {
        panic!("Invalid RUSTC path");
    };

    // The path to the symbol-report binary.
    let symbol_report_path = build_root().join(format!("stage{stage}-tools-bin/symbol-report"));

    // The path to the testcases folder.
    let testcases_path = source_root().join("tests/run-make/symbol-report/testcases/");

    for result in std::fs::read_dir(&testcases_path).unwrap() {
        let dir_entry = result.unwrap();
        let file_type = dir_entry.file_type().unwrap();
        // Traverse each directory inside the testcase folder.
        if file_type.is_dir() {
            // The path to the input source file for this testcase.
            let input_path = dir_entry.path().join("input.rs");
            // The path to the expected symbol report JSON file for this testcase.
            let expected_path = dir_entry.path().join("expected.json");

            // Create a new command with the symbol-report binary and set all the environment
            // variables that a regular rustc command would have.
            let mut command = cmd(&symbol_report_path);

            for (k, maybe_v) in run_make_support::rustc().into_raw_command().get_envs() {
                if let Some(v) = maybe_v {
                    command.env(k, v);
                }
            }

            // Run the command and parse its STDOUT stream as a symbol report.
            let stdout = command.arg(input_path).run().stdout_utf8();
            let mut actual_report: SymbolReport = serde_json::from_str(&stdout).unwrap();

            // Trim the testcases path from all the filenames in the symbol report so we can run
            // these tests regardless of the location of the ferrocene root.
            //
            // FIXME(@pvdrz): This should probably be a flag for the symbol-report binary instead.
            let testcases_path = testcases_path.display().to_string();
            for function in &mut actual_report.symbols {
                function.filename =
                    function.filename.strip_prefix(&testcases_path).unwrap().to_owned();
            }

            // Parse the expected file as a symbol report.
            let expected_text = std::fs::read_to_string(expected_path).unwrap();
            let expected_report: SymbolReport = serde_json::from_str(&expected_text).unwrap();

            // Assert that both reports are equal.
            assert_eq!(expected_report, actual_report);
        }
    }
}
