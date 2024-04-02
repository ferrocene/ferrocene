// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::utils::tarball::GeneratedTarball;

use super::subsetter::Subsetter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Docs {
    target: TargetSelection,
}

impl Step for Docs {
    type Output = Vec<GeneratedTarball>;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.docs;
        run.alias("ferrocene-docs").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Docs { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        // Build all of the documentation.
        builder.default_doc(&[]);
        let doc_out = builder.out.join(&self.target.triple).join("doc");

        let mut subsetter = Subsetter::new(builder, "ferrocene-docs", "share/doc/ferrocene/html");
        subsetter.add_directory(&doc_out, &doc_out);

        subsetter.into_tarballs().map(|tarball| tarball.generate()).collect()
    }
}
