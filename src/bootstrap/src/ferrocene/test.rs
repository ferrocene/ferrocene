// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{self, SourceType};
use crate::core::config::TargetSelection;
use crate::ferrocene::sign::{document_signatures_cmd, error_when_signatures_are_ignored};
use crate::Mode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraceabilityMatrixTool {
    host: TargetSelection,
}

impl Step for TraceabilityMatrixTool {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/traceability-matrix")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TraceabilityMatrixTool { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Testing ferrocene/tools/traceability-matrix");
        builder.run(
            &mut tool::prepare_tool_cargo(
                builder,
                builder.compiler(0, self.host),
                Mode::ToolBootstrap,
                self.host,
                "test",
                "ferrocene/tools/traceability-matrix",
                SourceType::InTree,
                &[],
            )
            .into(),
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SelfTest {
    target: TargetSelection,
}

impl Step for SelfTest {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/self-test")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Testing ferrocene/tools/self-test");

        let mut cmd = tool::prepare_tool_cargo(
            builder,
            builder.compiler(0, self.target),
            Mode::ToolBootstrap,
            self.target,
            "test",
            "ferrocene/tools/self-test",
            SourceType::InTree,
            &[],
        )
        .into();
        crate::ferrocene::tool::SelfTest::update_command(&mut cmd, builder, self.target);
        builder.run(&mut cmd);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CheckDocumentSignatures {
    target: TargetSelection,
}

impl Step for CheckDocumentSignatures {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default_condition = !matches!(
            run.builder.config.ferrocene_document_signatures,
            crate::core::config::FerroceneDocumentSignatures::Disabled
        );
        run.alias("ferrocene-check-document-signatures").default_condition(default_condition)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(CheckDocumentSignatures { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        error_when_signatures_are_ignored(builder, "check document signatures");

        builder.info("Checking document signatures");
        crate::ferrocene::sign::for_each_signable_document(
            builder,
            self.target,
            // Condition
            |source| source.join("signature").join("signature.toml").exists(),
            // Function executed
            |source, output| {
                builder.run(
                    document_signatures_cmd(builder, source).arg("verify").arg(source).arg(output),
                );
            },
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct GenerateTarball {
    target: TargetSelection,
}

impl Step for GenerateTarball {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) {
        builder.info("test generate-tarball");

        let compiler = builder.compiler(0, self.target);
        let cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            "test",
            "ferrocene/tools/generate-tarball",
            SourceType::InTree,
            &[],
        );
        crate::core::build_steps::test::run_cargo_test(
            cargo,
            &[],
            &[],
            "generate-tarball",
            "generate-tarball",
            compiler,
            self.target,
            builder,
        );
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/generate-tarball")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self { target: run.target });
    }
}
