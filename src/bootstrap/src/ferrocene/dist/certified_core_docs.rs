// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::doc;
use crate::core::config::TargetSelection;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CertifiedCoreDocs {
    target: TargetSelection,
}

impl CertifiedCoreDocs {
    const PATH: &str = "ferrocene-certified-core-docs";
}

impl Step for CertifiedCoreDocs {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const IS_HOST: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.docs;
        run.alias(Self::PATH).default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let doc_path = builder.ensure(doc::Std::new(
            builder.compiler(builder.top_stage, builder.host_target),
            self.target,
            doc::DocumentationFormat::Html,
            vec!["core".into()],
        ));

        let tarball = Tarball::new(builder, Self::PATH, &self.target.triple);
        tarball.add_dir(doc_path, "share/doc/ferrocene/certified-core-docs");

        tarball.generate()
    }
}
