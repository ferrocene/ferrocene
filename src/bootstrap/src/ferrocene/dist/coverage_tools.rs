// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::core::builder::{Builder, CommandLineStep, RunConfig, ShouldRun};
use crate::core::compiler::Compiler;
use crate::core::config::TargetSelection;
use crate::core::session::FileType;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SymbolReport {
    /// `symbol-report` links to `rustc_driver`, so it only works with this compiler.
    pub(crate) target_compiler: Compiler,
}

impl CommandLineStep for SymbolReport {
    type Output = GeneratedTarball;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("symbol-report")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SymbolReport {
            target_compiler: run.builder.compiler(run.builder.top_stage, run.target),
        });
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let symbol_report = builder
            .ensure(crate::ferrocene::tool::SymbolReport { target_compiler: self.target_compiler });

        let mut tarball = Tarball::new(builder, "symbol-report", &self.target_compiler.host.triple);
        tarball.add_file(symbol_report, "bin", FileType::Executable);
        tarball.ferrocene_proxied_binary("bin/symbol-report");
        tarball.generate()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Blanket {
    pub(crate) target: TargetSelection,
}

impl CommandLineStep for Blanket {
    type Output = GeneratedTarball;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("blanket")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Blanket { target: run.target });
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let blanket = builder.ensure(crate::ferrocene::tool::Blanket { target: self.target });

        let mut tarball = Tarball::new(builder, "blanket", &self.target.triple);
        tarball.add_file(blanket, "bin", FileType::Executable);
        tarball.ferrocene_proxied_binary("bin/blanket");
        tarball.generate()
    }
}
