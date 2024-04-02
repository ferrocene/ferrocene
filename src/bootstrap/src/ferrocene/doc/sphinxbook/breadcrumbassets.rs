// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::builder::{Builder, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(super) struct BreadcrumbsAssets {
    pub(super) target: TargetSelection,
    pub(super) dest: Option<PathBuf>,
}

impl Step for BreadcrumbsAssets {
    type Output = ();
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let src = builder.src.join("ferrocene/doc/breadcrumbs/ferrocene-breadcrumbs.css");
        let dest = self
            .dest
            .unwrap_or_else(|| builder.doc_out(self.target))
            .join("ferrocene-breadcrumbs.css");

        if let Some(parent) = dest.parent() {
            if !parent.is_dir() {
                builder.create_dir(parent);
            }
        }
        builder.copy_link(&src, &dest);
    }
}
