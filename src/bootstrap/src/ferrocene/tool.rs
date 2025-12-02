// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod flip_link;

use std::path::PathBuf;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{
    RustcPrivateCompilers, SourceType, ToolArtifactKind, ToolBuild, prepare_tool_cargo,
};
use crate::core::config::TargetSelection;
use crate::utils::exec::BootstrapCommand;
use crate::{Compiler, Kind, Mode, exe};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SelfTest {
    pub(crate) target: TargetSelection,
}

impl SelfTest {
    pub(super) fn update_command(
        cmd: &mut BootstrapCommand,
        builder: &Builder<'_>,
        target: TargetSelection,
    ) {
        cmd.env("SELFTEST_TARGET", target.to_string());
        if let Some(hash) = builder.rust_info().sha() {
            cmd.env("SELFTEST_RUST_HASH", hash);
        }
        if let Some(hash) = builder.cargo_info.sha() {
            cmd.env("SELFTEST_CARGO_HASH", hash);
        }
    }
}

impl Step for SelfTest {
    type Output = PathBuf;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/self-test")
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Building ferrocene/tools/self-test");

        let compiler = builder.compiler(0, builder.config.host_target);
        let mut cmd = prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            Kind::Build,
            "ferrocene/tools/self-test",
            SourceType::InTree,
            &[],
        )
        .into();
        Self::update_command(&mut cmd, builder, self.target);

        cmd.run(builder);

        builder
            .cargo_out(compiler, Mode::ToolBootstrap, self.target)
            .join(exe("ferrocene-self-test", self.target))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SymbolReport {
    pub(crate) target_compiler: Compiler,
}
pub(super) const SYMBOL_PATH: &str = "ferrocene/tools/symbol-report";

impl Step for SymbolReport {
    type Output = PathBuf;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(SYMBOL_PATH)
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SymbolReport {
            target_compiler: run.builder.compiler(run.builder.top_stage, run.target),
        });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let mode = Mode::ToolRustcPrivate;
        let compilers = RustcPrivateCompilers::from_target_compiler(builder, self.target_compiler);
        let tool_build = ToolBuild {
            build_compiler: compilers.build_compiler(),
            target: self.target_compiler.host,
            tool: "symbol-report",
            mode,
            path: SYMBOL_PATH,
            source_type: SourceType::InTree,
            extra_features: vec![],
            allow_features: "",
            cargo_args: Vec::new(),
            artifact_kind: ToolArtifactKind::Binary,
        };
        builder.ensure(tool_build).tool_path
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Blanket {}
pub(super) const BLANKET_PATH: &str = "ferrocene/tools/blanket";

impl Step for Blanket {
    type Output = PathBuf;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(BLANKET_PATH)
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Blanket {});
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let mode = Mode::ToolBootstrap;
        let tool_build = ToolBuild {
            build_compiler: builder.compiler(0, builder.host_target),
            target: builder.host_target,
            tool: "blanket",
            mode,
            path: BLANKET_PATH,
            source_type: SourceType::InTree,
            extra_features: vec![],
            allow_features: "",
            cargo_args: Vec::new(),
            artifact_kind: ToolArtifactKind::Binary,
        };
        builder.ensure(tool_build).tool_path
    }
}
