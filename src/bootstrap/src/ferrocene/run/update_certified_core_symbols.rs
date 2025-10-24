// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;

use build_helper::symbol_report::SymbolReport;

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
        let symbol_report_path =
            builder.ensure(CertifiedCoreSymbols { build_compiler, target: self.host });
        let symbol_report_content = fs::read_to_string(&symbol_report_path).unwrap();
        let symbol_report = serde_json::from_str::<SymbolReport>(&symbol_report_content).unwrap();

        let qualified_name_list = symbol_report.to_qualified_fn_list();
        let s = serde_json::to_string_pretty(&qualified_name_list).unwrap();
        fs::write(TRACKED_FILE, s).unwrap();

        eprintln!("Updated {TRACKED_FILE}");
    }
}
