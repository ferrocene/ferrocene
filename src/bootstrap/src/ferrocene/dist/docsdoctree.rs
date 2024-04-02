// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::doc::ensure_all_xml_doctrees;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct DocsDoctrees {
    target: TargetSelection,
}

impl Step for DocsDoctrees {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.docs;
        run.alias("ferrocene-docs-doctrees").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let paths = ensure_all_xml_doctrees(builder, self.target);

        let tarball = Tarball::new_targetless(builder, "ferrocene-docs-doctrees");
        for (name, path) in paths {
            tarball.add_dir(path, format!("share/doc/ferrocene/xml-doctrees/{name}"));
        }

        tarball.generate()
    }
}
