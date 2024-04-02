// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::builder::{Builder, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::doc::WithSource;

use super::{document_signatures_cmd, error_when_signatures_are_ignored};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct SignDocument<S: Step<Output = PathBuf> + WithSource> {
    pub(super) target: TargetSelection,
    pub(super) document: S,
}

impl<S: Step<Output = PathBuf> + WithSource> Step for SignDocument<S> {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) {
        error_when_signatures_are_ignored(builder, "sign a document");

        let document = builder.ensure(self.document);
        let source_dir = builder.src.join(S::SOURCE);
        builder.run(
            document_signatures_cmd(builder, &source_dir)
                .arg("sign")
                .arg(&source_dir)
                .arg(&document),
        );
    }
}
