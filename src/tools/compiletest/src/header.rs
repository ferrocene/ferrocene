use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use regex::Regex;
use tracing::*;

use crate::common::{Config, Debugger, FailMode, Mode, PassMode};
use crate::header::cfg::{parse_cfg_name_directive, MatchOutcome};
use crate::header::needs::CachedNeedsConditions;
use crate::util::static_regex;
use crate::{extract_cdb_version, extract_gdb_version};

mod cfg;
mod needs;
#[cfg(test)]
mod tests;

pub struct HeadersCache {
    needs: CachedNeedsConditions,
}

impl HeadersCache {
    pub fn load(config: &Config) -> Self {
        Self { needs: CachedNeedsConditions::load(config) }
    }
}

/// Properties which must be known very early, before actually running
/// the test.
#[derive(Default)]
pub struct EarlyProps {
    pub aux: Vec<String>,
    pub aux_bin: Vec<String>,
    pub aux_crate: Vec<(String, String)>,
    pub revisions: Vec<String>,
}

impl EarlyProps {
    pub fn from_file(config: &Config, testfile: &Path) -> Self {
        let file = File::open(testfile).expect("open test file to parse earlyprops");
        Self::from_reader(config, testfile, file)
    }

    pub fn from_reader<R: Read>(config: &Config, testfile: &Path, rdr: R) -> Self {
        let mut props = EarlyProps::default();
        let mut poisoned = false;
        iter_header(
            config.mode,
            &config.suite,
            &mut poisoned,
            testfile,
            rdr,
            &mut |HeaderLine { directive: ln, .. }| {
                config.push_name_value_directive(ln, directives::AUX_BUILD, &mut props.aux, |r| {
                    r.trim().to_string()
                });
                config.push_name_value_directive(
                    ln,
                    directives::AUX_BIN,
                    &mut props.aux_bin,
                    |r| r.trim().to_string(),
                );
                config.push_name_value_directive(
                    ln,
                    directives::AUX_CRATE,
                    &mut props.aux_crate,
                    Config::parse_aux_crate,
                );
                config.parse_and_update_revisions(ln, &mut props.revisions);
            },
        );

        if poisoned {
            eprintln!("errors encountered during EarlyProps parsing: {}", testfile.display());
            panic!("errors encountered during EarlyProps parsing");
        }

        props
    }
}

#[derive(Clone, Debug)]
pub struct TestProps {
    // Lines that should be expected, in order, on standard out
    pub error_patterns: Vec<String>,
    // Regexes that should be expected, in order, on standard out
    pub regex_error_patterns: Vec<String>,
    // Extra flags to pass to the compiler
    pub compile_flags: Vec<String>,
    // Extra flags to pass when the compiled code is run (such as --bench)
    pub run_flags: Vec<String>,
    /// Extra flags to pass to rustdoc but not the compiler.
    pub doc_flags: Vec<String>,
    // If present, the name of a file that this test should match when
    // pretty-printed
    pub pp_exact: Option<PathBuf>,
    // Other crates that should be compiled (typically from the same
    // directory as the test, but for backwards compatibility reasons
    // we also check the auxiliary directory)
    pub aux_builds: Vec<String>,
    // Auxiliary crates that should be compiled as `#![crate_type = "bin"]`.
    pub aux_bins: Vec<String>,
    // Similar to `aux_builds`, but a list of NAME=somelib.rs of dependencies
    // to build and pass with the `--extern` flag.
    pub aux_crates: Vec<(String, String)>,
    /// Similar to `aux_builds`, but also passes the resulting dylib path to
    /// `-Zcodegen-backend`.
    pub aux_codegen_backend: Option<String>,
    // Environment settings to use for compiling
    pub rustc_env: Vec<(String, String)>,
    // Environment variables to unset prior to compiling.
    // Variables are unset before applying 'rustc_env'.
    pub unset_rustc_env: Vec<String>,
    // Environment settings to use during execution
    pub exec_env: Vec<(String, String)>,
    // Environment variables to unset prior to execution.
    // Variables are unset before applying 'exec_env'
    pub unset_exec_env: Vec<String>,
    // Build documentation for all specified aux-builds as well
    pub build_aux_docs: bool,
    /// Build the documentation for each crate in a unique output directory.
    /// Uses <root output directory>/docs/<test name>/doc
    pub unique_doc_out_dir: bool,
    // Flag to force a crate to be built with the host architecture
    pub force_host: bool,
    // Check stdout for error-pattern output as well as stderr
    pub check_stdout: bool,
    // Check stdout & stderr for output of run-pass test
    pub check_run_results: bool,
    // For UI tests, allows compiler to generate arbitrary output to stdout
    pub dont_check_compiler_stdout: bool,
    // For UI tests, allows compiler to generate arbitrary output to stderr
    pub dont_check_compiler_stderr: bool,
    // When checking the output of stdout or stderr check
    // that the lines of expected output are a subset of the actual output.
    pub compare_output_lines_by_subset: bool,
    // Don't force a --crate-type=dylib flag on the command line
    //
    // Set this for example if you have an auxiliary test file that contains
    // a proc-macro and needs `#![crate_type = "proc-macro"]`. This ensures
    // that the aux file is compiled as a `proc-macro` and not as a `dylib`.
    pub no_prefer_dynamic: bool,
    // Run -Zunpretty expanded when running pretty printing tests
    pub pretty_expanded: bool,
    // Which pretty mode are we testing with, default to 'normal'
    pub pretty_mode: String,
    // Only compare pretty output and don't try compiling
    pub pretty_compare_only: bool,
    // Patterns which must not appear in the output of a cfail test.
    pub forbid_output: Vec<String>,
    // Revisions to test for incremental compilation.
    pub revisions: Vec<String>,
    // Directory (if any) to use for incremental compilation.  This is
    // not set by end-users; rather it is set by the incremental
    // testing harness and used when generating compilation
    // arguments. (In particular, it propagates to the aux-builds.)
    pub incremental_dir: Option<PathBuf>,
    // If `true`, this test will use incremental compilation.
    //
    // This can be set manually with the `incremental` header, or implicitly
    // by being a part of an incremental mode test. Using the `incremental`
    // header should be avoided if possible; using an incremental mode test is
    // preferred. Incremental mode tests support multiple passes, which can
    // verify that the incremental cache can be loaded properly after being
    // created. Just setting the header will only verify the behavior with
    // creating an incremental cache, but doesn't check that it is created
    // correctly.
    //
    // Compiletest will create the incremental directory, and ensure it is
    // empty before the test starts. Incremental mode tests will reuse the
    // incremental directory between passes in the same test.
    pub incremental: bool,
    // If `true`, this test is a known bug.
    //
    // When set, some requirements are relaxed. Currently, this only means no
    // error annotations are needed, but this may be updated in the future to
    // include other relaxations.
    pub known_bug: bool,
    // How far should the test proceed while still passing.
    pass_mode: Option<PassMode>,
    // Ignore `--pass` overrides from the command line for this test.
    ignore_pass: bool,
    // How far this test should proceed to start failing.
    pub fail_mode: Option<FailMode>,
    // rustdoc will test the output of the `--test` option
    pub check_test_line_numbers_match: bool,
    // customized normalization rules
    pub normalize_stdout: Vec<(String, String)>,
    pub normalize_stderr: Vec<(String, String)>,
    pub failure_status: Option<i32>,
    // For UI tests, allows compiler to exit with arbitrary failure status
    pub dont_check_failure_status: bool,
    // Whether or not `rustfix` should apply the `CodeSuggestion`s of this test and compile the
    // resulting Rust code.
    pub run_rustfix: bool,
    // If true, `rustfix` will only apply `MachineApplicable` suggestions.
    pub rustfix_only_machine_applicable: bool,
    pub assembly_output: Option<String>,
    // If true, the test is expected to ICE
    pub should_ice: bool,
    // If true, the stderr is expected to be different across bit-widths.
    pub stderr_per_bitwidth: bool,
    // The MIR opt to unit test, if any
    pub mir_unit_test: Option<String>,
    // Whether to tell `rustc` to remap the "src base" directory to a fake
    // directory.
    pub remap_src_base: bool,
    /// Extra flags to pass to `llvm-cov` when producing coverage reports.
    /// Only used by the "coverage-run" test mode.
    pub llvm_cov_flags: Vec<String>,
    /// Extra flags to pass to LLVM's `filecheck` tool, in tests that use it.
    pub filecheck_flags: Vec<String>,
    /// Don't automatically insert any `--check-cfg` args
    pub no_auto_check_cfg: bool,
}

mod directives {
    pub const ERROR_PATTERN: &'static str = "error-pattern";
    pub const REGEX_ERROR_PATTERN: &'static str = "regex-error-pattern";
    pub const COMPILE_FLAGS: &'static str = "compile-flags";
    pub const RUN_FLAGS: &'static str = "run-flags";
    pub const DOC_FLAGS: &'static str = "doc-flags";
    pub const SHOULD_ICE: &'static str = "should-ice";
    pub const BUILD_AUX_DOCS: &'static str = "build-aux-docs";
    pub const UNIQUE_DOC_OUT_DIR: &'static str = "unique-doc-out-dir";
    pub const FORCE_HOST: &'static str = "force-host";
    pub const CHECK_STDOUT: &'static str = "check-stdout";
    pub const CHECK_RUN_RESULTS: &'static str = "check-run-results";
    pub const DONT_CHECK_COMPILER_STDOUT: &'static str = "dont-check-compiler-stdout";
    pub const DONT_CHECK_COMPILER_STDERR: &'static str = "dont-check-compiler-stderr";
    pub const NO_PREFER_DYNAMIC: &'static str = "no-prefer-dynamic";
    pub const PRETTY_EXPANDED: &'static str = "pretty-expanded";
    pub const PRETTY_MODE: &'static str = "pretty-mode";
    pub const PRETTY_COMPARE_ONLY: &'static str = "pretty-compare-only";
    pub const AUX_BIN: &'static str = "aux-bin";
    pub const AUX_BUILD: &'static str = "aux-build";
    pub const AUX_CRATE: &'static str = "aux-crate";
    pub const AUX_CODEGEN_BACKEND: &'static str = "aux-codegen-backend";
    pub const EXEC_ENV: &'static str = "exec-env";
    pub const RUSTC_ENV: &'static str = "rustc-env";
    pub const UNSET_EXEC_ENV: &'static str = "unset-exec-env";
    pub const UNSET_RUSTC_ENV: &'static str = "unset-rustc-env";
    pub const FORBID_OUTPUT: &'static str = "forbid-output";
    pub const CHECK_TEST_LINE_NUMBERS_MATCH: &'static str = "check-test-line-numbers-match";
    pub const IGNORE_PASS: &'static str = "ignore-pass";
    pub const FAILURE_STATUS: &'static str = "failure-status";
    pub const DONT_CHECK_FAILURE_STATUS: &'static str = "dont-check-failure-status";
    pub const RUN_RUSTFIX: &'static str = "run-rustfix";
    pub const RUSTFIX_ONLY_MACHINE_APPLICABLE: &'static str = "rustfix-only-machine-applicable";
    pub const ASSEMBLY_OUTPUT: &'static str = "assembly-output";
    pub const STDERR_PER_BITWIDTH: &'static str = "stderr-per-bitwidth";
    pub const INCREMENTAL: &'static str = "incremental";
    pub const KNOWN_BUG: &'static str = "known-bug";
    pub const TEST_MIR_PASS: &'static str = "test-mir-pass";
    pub const REMAP_SRC_BASE: &'static str = "remap-src-base";
    pub const COMPARE_OUTPUT_LINES_BY_SUBSET: &'static str = "compare-output-lines-by-subset";
    pub const LLVM_COV_FLAGS: &'static str = "llvm-cov-flags";
    pub const FILECHECK_FLAGS: &'static str = "filecheck-flags";
    pub const NO_AUTO_CHECK_CFG: &'static str = "no-auto-check-cfg";
    // This isn't a real directive, just one that is probably mistyped often
    pub const INCORRECT_COMPILER_FLAGS: &'static str = "compiler-flags";
}

impl TestProps {
    pub fn new() -> Self {
        TestProps {
            error_patterns: vec![],
            regex_error_patterns: vec![],
            compile_flags: vec![],
            run_flags: vec![],
            doc_flags: vec![],
            pp_exact: None,
            aux_builds: vec![],
            aux_bins: vec![],
            aux_crates: vec![],
            aux_codegen_backend: None,
            revisions: vec![],
            rustc_env: vec![
                ("RUSTC_ICE".to_string(), "0".to_string()),
                ("RUST_BACKTRACE".to_string(), "short".to_string()),
            ],
            unset_rustc_env: vec![("RUSTC_LOG_COLOR".to_string())],
            exec_env: vec![],
            unset_exec_env: vec![],
            build_aux_docs: false,
            unique_doc_out_dir: false,
            force_host: false,
            check_stdout: false,
            check_run_results: false,
            dont_check_compiler_stdout: false,
            dont_check_compiler_stderr: false,
            compare_output_lines_by_subset: false,
            no_prefer_dynamic: false,
            pretty_expanded: false,
            pretty_mode: "normal".to_string(),
            pretty_compare_only: false,
            forbid_output: vec![],
            incremental_dir: None,
            incremental: false,
            known_bug: false,
            pass_mode: None,
            fail_mode: None,
            ignore_pass: false,
            check_test_line_numbers_match: false,
            normalize_stdout: vec![],
            normalize_stderr: vec![],
            failure_status: None,
            dont_check_failure_status: false,
            run_rustfix: false,
            rustfix_only_machine_applicable: false,
            assembly_output: None,
            should_ice: false,
            stderr_per_bitwidth: false,
            mir_unit_test: None,
            remap_src_base: false,
            llvm_cov_flags: vec![],
            filecheck_flags: vec![],
            no_auto_check_cfg: false,
        }
    }

    pub fn from_aux_file(&self, testfile: &Path, revision: Option<&str>, config: &Config) -> Self {
        let mut props = TestProps::new();

        // copy over select properties to the aux build:
        props.incremental_dir = self.incremental_dir.clone();
        props.ignore_pass = true;
        props.load_from(testfile, revision, config);

        props
    }

    pub fn from_file(testfile: &Path, revision: Option<&str>, config: &Config) -> Self {
        let mut props = TestProps::new();
        props.load_from(testfile, revision, config);
        props.exec_env.push(("RUSTC".to_string(), config.rustc_path.display().to_string()));

        match (props.pass_mode, props.fail_mode) {
            (None, None) if config.mode == Mode::Ui => props.fail_mode = Some(FailMode::Check),
            (Some(_), Some(_)) => panic!("cannot use a *-fail and *-pass mode together"),
            _ => {}
        }

        props
    }

    /// Loads properties from `testfile` into `props`. If a property is
    /// tied to a particular revision `foo` (indicated by writing
    /// `//@[foo]`), then the property is ignored unless `test_revision` is
    /// `Some("foo")`.
    fn load_from(&mut self, testfile: &Path, test_revision: Option<&str>, config: &Config) {
        let mut has_edition = false;
        if !testfile.is_dir() {
            let file = File::open(testfile).unwrap();

            let mut poisoned = false;

            iter_header(
                config.mode,
                &config.suite,
                &mut poisoned,
                testfile,
                file,
                &mut |HeaderLine { header_revision, directive: ln, .. }| {
                    if header_revision.is_some() && header_revision != test_revision {
                        return;
                    }

                    use directives::*;

                    config.push_name_value_directive(
                        ln,
                        ERROR_PATTERN,
                        &mut self.error_patterns,
                        |r| r,
                    );
                    config.push_name_value_directive(
                        ln,
                        REGEX_ERROR_PATTERN,
                        &mut self.regex_error_patterns,
                        |r| r,
                    );

                    config.push_name_value_directive(ln, DOC_FLAGS, &mut self.doc_flags, |r| r);

                    fn split_flags(flags: &str) -> Vec<String> {
                        // Individual flags can be single-quoted to preserve spaces; see
                        // <https://github.com/rust-lang/rust/pull/115948/commits/957c5db6>.
                        flags
                            .split('\'')
                            .enumerate()
                            .flat_map(|(i, f)| {
                                if i % 2 == 1 { vec![f] } else { f.split_whitespace().collect() }
                            })
                            .map(move |s| s.to_owned())
                            .collect::<Vec<_>>()
                    }

                    if let Some(flags) = config.parse_name_value_directive(ln, COMPILE_FLAGS) {
                        self.compile_flags.extend(split_flags(&flags));
                    }
                    if config.parse_name_value_directive(ln, INCORRECT_COMPILER_FLAGS).is_some() {
                        panic!("`compiler-flags` directive should be spelled `compile-flags`");
                    }

                    if let Some(edition) = config.parse_edition(ln) {
                        self.compile_flags.push(format!("--edition={}", edition.trim()));
                        has_edition = true;
                    }

                    config.parse_and_update_revisions(ln, &mut self.revisions);

                    if let Some(flags) = config.parse_name_value_directive(ln, RUN_FLAGS) {
                        self.run_flags.extend(split_flags(&flags));
                    }

                    if self.pp_exact.is_none() {
                        self.pp_exact = config.parse_pp_exact(ln, testfile);
                    }

                    config.set_name_directive(ln, SHOULD_ICE, &mut self.should_ice);
                    config.set_name_directive(ln, BUILD_AUX_DOCS, &mut self.build_aux_docs);
                    config.set_name_directive(ln, UNIQUE_DOC_OUT_DIR, &mut self.unique_doc_out_dir);

                    config.set_name_directive(ln, FORCE_HOST, &mut self.force_host);
                    config.set_name_directive(ln, CHECK_STDOUT, &mut self.check_stdout);
                    config.set_name_directive(ln, CHECK_RUN_RESULTS, &mut self.check_run_results);
                    config.set_name_directive(
                        ln,
                        DONT_CHECK_COMPILER_STDOUT,
                        &mut self.dont_check_compiler_stdout,
                    );
                    config.set_name_directive(
                        ln,
                        DONT_CHECK_COMPILER_STDERR,
                        &mut self.dont_check_compiler_stderr,
                    );
                    config.set_name_directive(ln, NO_PREFER_DYNAMIC, &mut self.no_prefer_dynamic);
                    config.set_name_directive(ln, PRETTY_EXPANDED, &mut self.pretty_expanded);

                    if let Some(m) = config.parse_name_value_directive(ln, PRETTY_MODE) {
                        self.pretty_mode = m;
                    }

                    config.set_name_directive(
                        ln,
                        PRETTY_COMPARE_ONLY,
                        &mut self.pretty_compare_only,
                    );
                    config.push_name_value_directive(ln, AUX_BUILD, &mut self.aux_builds, |r| {
                        r.trim().to_string()
                    });
                    config.push_name_value_directive(ln, AUX_BIN, &mut self.aux_bins, |r| {
                        r.trim().to_string()
                    });
                    config.push_name_value_directive(
                        ln,
                        AUX_CRATE,
                        &mut self.aux_crates,
                        Config::parse_aux_crate,
                    );
                    if let Some(r) = config.parse_name_value_directive(ln, AUX_CODEGEN_BACKEND) {
                        self.aux_codegen_backend = Some(r.trim().to_owned());
                    }
                    config.push_name_value_directive(
                        ln,
                        EXEC_ENV,
                        &mut self.exec_env,
                        Config::parse_env,
                    );
                    config.push_name_value_directive(
                        ln,
                        UNSET_EXEC_ENV,
                        &mut self.unset_exec_env,
                        |r| r,
                    );
                    config.push_name_value_directive(
                        ln,
                        RUSTC_ENV,
                        &mut self.rustc_env,
                        Config::parse_env,
                    );
                    config.push_name_value_directive(
                        ln,
                        UNSET_RUSTC_ENV,
                        &mut self.unset_rustc_env,
                        |r| r,
                    );
                    config.push_name_value_directive(
                        ln,
                        FORBID_OUTPUT,
                        &mut self.forbid_output,
                        |r| r,
                    );
                    config.set_name_directive(
                        ln,
                        CHECK_TEST_LINE_NUMBERS_MATCH,
                        &mut self.check_test_line_numbers_match,
                    );

                    self.update_pass_mode(ln, test_revision, config);
                    self.update_fail_mode(ln, config);

                    config.set_name_directive(ln, IGNORE_PASS, &mut self.ignore_pass);

                    if let Some(rule) = config.parse_custom_normalization(ln, "normalize-stdout") {
                        self.normalize_stdout.push(rule);
                    }
                    if let Some(rule) = config.parse_custom_normalization(ln, "normalize-stderr") {
                        self.normalize_stderr.push(rule);
                    }

                    if let Some(code) = config
                        .parse_name_value_directive(ln, FAILURE_STATUS)
                        .and_then(|code| code.trim().parse::<i32>().ok())
                    {
                        self.failure_status = Some(code);
                    }

                    config.set_name_directive(
                        ln,
                        DONT_CHECK_FAILURE_STATUS,
                        &mut self.dont_check_failure_status,
                    );

                    config.set_name_directive(ln, RUN_RUSTFIX, &mut self.run_rustfix);
                    config.set_name_directive(
                        ln,
                        RUSTFIX_ONLY_MACHINE_APPLICABLE,
                        &mut self.rustfix_only_machine_applicable,
                    );
                    config.set_name_value_directive(
                        ln,
                        ASSEMBLY_OUTPUT,
                        &mut self.assembly_output,
                        |r| r.trim().to_string(),
                    );
                    config.set_name_directive(
                        ln,
                        STDERR_PER_BITWIDTH,
                        &mut self.stderr_per_bitwidth,
                    );
                    config.set_name_directive(ln, INCREMENTAL, &mut self.incremental);

                    // Unlike the other `name_value_directive`s this needs to be handled manually,
                    // because it sets a `bool` flag.
                    if let Some(known_bug) = config.parse_name_value_directive(ln, KNOWN_BUG) {
                        let known_bug = known_bug.trim();
                        if known_bug == "unknown"
                            || known_bug.split(',').all(|issue_ref| {
                                issue_ref
                                    .trim()
                                    .split_once('#')
                                    .filter(|(_, number)| {
                                        number.chars().all(|digit| digit.is_numeric())
                                    })
                                    .is_some()
                            })
                        {
                            self.known_bug = true;
                        } else {
                            panic!(
                                "Invalid known-bug value: {known_bug}\nIt requires comma-separated issue references (`#000` or `chalk#000`) or `known-bug: unknown`."
                            );
                        }
                    } else if config.parse_name_directive(ln, KNOWN_BUG) {
                        panic!(
                            "Invalid known-bug attribute, requires comma-separated issue references (`#000` or `chalk#000`) or `known-bug: unknown`."
                        );
                    }

                    config.set_name_value_directive(
                        ln,
                        TEST_MIR_PASS,
                        &mut self.mir_unit_test,
                        |s| s.trim().to_string(),
                    );
                    config.set_name_directive(ln, REMAP_SRC_BASE, &mut self.remap_src_base);
                    config.set_name_directive(
                        ln,
                        COMPARE_OUTPUT_LINES_BY_SUBSET,
                        &mut self.compare_output_lines_by_subset,
                    );

                    if let Some(flags) = config.parse_name_value_directive(ln, LLVM_COV_FLAGS) {
                        self.llvm_cov_flags.extend(split_flags(&flags));
                    }

                    if let Some(flags) = config.parse_name_value_directive(ln, FILECHECK_FLAGS) {
                        self.filecheck_flags.extend(split_flags(&flags));
                    }

                    config.set_name_directive(ln, NO_AUTO_CHECK_CFG, &mut self.no_auto_check_cfg);
                },
            );

            if poisoned {
                eprintln!("errors encountered during TestProps parsing: {}", testfile.display());
                panic!("errors encountered during TestProps parsing");
            }
        }

        if self.should_ice {
            self.failure_status = Some(101);
        }

        if config.mode == Mode::Incremental {
            self.incremental = true;
        }

        if config.mode == Mode::Crashes {
            // we don't want to pollute anything with backtrace-files
            // also turn off backtraces in order to save some execution
            // time on the tests; we only need to know IF it crashes
            self.rustc_env = vec![
                ("RUST_BACKTRACE".to_string(), "0".to_string()),
                ("RUSTC_ICE".to_string(), "0".to_string()),
            ];
        }

        for key in &["RUST_TEST_NOCAPTURE", "RUST_TEST_THREADS"] {
            if let Ok(val) = env::var(key) {
                if !self.exec_env.iter().any(|&(ref x, _)| x == key) {
                    self.exec_env.push(((*key).to_owned(), val))
                }
            }
        }

        if let (Some(edition), false) = (&config.edition, has_edition) {
            self.compile_flags.push(format!("--edition={}", edition));
        }
    }

    fn update_fail_mode(&mut self, ln: &str, config: &Config) {
        let check_ui = |mode: &str| {
            // Mode::Crashes may need build-fail in order to trigger llvm errors or stack overflows
            if config.mode != Mode::Ui && config.mode != Mode::Crashes {
                panic!("`{}-fail` header is only supported in UI tests", mode);
            }
        };
        if config.mode == Mode::Ui && config.parse_name_directive(ln, "compile-fail") {
            panic!("`compile-fail` header is useless in UI tests");
        }
        let fail_mode = if config.parse_name_directive(ln, "check-fail") {
            check_ui("check");
            Some(FailMode::Check)
        } else if config.parse_name_directive(ln, "build-fail") {
            check_ui("build");
            Some(FailMode::Build)
        } else if config.parse_name_directive(ln, "run-fail") {
            check_ui("run");
            Some(FailMode::Run)
        } else {
            None
        };
        match (self.fail_mode, fail_mode) {
            (None, Some(_)) => self.fail_mode = fail_mode,
            (Some(_), Some(_)) => panic!("multiple `*-fail` headers in a single test"),
            (_, None) => {}
        }
    }

    fn update_pass_mode(&mut self, ln: &str, revision: Option<&str>, config: &Config) {
        let check_no_run = |s| match (config.mode, s) {
            (Mode::Ui, _) => (),
            (Mode::Crashes, _) => (),
            (Mode::Codegen, "build-pass") => (),
            (Mode::Incremental, _) => {
                if revision.is_some() && !self.revisions.iter().all(|r| r.starts_with("cfail")) {
                    panic!("`{s}` header is only supported in `cfail` incremental tests")
                }
            }
            (mode, _) => panic!("`{s}` header is not supported in `{mode}` tests"),
        };
        let pass_mode = if config.parse_name_directive(ln, "check-pass") {
            check_no_run("check-pass");
            Some(PassMode::Check)
        } else if config.parse_name_directive(ln, "build-pass") {
            check_no_run("build-pass");
            Some(PassMode::Build)
        } else if config.parse_name_directive(ln, "run-pass") {
            check_no_run("run-pass");
            Some(PassMode::Run)
        } else {
            None
        };
        match (self.pass_mode, pass_mode) {
            (None, Some(_)) => self.pass_mode = pass_mode,
            (Some(_), Some(_)) => panic!("multiple `*-pass` headers in a single test"),
            (_, None) => {}
        }
    }

    pub fn pass_mode(&self, config: &Config) -> Option<PassMode> {
        if !self.ignore_pass && self.fail_mode.is_none() {
            if let mode @ Some(_) = config.force_pass_mode {
                return mode;
            }
        }
        self.pass_mode
    }

    // does not consider CLI override for pass mode
    pub fn local_pass_mode(&self) -> Option<PassMode> {
        self.pass_mode
    }
}

/// Extract an `(Option<line_revision>, directive)` directive from a line if comment is present.
///
/// See [`HeaderLine`] for a diagram.
pub fn line_directive<'line>(
    comment: &str,
    original_line: &'line str,
) -> Option<(Option<&'line str>, &'line str)> {
    // Ignore lines that don't start with the comment prefix.
    let after_comment = original_line.trim_start().strip_prefix(comment)?.trim_start();

    if let Some(after_open_bracket) = after_comment.strip_prefix('[') {
        // A comment like `//@[foo]` only applies to revision `foo`.
        let Some((line_revision, directive)) = after_open_bracket.split_once(']') else {
            panic!(
                "malformed condition directive: expected `{comment}[foo]`, found `{original_line}`"
            )
        };

        Some((Some(line_revision), directive.trim_start()))
    } else {
        Some((None, after_comment))
    }
}

// To prevent duplicating the list of commmands between `compiletest`,`htmldocck` and `jsondocck`,
// we put it into a common file which is included in rust code and parsed here.
// FIXME: This setup is temporary until we figure out how to improve this situation.
//        See <https://github.com/rust-lang/rust/issues/125813#issuecomment-2141953780>.
include!("command-list.rs");

const KNOWN_HTMLDOCCK_DIRECTIVE_NAMES: &[&str] = &[
    "count",
    "!count",
    "files",
    "!files",
    "has",
    "!has",
    "has-dir",
    "!has-dir",
    "hasraw",
    "!hasraw",
    "matches",
    "!matches",
    "matchesraw",
    "!matchesraw",
    "snapshot",
    "!snapshot",
];

const KNOWN_JSONDOCCK_DIRECTIVE_NAMES: &[&str] =
    &["count", "!count", "has", "!has", "is", "!is", "ismany", "!ismany", "set", "!set"];

/// The broken-down contents of a line containing a test header directive,
/// which [`iter_header`] passes to its callback function.
///
/// For example:
///
/// ```text
/// //@ compile-flags: -O
///     ^^^^^^^^^^^^^^^^^ directive
/// ^^^^^^^^^^^^^^^^^^^^^ original_line
///
/// //@ [foo] compile-flags: -O
///      ^^^                    header_revision
///           ^^^^^^^^^^^^^^^^^ directive
/// ^^^^^^^^^^^^^^^^^^^^^^^^^^^ original_line
/// ```
struct HeaderLine<'ln> {
    line_number: usize,
    /// Raw line from the test file, including comment prefix and any revision.
    original_line: &'ln str,
    /// Some header directives start with a revision name in square brackets
    /// (e.g. `[foo]`), and only apply to that revision of the test.
    /// If present, this field contains the revision name (e.g. `foo`).
    header_revision: Option<&'ln str>,
    /// The main part of the header directive, after removing the comment prefix
    /// and the optional revision specifier.
    directive: &'ln str,
}

pub(crate) struct CheckDirectiveResult<'ln> {
    is_known_directive: bool,
    directive_name: &'ln str,
    trailing_directive: Option<&'ln str>,
}

pub(crate) fn check_directive<'a>(
    directive_ln: &'a str,
    mode: Mode,
    original_line: &str,
) -> CheckDirectiveResult<'a> {
    let (directive_name, post) = directive_ln.split_once([':', ' ']).unwrap_or((directive_ln, ""));

    let trailing = post.trim().split_once(' ').map(|(pre, _)| pre).unwrap_or(post);
    let is_known = |s: &str| {
        KNOWN_DIRECTIVE_NAMES.contains(&s)
            || match mode {
                Mode::Rustdoc | Mode::RustdocJson => {
                    original_line.starts_with("//@")
                        && match mode {
                            Mode::Rustdoc => KNOWN_HTMLDOCCK_DIRECTIVE_NAMES,
                            Mode::RustdocJson => KNOWN_JSONDOCCK_DIRECTIVE_NAMES,
                            _ => unreachable!(),
                        }
                        .contains(&s)
                }
                _ => false,
            }
    };
    let trailing_directive = {
        // 1. is the directive name followed by a space? (to exclude `:`)
        matches!(directive_ln.get(directive_name.len()..), Some(s) if s.starts_with(' '))
            // 2. is what is after that directive also a directive (ex: "only-x86 only-arm")
            && is_known(trailing)
    }
    .then_some(trailing);

    CheckDirectiveResult {
        is_known_directive: is_known(&directive_name),
        directive_name: directive_ln,
        trailing_directive,
    }
}

fn iter_header(
    mode: Mode,
    _suite: &str,
    poisoned: &mut bool,
    testfile: &Path,
    rdr: impl Read,
    it: &mut dyn FnMut(HeaderLine<'_>),
) {
    if testfile.is_dir() {
        return;
    }

    // Coverage tests in coverage-run mode always have these extra directives,
    // without needing to specify them manually in every test file.
    // (Some of the comments below have been copied over from the old
    // `tests/run-make/coverage-reports/Makefile`, which no longer exists.)
    if mode == Mode::CoverageRun {
        let extra_directives: &[&str] = &[
            "needs-profiler-support",
            // FIXME(pietroalbini): this test currently does not work on cross-compiled
            // targets because remote-test is not capable of sending back the *.profraw
            // files generated by the LLVM instrumentation.
            "ignore-cross-compile",
        ];
        // Process the extra implied directives, with a dummy line number of 0.
        for directive in extra_directives {
            it(HeaderLine { line_number: 0, original_line: "", header_revision: None, directive });
        }
    }

    let comment = if testfile.extension().is_some_and(|e| e == "rs") { "//@" } else { "#" };

    let mut rdr = BufReader::with_capacity(1024, rdr);
    let mut ln = String::new();
    let mut line_number = 0;

    // Match on error annotations like `//~ERROR`.
    static REVISION_MAGIC_COMMENT_RE: OnceLock<Regex> = OnceLock::new();
    let revision_magic_comment_re =
        REVISION_MAGIC_COMMENT_RE.get_or_init(|| Regex::new("//(\\[.*\\])?~.*").unwrap());

    loop {
        line_number += 1;
        ln.clear();
        if rdr.read_line(&mut ln).unwrap() == 0 {
            break;
        }

        // Assume that any directives will be found before the first
        // module or function. This doesn't seem to be an optimization
        // with a warm page cache. Maybe with a cold one.
        let original_line = &ln;
        let ln = ln.trim();
        if ln.starts_with("fn") || ln.starts_with("mod") {
            return;

        // First try to accept `ui_test` style comments (`//@`)
        } else if let Some((header_revision, non_revisioned_directive_line)) =
            line_directive(comment, ln)
        {
            // Perform unknown directive check on Rust files.
            if testfile.extension().map(|e| e == "rs").unwrap_or(false) {
                let directive_ln = non_revisioned_directive_line.trim();

                let CheckDirectiveResult { is_known_directive, trailing_directive, .. } =
                    check_directive(directive_ln, mode, ln);

                if !is_known_directive {
                    *poisoned = true;

                    eprintln!(
                        "error: detected unknown compiletest test directive `{}` in {}:{}",
                        directive_ln,
                        testfile.display(),
                        line_number,
                    );

                    return;
                }

                if let Some(trailing_directive) = &trailing_directive {
                    *poisoned = true;

                    eprintln!(
                        "error: detected trailing compiletest test directive `{}` in {}:{}\n \
                          help: put the trailing directive in it's own line: `//@ {}`",
                        trailing_directive,
                        testfile.display(),
                        line_number,
                        trailing_directive,
                    );

                    return;
                }
            }

            it(HeaderLine {
                line_number,
                original_line,
                header_revision,
                directive: non_revisioned_directive_line,
            });
        // Then we try to check for legacy-style candidates, which are not the magic ~ERROR family
        // error annotations.
        } else if !revision_magic_comment_re.is_match(ln) {
            let Some((_, rest)) = line_directive("//", ln) else {
                continue;
            };

            if rest.trim_start().starts_with(':') {
                // This is likely a markdown link:
                // `[link_name]: https://example.org`
                continue;
            }

            let rest = rest.trim_start();

            let CheckDirectiveResult { is_known_directive, directive_name, .. } =
                check_directive(rest, mode, ln);

            if is_known_directive {
                *poisoned = true;
                eprintln!(
                    "error: detected legacy-style directive {} in compiletest test: {}:{}, please use `ui_test`-style directives `//@` instead: {:#?}",
                    directive_name,
                    testfile.display(),
                    line_number,
                    line_directive("//", ln),
                );
                return;
            }
        }
    }
}

impl Config {
    fn parse_aux_crate(r: String) -> (String, String) {
        let mut parts = r.trim().splitn(2, '=');
        (
            parts.next().expect("missing aux-crate name (e.g. log=log.rs)").to_string(),
            parts.next().expect("missing aux-crate value (e.g. log=log.rs)").to_string(),
        )
    }

    fn parse_and_update_revisions(&self, line: &str, existing: &mut Vec<String>) {
        if let Some(raw) = self.parse_name_value_directive(line, "revisions") {
            let mut duplicates: HashSet<_> = existing.iter().cloned().collect();
            for revision in raw.split_whitespace().map(|r| r.to_string()) {
                if !duplicates.insert(revision.clone()) {
                    panic!("Duplicate revision: `{}` in line `{}`", revision, raw);
                }
                existing.push(revision);
            }
        }
    }

    fn parse_env(nv: String) -> (String, String) {
        // nv is either FOO or FOO=BAR
        let mut strs: Vec<String> = nv.splitn(2, '=').map(str::to_owned).collect();

        match strs.len() {
            1 => (strs.pop().unwrap(), String::new()),
            2 => {
                let end = strs.pop().unwrap();
                (strs.pop().unwrap(), end)
            }
            n => panic!("Expected 1 or 2 strings, not {}", n),
        }
    }

    fn parse_pp_exact(&self, line: &str, testfile: &Path) -> Option<PathBuf> {
        if let Some(s) = self.parse_name_value_directive(line, "pp-exact") {
            Some(PathBuf::from(&s))
        } else if self.parse_name_directive(line, "pp-exact") {
            testfile.file_name().map(PathBuf::from)
        } else {
            None
        }
    }

    fn parse_custom_normalization(&self, line: &str, prefix: &str) -> Option<(String, String)> {
        let parsed = parse_cfg_name_directive(self, line, prefix);
        if parsed.outcome != MatchOutcome::Match {
            return None;
        }
        let name = parsed.name.expect("successful match always has a name");

        let Some((regex, replacement)) = parse_normalize_rule(line) else {
            panic!(
                "couldn't parse custom normalization rule: `{line}`\n\
                help: expected syntax is: `{prefix}-{name}: \"REGEX\" -> \"REPLACEMENT\"`"
            );
        };
        Some((regex, replacement))
    }

    fn parse_name_directive(&self, line: &str, directive: &str) -> bool {
        // Ensure the directive is a whole word. Do not match "ignore-x86" when
        // the line says "ignore-x86_64".
        line.starts_with(directive)
            && matches!(line.as_bytes().get(directive.len()), None | Some(&b' ') | Some(&b':'))
    }

    fn parse_negative_name_directive(&self, line: &str, directive: &str) -> bool {
        line.starts_with("no-") && self.parse_name_directive(&line[3..], directive)
    }

    pub fn parse_name_value_directive(&self, line: &str, directive: &str) -> Option<String> {
        let colon = directive.len();
        if line.starts_with(directive) && line.as_bytes().get(colon) == Some(&b':') {
            let value = line[(colon + 1)..].to_owned();
            debug!("{}: {}", directive, value);
            Some(expand_variables(value, self))
        } else {
            None
        }
    }

    pub fn find_rust_src_root(&self) -> Option<PathBuf> {
        let mut path = self.src_base.clone();
        let path_postfix = Path::new("src/etc/lldb_batchmode.py");

        while path.pop() {
            if path.join(&path_postfix).is_file() {
                return Some(path);
            }
        }

        None
    }

    fn parse_edition(&self, line: &str) -> Option<String> {
        self.parse_name_value_directive(line, "edition")
    }

    fn set_name_directive(&self, line: &str, directive: &str, value: &mut bool) {
        match value {
            true => {
                if self.parse_negative_name_directive(line, directive) {
                    *value = false;
                }
            }
            false => {
                if self.parse_name_directive(line, directive) {
                    *value = true;
                }
            }
        }
    }

    fn set_name_value_directive<T>(
        &self,
        line: &str,
        directive: &str,
        value: &mut Option<T>,
        parse: impl FnOnce(String) -> T,
    ) {
        if value.is_none() {
            *value = self.parse_name_value_directive(line, directive).map(parse);
        }
    }

    fn push_name_value_directive<T>(
        &self,
        line: &str,
        directive: &str,
        values: &mut Vec<T>,
        parse: impl FnOnce(String) -> T,
    ) {
        if let Some(value) = self.parse_name_value_directive(line, directive).map(parse) {
            values.push(value);
        }
    }
}

fn expand_variables(mut value: String, config: &Config) -> String {
    const CWD: &str = "{{cwd}}";
    const SRC_BASE: &str = "{{src-base}}";
    const BUILD_BASE: &str = "{{build-base}}";
    const SYSROOT_BASE: &str = "{{sysroot-base}}";
    const TARGET_LINKER: &str = "{{target-linker}}";
    const TARGET: &str = "{{target}}";

    if value.contains(CWD) {
        let cwd = env::current_dir().unwrap();
        value = value.replace(CWD, &cwd.to_string_lossy());
    }

    if value.contains(SRC_BASE) {
        value = value.replace(SRC_BASE, &config.src_base.to_string_lossy());
    }

    if value.contains(BUILD_BASE) {
        value = value.replace(BUILD_BASE, &config.build_base.to_string_lossy());
    }

    if value.contains(SYSROOT_BASE) {
        value = value.replace(SYSROOT_BASE, &config.sysroot_base.to_string_lossy());
    }

    if value.contains(TARGET_LINKER) {
        value = value.replace(TARGET_LINKER, config.target_linker.as_deref().unwrap_or(""));
    }

    if value.contains(TARGET) {
        value = value.replace(TARGET, &config.target);
    }

    value
}

/// Parses the regex and replacement values of a `//@ normalize-*` header,
/// in the format:
/// ```text
/// normalize-*: "REGEX" -> "REPLACEMENT"
/// ```
fn parse_normalize_rule(header: &str) -> Option<(String, String)> {
    // FIXME: Support escaped double-quotes in strings.
    let captures = static_regex!(
        r#"(?x) # (verbose mode regex)
        ^
        [^:\s]+:\s*             # (header name followed by colon)
        "(?<regex>[^"]*)"       # "REGEX"
        \s+->\s+                # ->
        "(?<replacement>[^"]*)" # "REPLACEMENT"
        $
        "#
    )
    .captures(header)?;
    let regex = captures["regex"].to_owned();
    let replacement = captures["replacement"].to_owned();
    Some((regex, replacement))
}

pub fn extract_llvm_version(version: &str) -> Option<u32> {
    let pat = |c: char| !c.is_ascii_digit() && c != '.';
    let version_without_suffix = match version.find(pat) {
        Some(pos) => &version[..pos],
        None => version,
    };
    let components: Vec<u32> = version_without_suffix
        .split('.')
        .map(|s| s.parse().expect("Malformed version component"))
        .collect();
    let version = match *components {
        [a] => a * 10_000,
        [a, b] => a * 10_000 + b * 100,
        [a, b, c] => a * 10_000 + b * 100 + c,
        _ => panic!("Malformed version"),
    };
    Some(version)
}

pub fn extract_llvm_version_from_binary(binary_path: &str) -> Option<u32> {
    let output = Command::new(binary_path).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let version = String::from_utf8(output.stdout).ok()?;
    for line in version.lines() {
        if let Some(version) = line.split("LLVM version ").nth(1) {
            return extract_llvm_version(version);
        }
    }
    None
}

/// Takes a directive of the form "<version1> [- <version2>]",
/// returns the numeric representation of <version1> and <version2> as
/// tuple: (<version1> as u32, <version2> as u32)
///
/// If the <version2> part is omitted, the second component of the tuple
/// is the same as <version1>.
fn extract_version_range<F>(line: &str, parse: F) -> Option<(u32, u32)>
where
    F: Fn(&str) -> Option<u32>,
{
    let mut splits = line.splitn(2, "- ").map(str::trim);
    let min = splits.next().unwrap();
    if min.ends_with('-') {
        return None;
    }

    let max = splits.next();

    if min.is_empty() {
        return None;
    }

    let min = parse(min)?;
    let max = match max {
        Some("") => return None,
        Some(max) => parse(max)?,
        _ => min,
    };

    Some((min, max))
}

pub fn make_test_description<R: Read>(
    config: &Config,
    cache: &HeadersCache,
    name: test::TestName,
    path: &Path,
    src: R,
    test_revision: Option<&str>,
    poisoned: &mut bool,
) -> test::TestDesc {
    let mut ignore = false;
    let mut ignore_message = None;
    let mut should_fail = false;

    let mut local_poisoned = false;

    iter_header(
        config.mode,
        &config.suite,
        &mut local_poisoned,
        path,
        src,
        &mut |HeaderLine { header_revision, original_line, directive: ln, line_number }| {
            if header_revision.is_some() && header_revision != test_revision {
                return;
            }

            macro_rules! decision {
                ($e:expr) => {
                    match $e {
                        IgnoreDecision::Ignore { reason } => {
                            ignore = true;
                            // The ignore reason must be a &'static str, so we have to leak memory to
                            // create it. This is fine, as the header is parsed only at the start of
                            // compiletest so it won't grow indefinitely.
                            ignore_message = Some(&*Box::leak(Box::<str>::from(reason)));
                        }
                        IgnoreDecision::Error { message } => {
                            eprintln!("error: {}:{line_number}: {message}", path.display());
                            *poisoned = true;
                            return;
                        }
                        IgnoreDecision::Continue => {}
                    }
                };
            }

            if let Some((_, post)) = original_line.trim_start().split_once("//") {
                let post = post.trim_start();
                if post.starts_with("ignore-tidy") {
                    // Not handled by compiletest.
                } else {
                    decision!(cfg::handle_ignore(config, ln));
                }
            } else {
                decision!(cfg::handle_ignore(config, ln));
            }

            decision!(cfg::handle_only(config, ln));
            decision!(needs::handle_needs(&cache.needs, config, ln));
            decision!(ignore_llvm(config, ln));
            decision!(ignore_cdb(config, ln));
            decision!(ignore_gdb(config, ln));
            decision!(ignore_lldb(config, ln));

            if config.target == "wasm32-unknown-unknown"
                && config.parse_name_directive(ln, directives::CHECK_RUN_RESULTS)
            {
                decision!(IgnoreDecision::Ignore {
                    reason: "ignored on WASM as the run results cannot be checked there".into(),
                });
            }

            should_fail |= config.parse_name_directive(ln, "should-fail");
        },
    );

    if local_poisoned {
        eprintln!("errors encountered when trying to make test description: {}", path.display());
        panic!("errors encountered when trying to make test description");
    }

    // The `should-fail` annotation doesn't apply to pretty tests,
    // since we run the pretty printer across all tests by default.
    // If desired, we could add a `should-fail-pretty` annotation.
    let should_panic = match config.mode {
        crate::common::Pretty => test::ShouldPanic::No,
        _ if should_fail => test::ShouldPanic::Yes,
        _ => test::ShouldPanic::No,
    };

    test::TestDesc {
        name,
        ignore,
        ignore_message,
        source_file: "",
        start_line: 0,
        start_col: 0,
        end_line: 0,
        end_col: 0,
        should_panic,
        compile_fail: false,
        no_run: false,
        test_type: test::TestType::Unknown,
    }
}

fn ignore_cdb(config: &Config, line: &str) -> IgnoreDecision {
    if config.debugger != Some(Debugger::Cdb) {
        return IgnoreDecision::Continue;
    }

    if let Some(actual_version) = config.cdb_version {
        if let Some(rest) = line.strip_prefix("min-cdb-version:").map(str::trim) {
            let min_version = extract_cdb_version(rest).unwrap_or_else(|| {
                panic!("couldn't parse version range: {:?}", rest);
            });

            // Ignore if actual version is smaller than the minimum
            // required version
            if actual_version < min_version {
                return IgnoreDecision::Ignore {
                    reason: format!("ignored when the CDB version is lower than {rest}"),
                };
            }
        }
    }
    IgnoreDecision::Continue
}

fn ignore_gdb(config: &Config, line: &str) -> IgnoreDecision {
    if config.debugger != Some(Debugger::Gdb) {
        return IgnoreDecision::Continue;
    }

    if let Some(actual_version) = config.gdb_version {
        if let Some(rest) = line.strip_prefix("min-gdb-version:").map(str::trim) {
            let (start_ver, end_ver) = extract_version_range(rest, extract_gdb_version)
                .unwrap_or_else(|| {
                    panic!("couldn't parse version range: {:?}", rest);
                });

            if start_ver != end_ver {
                panic!("Expected single GDB version")
            }
            // Ignore if actual version is smaller than the minimum
            // required version
            if actual_version < start_ver {
                return IgnoreDecision::Ignore {
                    reason: format!("ignored when the GDB version is lower than {rest}"),
                };
            }
        } else if let Some(rest) = line.strip_prefix("ignore-gdb-version:").map(str::trim) {
            let (min_version, max_version) = extract_version_range(rest, extract_gdb_version)
                .unwrap_or_else(|| {
                    panic!("couldn't parse version range: {:?}", rest);
                });

            if max_version < min_version {
                panic!("Malformed GDB version range: max < min")
            }

            if actual_version >= min_version && actual_version <= max_version {
                if min_version == max_version {
                    return IgnoreDecision::Ignore {
                        reason: format!("ignored when the GDB version is {rest}"),
                    };
                } else {
                    return IgnoreDecision::Ignore {
                        reason: format!("ignored when the GDB version is between {rest}"),
                    };
                }
            }
        }
    }
    IgnoreDecision::Continue
}

fn ignore_lldb(config: &Config, line: &str) -> IgnoreDecision {
    if config.debugger != Some(Debugger::Lldb) {
        return IgnoreDecision::Continue;
    }

    if let Some(actual_version) = config.lldb_version {
        if let Some(rest) = line.strip_prefix("min-lldb-version:").map(str::trim) {
            let min_version = rest.parse().unwrap_or_else(|e| {
                panic!("Unexpected format of LLDB version string: {}\n{:?}", rest, e);
            });
            // Ignore if actual version is smaller the minimum required
            // version
            if actual_version < min_version {
                return IgnoreDecision::Ignore {
                    reason: format!("ignored when the LLDB version is {rest}"),
                };
            }
        }
    }
    IgnoreDecision::Continue
}

fn ignore_llvm(config: &Config, line: &str) -> IgnoreDecision {
    if let Some(needed_components) =
        config.parse_name_value_directive(line, "needs-llvm-components")
    {
        let components: HashSet<_> = config.llvm_components.split_whitespace().collect();
        if let Some(missing_component) = needed_components
            .split_whitespace()
            .find(|needed_component| !components.contains(needed_component))
        {
            if env::var_os("COMPILETEST_REQUIRE_ALL_LLVM_COMPONENTS").is_some() {
                panic!(
                    "missing LLVM component {}, and COMPILETEST_REQUIRE_ALL_LLVM_COMPONENTS is set",
                    missing_component
                );
            }
            return IgnoreDecision::Ignore {
                reason: format!("ignored when the {missing_component} LLVM component is missing"),
            };
        }
    }
    if let Some(actual_version) = config.llvm_version {
        if let Some(rest) = line.strip_prefix("min-llvm-version:").map(str::trim) {
            let min_version = extract_llvm_version(rest).unwrap();
            // Ignore if actual version is smaller the minimum required
            // version
            if actual_version < min_version {
                return IgnoreDecision::Ignore {
                    reason: format!("ignored when the LLVM version is older than {rest}"),
                };
            }
        } else if let Some(rest) = line.strip_prefix("min-system-llvm-version:").map(str::trim) {
            let min_version = extract_llvm_version(rest).unwrap();
            // Ignore if using system LLVM and actual version
            // is smaller the minimum required version
            if config.system_llvm && actual_version < min_version {
                return IgnoreDecision::Ignore {
                    reason: format!("ignored when the system LLVM version is older than {rest}"),
                };
            }
        } else if let Some(rest) = line.strip_prefix("ignore-llvm-version:").map(str::trim) {
            // Syntax is: "ignore-llvm-version: <version1> [- <version2>]"
            let (v_min, v_max) =
                extract_version_range(rest, extract_llvm_version).unwrap_or_else(|| {
                    panic!("couldn't parse version range: {:?}", rest);
                });
            if v_max < v_min {
                panic!("Malformed LLVM version range: max < min")
            }
            // Ignore if version lies inside of range.
            if actual_version >= v_min && actual_version <= v_max {
                if v_min == v_max {
                    return IgnoreDecision::Ignore {
                        reason: format!("ignored when the LLVM version is {rest}"),
                    };
                } else {
                    return IgnoreDecision::Ignore {
                        reason: format!("ignored when the LLVM version is between {rest}"),
                    };
                }
            }
        }
    }
    IgnoreDecision::Continue
}

enum IgnoreDecision {
    Ignore { reason: String },
    Continue,
    Error { message: String },
}
