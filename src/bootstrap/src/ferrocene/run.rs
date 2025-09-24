// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod coverage_of_subset;

use std::path::PathBuf;

use crate::Compiler;
use crate::builder::{Builder, Cargo, RunConfig, ShouldRun, Step, crate_description};
use crate::core::build_steps::tool::{Tool, SourceType};
use crate::core::build_steps::compile::{run_cargo, std_cargo};
use crate::core::config::{FerroceneTraceabilityMatrixMode, TargetSelection};
use crate::ferrocene::doc::{Specification, SphinxMode, UserManual};
use crate::ferrocene::test_outcomes::TestOutcomesDir;
use crate::ferrocene::tool::{SymbolReport, SYMBOL_PATH};
use crate::utils::exec::BootstrapCommand;
use crate::utils::build_stamp;
use crate::{Kind, Mode};

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

impl Step for CertifiedCoreSymbols {
    type Output = PathBuf;
    const DEFAULT: bool = false;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(SYMBOL_PATH).alias("certified-core-symbols")
    }

    fn make_run(run: RunConfig<'_>) {
        let build_compiler = run.builder.compiler(run.builder.top_stage.max(1), run.build_triple());
        run.builder.ensure(CertifiedCoreSymbols { build_compiler, target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let CertifiedCoreSymbols { build_compiler, target } = self;
        let symbol_report = builder.ensure(SymbolReport { target_compiler: build_compiler });

        let certified_target = self
            .target
            .certified_equivalent()
            .expect(&format!("no certified equivalent exists for target \"{target}\""));

        // c.f. check::std
        let mut cargo = Cargo::new(
            builder,
            build_compiler,
            Mode::Std,
            SourceType::InTree,
            certified_target,
            Kind::Check,
        );
        let crates = vec!["core".to_owned()];  // currently, only core is certified
        for krate in &*crates {
            cargo.arg("-p").arg(krate);
        }
        std_cargo(builder, certified_target, &mut cargo, &crates);
        cargo.env("RUSTC_REAL", symbol_report);
        let report = builder.cargo_out(build_compiler, Mode::Std, certified_target).join("symbol-report.txt");
        cargo.env("SYMBOL_REPORT_OUT", &report);

        let _guard = builder.msg(
            Kind::Run,
            format_args!("symbol-report for certified library subset{}", crate_description(&crates)),
            Mode::Std,
            build_compiler,
            certified_target,
        );

        let check_stamp =
            build_stamp::libstd_stamp(builder, build_compiler, certified_target).with_prefix("symbol-report");
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
