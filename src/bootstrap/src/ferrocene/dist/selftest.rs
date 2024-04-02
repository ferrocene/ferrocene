// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SelfTest {
    pub(crate) target: TargetSelection,
}

impl Step for SelfTest {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-self-test")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let self_test = builder.ensure(crate::ferrocene::tool::SelfTest { target: self.target });

        let tarball = Tarball::new(builder, "ferrocene-self-test", &self.target.triple);
        tarball.add_file(self_test, "bin", 0o755);
        tarball.generate()
    }
}
