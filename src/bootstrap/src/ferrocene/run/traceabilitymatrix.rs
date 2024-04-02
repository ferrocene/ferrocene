// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;
use std::process::Command;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::Tool;
use crate::core::config::{FerroceneTraceabilityMatrixMode, TargetSelection};
use crate::ferrocene::doc::{Specification, SphinxMode, UserManual};
use crate::ferrocene::test_outcomes::TestOutcomesDir;

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
