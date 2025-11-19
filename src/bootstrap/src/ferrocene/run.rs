// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod update_certified_core_symbols;

use std::path::{Path, PathBuf};

use crate::builder::{Builder, Cargo, RunConfig, ShouldRun, Step, crate_description};
use crate::core::build_steps::compile::{run_cargo, std_cargo};
use crate::core::build_steps::tool::{SourceType, Tool};
use crate::core::config::flags::FerroceneCoverageFor;
use crate::core::config::{FerroceneTraceabilityMatrixMode, TargetSelection};
use crate::ferrocene::code_coverage::{self, CoverageState, Paths, coverage_file};
use crate::ferrocene::doc::{Specification, SphinxMode, UserManual};
use crate::ferrocene::test_outcomes::TestOutcomesDir;
use crate::ferrocene::tool::{Blanket, SYMBOL_PATH, SymbolReport};
use crate::utils::channel::GitInfo;
use crate::utils::exec::{self, BootstrapCommand};
use crate::utils::{build_stamp, helpers};
use crate::{Compiler, Kind, Mode};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct TraceabilityMatrix {
    pub(crate) target: TargetSelection,
    pub(crate) compiler: Compiler,
}

impl Step for TraceabilityMatrix {
    type Output = PathBuf;
    const IS_HOST: bool = true;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/traceability-matrix")
    }

    fn make_run(run: RunConfig<'_>) {
        let compiler = run.builder.compiler(run.builder.top_stage, run.build_triple());
        run.builder.ensure(TraceabilityMatrix { target: run.target, compiler });
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
        for (suite, mode) in &[("ui", "ui"), ("run-make", "run-make")] {
            builder.info(&format!("Loading test annotations from {suite}"));

            let dest = test_annotations_base.join(format!("{}.json", suite.replace('/', "-")));
            BootstrapCommand::new(&compiletest)
                .env("FERROCENE_COLLECT_ANNOTATIONS", "1")
                .env("FERROCENE_DEST", dest)
                .env("FERROCENE_SRC_ROOT", &builder.src)
                .env("FERROCENE_SRC_TEST_SUITE_ROOT", builder.src.join("tests").join(suite))
                .env("FERROCENE_BUILD_ROOT", &builder.out)
                .env(
                    "FERROCENE_BUILD_TEST_SUITE_ROOT",
                    builder.out.join(self.compiler.host).join(suite),
                )
                .env("FERROCENE_MODE", mode)
                .env("FERROCENE_SUITE", suite)
                .run(builder);
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

        cmd.run(builder);
        html_output
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CertifiedCoreSymbols {
    pub(super) build_compiler: Compiler,
    pub(super) target: TargetSelection,
}

impl CertifiedCoreSymbols {
    pub(super) fn new(builder: &Builder<'_>, target: TargetSelection) -> Self {
        // We need at least stage 1 so that our compiler knows about .certified targets.
        let stage = builder.top_stage.max(1);
        let build_compiler = builder.compiler(stage, builder.config.host_target);
        CertifiedCoreSymbols { build_compiler, target }
    }
}

pub(super) const CERTIFIED_CORE_SYMBOLS_ALIAS: &str = "certified-core-symbols";

impl Step for CertifiedCoreSymbols {
    type Output = PathBuf;
    const DEFAULT: bool = false;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(SYMBOL_PATH).alias(CERTIFIED_CORE_SYMBOLS_ALIAS)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(CertifiedCoreSymbols::new(run.builder, run.target));
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let CertifiedCoreSymbols { build_compiler, target } = self;
        let symbol_report = builder.ensure(SymbolReport { target_compiler: build_compiler });

        let certified_target = target.certified_equivalent();

        // c.f. check::std
        let mut cargo = Cargo::new(
            builder,
            build_compiler,
            Mode::Std,
            SourceType::InTree,
            certified_target,
            Kind::Check,
        );
        let crates = vec!["core".to_owned()]; // currently, only core is certified
        std_cargo(builder, certified_target, &mut cargo, &crates);
        cargo.env("RUSTC_REAL", symbol_report);
        let report = builder
            .cargo_out(build_compiler, Mode::Std, certified_target)
            .join("symbol-report.json");
        cargo.env("SYMBOL_REPORT_OUT", &report);

        let _guard = builder.msg(
            Kind::Run,
            format!("symbol-report for certified library subset{}", crate_description(&crates)),
            Mode::Std,
            build_compiler,
            certified_target,
        );

        let check_stamp = build_stamp::libstd_stamp(builder, build_compiler, certified_target)
            .with_prefix("symbol-report");
        run_cargo(
            builder,
            cargo,
            builder.config.free_args.clone(),
            &check_stamp,
            vec![],
            true,
            false,
        );
        drop(_guard);

        println!("Generated report at {}", report.display());
        report
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CoverageReport {
    pub(super) certified_target: TargetSelection,
    pub(super) profdata: PathBuf,
    pub(super) symbol_report: PathBuf,
    pub(super) instrumented_binaries: Vec<PathBuf>,
}

impl Step for CoverageReport {
    type Output = PathBuf;
    const DEFAULT: bool = false;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-coverage-report")
    }

    fn make_run(run: RunConfig<'_>) {
        let builder = run.builder;
        let certified_target = run.target.certified_equivalent();
        let for_ = FerroceneCoverageFor::Library;
        let paths = Paths::find(builder, run.target, for_);
        // FIXME(@jyn514): this is not a good CLI interface.
        // I would like to have `x run coverage-report` by itself have a stamp file, and rerun tests
        // whenever it's out of date.
        //
        // Additionally, we would like to let people choose which tests get run when the report is
        // generated, especially since doctests don't work with coverage today but will likely get fixed soon.
        // Rather than trying to make this Step very smart, it's done as part of `generate_coverage_report`
        // after running tests.
        if !paths.profdata_file.exists() {
            panic!(
                "ferrocene-coverage-report requires you to have already run tests. consider running `x test --coverage=library` first."
            );
        }

        // We need at least stage 1 so that our compiler knows about .certified targets.
        let build_compiler = builder.compiler(builder.top_stage.max(1), builder.config.host_target);
        let state =
            CoverageState { compiler: build_compiler, target: run.target, coverage_for: for_ };
        let instrumented_binaries = code_coverage::instrumented_binaries(builder, &paths, &state);

        let symbol_report = builder.ensure(CertifiedCoreSymbols::new(builder, certified_target));

        builder.ensure(CoverageReport {
            certified_target,
            profdata: paths.profdata_file,
            instrumented_binaries,
            symbol_report,
        });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let src = to_str(&builder.src);
        let html_out = coverage_file(&builder, self.certified_target);
        let sha_buf;
        let sha = match builder.rust_info() {
            GitInfo::Absent => panic!(
                "collecting coverage for dist profile requires git info, either through a .git repo or a hash recorded in a tarball"
            ),
            GitInfo::RecordedForTarball(recorded) => &recorded.sha,
            GitInfo::Present(Some(cached)) => &cached.sha,
            // In `dev` builds, `omit_git_info` will be set to false.
            // Rather than refactoring all of bootstrap to try and switch on that info,
            // just run `git rev-parse` here real quick.
            GitInfo::Present(None) => {
                sha_buf = helpers::git(None)
                    .args(&["rev-parse", "HEAD"])
                    .run_capture_stdout(&builder.config)
                    .stdout();
                sha_buf.trim()
            }
        };
        let remap_path = format!("/rustc/{sha},{src}");
        fn to_str(p: &Path) -> &str {
            p.as_os_str().to_str().expect("invalid utf8 in path")
        }

        let mut blanket = exec::command(builder.ensure(Blanket {}));
        blanket.args(&[
            "show",
            "--instr-profile",
            to_str(&self.profdata),
            "--report",
            to_str(&self.symbol_report),
            "--path-equivalence",
            &remap_path,
            "--ferrocene-src",
            src,
            "--html-out",
            to_str(&html_out),
        ]);
        for bin in self.instrumented_binaries {
            blanket.args(&["--object", to_str(&bin)]);
        }
        if builder.verbosity > 1 {
            blanket.arg("--debug");
        }
        blanket.run(&builder.config);
        html_out
    }
}
