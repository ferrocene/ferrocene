// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use serde_json::Value;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::compile::Std;
use crate::core::build_steps::tool::{self, SourceType};
use crate::core::config::TargetSelection;
use crate::ferrocene::sign::error_when_signatures_are_ignored;
use crate::utils::exec::BootstrapCommand;
use crate::{Kind, Mode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TraceabilityMatrixTool {
    host: TargetSelection,
}

impl Step for TraceabilityMatrixTool {
    type Output = ();
    const DEFAULT: bool = true;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/traceability-matrix")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TraceabilityMatrixTool { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Testing ferrocene/tools/traceability-matrix");
        tool::prepare_tool_cargo(
            builder,
            builder.compiler(0, self.host),
            Mode::ToolBootstrap,
            self.host,
            Kind::Test,
            "ferrocene/tools/traceability-matrix",
            SourceType::InTree,
            &[],
        )
        .into_cmd()
        .run(builder);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SelfTest {
    target: TargetSelection,
}

impl Step for SelfTest {
    type Output = ();
    const DEFAULT: bool = true;
    const IS_HOST: bool = true;

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
            Kind::Test,
            "ferrocene/tools/self-test",
            SourceType::InTree,
            &[],
        )
        .into();
        crate::ferrocene::tool::SelfTest::update_command(&mut cmd, builder, self.target);
        cmd.run(builder);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CheckDocumentSignatures {
    target: TargetSelection,
}

impl Step for CheckDocumentSignatures {
    type Output = ();
    const DEFAULT: bool = true;
    const IS_HOST: bool = true;

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
            |mut cmd, source, output| {
                cmd.arg("verify").arg(source).arg(output).run(builder);
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
    const IS_HOST: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) {
        builder.info("test generate-tarball");

        let compiler = builder.compiler(0, self.target);
        let cargo = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            Kind::Test,
            "ferrocene/tools/generate-tarball",
            SourceType::InTree,
            &[],
        );
        crate::core::build_steps::test::run_cargo_test(
            cargo,
            &[],
            &[],
            "generate-tarball",
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlipLink {
    host: TargetSelection,
}

impl Step for FlipLink {
    type Output = ();
    const DEFAULT: bool = true;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/flip-link")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(FlipLink { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let compiler = builder.compiler(0, self.host);
        // The flip link tests require a thumbv7em-none-eabi target to exist
        builder.ensure(Std::new(compiler, TargetSelection::from_user("thumbv7em-none-eabi")));

        builder.info("Testing ferrocene/tools/flip-link");

        let out = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolTarget,
            self.host,
            Kind::Test,
            "ferrocene/tools/flip-link",
            SourceType::Submodule,
            &[],
        )
        .into_cmd()
        .args(["--no-run", "--message-format", "json"])
        .run_capture_stdout(builder);

        let mut test_artifacts = vec![];
        for line in out.stdout().lines() {
            let parsed: serde_json::Value = serde_json::from_str(&line).unwrap();

            // We only care about flip-link
            let Some(package_id) = parsed.pointer("/package_id").and_then(Value::as_str) else {
                continue;
            };
            if !package_id.contains("flip-link") {
                continue;
            }

            // Ensure we only get test binaries, not the actual `flip-link`.
            let Some(test) = parsed.pointer("/profile/test").and_then(Value::as_bool) else {
                continue;
            };
            if !test {
                continue;
            }

            let Some(kind) = parsed
                .pointer("/target/kind")
                .and_then(Value::as_array)
                .and_then(|v| v.first())
                .and_then(Value::as_str)
            else {
                continue;
            };
            if !(kind == "test" || kind == "bin") {
                continue;
            }

            let Some(executable) = parsed.pointer("/executable").and_then(Value::as_str) else {
                panic!(
                    "\
                    No executable found in cargo line that should include executable\n\
                    {line}\
                "
                );
            };

            test_artifacts.push(executable.to_string());
        }

        for artifact in test_artifacts {
            builder.info(&format!("Testing flip-link test binary ({artifact})"));
            BootstrapCommand::new(artifact).current_dir("ferrocene/tools/flip-link").run(builder);
        }
    }
}
