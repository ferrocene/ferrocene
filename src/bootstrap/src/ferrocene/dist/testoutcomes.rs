// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::ferrocene::test_outcomes::TestOutcomesDir;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TestOutcomes;

impl Step for TestOutcomes {
    type Output = Option<GeneratedTarball>;
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-test-outcomes")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TestOutcomes);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let Some(test_outcomes) = builder.ensure(TestOutcomesDir) else { return None };

        let tarball = Tarball::new_targetless(builder, "ferrocene-test-outcomes");
        tarball.add_dir(test_outcomes, "share/ferrocene/test-outcomes");
        Some(tarball.generate())
    }
}
