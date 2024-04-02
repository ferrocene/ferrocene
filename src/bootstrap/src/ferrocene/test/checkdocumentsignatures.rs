// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::sign::{document_signatures_cmd, error_when_signatures_are_ignored};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CheckDocumentSignatures {
    target: TargetSelection,
}

impl Step for CheckDocumentSignatures {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default_condition = !run.builder.config.ferrocene_ignore_document_signatures;
        run.alias("ferrocene-check-document-signatures").default_condition(default_condition)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(CheckDocumentSignatures { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        error_when_signatures_are_ignored(builder, "check document signatures");

        builder.info("Checking document signatures");
        crate::ferrocene::sign::for_each_signable_document(
            builder,
            self.target,
            // Condition
            |source| source.join("signature").join("signature.toml").exists(),
            // Function executed
            |source, output| {
                builder.run(
                    document_signatures_cmd(builder, source).arg("verify").arg(source).arg(output),
                );
            },
        );
    }
}
