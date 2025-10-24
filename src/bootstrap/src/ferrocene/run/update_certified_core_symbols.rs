// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;

use super::CertifiedCoreSymbols;
use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct UpdateCertifiedCoreSymbols {
    host: TargetSelection,
}

pub(crate) const TRACKED_FILE: &str = "ferrocene/doc/symbol-report.json";

impl Step for UpdateCertifiedCoreSymbols {
    type Output = ();
    const DEFAULT: bool = false;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("update-certified-core-symbols")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(UpdateCertifiedCoreSymbols { host: run.build_triple() });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let build_compiler = builder.compiler(builder.top_stage.max(1), self.host);
        let path = builder.ensure(CertifiedCoreSymbols { build_compiler, target: self.host });
        fs::copy(path, TRACKED_FILE).unwrap();
        eprintln!("Updated {TRACKED_FILE}");
    }
}
