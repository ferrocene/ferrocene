// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;
use std::process::Command;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::{prepare_tool_cargo, SourceType};
use crate::core::config::TargetSelection;
use crate::{exe, Mode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SelfTest {
    pub(crate) target: TargetSelection,
}

impl SelfTest {
    pub(crate) fn update_command(
        cmd: &mut Command,
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
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path("ferrocene/tools/self-test")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Building ferrocene/tools/self-test");

        let compiler = builder.compiler(0, builder.config.build);
        let mut cmd = prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolBootstrap,
            self.target,
            "build",
            "ferrocene/tools/self-test",
            SourceType::InTree,
            &[],
        )
        .into();
        Self::update_command(&mut cmd, builder, self.target);

        builder.run(&mut cmd);

        builder
            .cargo_out(compiler, Mode::ToolBootstrap, self.target)
            .join(exe("ferrocene-self-test", self.target))
    }
}
