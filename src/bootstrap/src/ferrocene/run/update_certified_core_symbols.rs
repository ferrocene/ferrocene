// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;

use build_helper::symbol_report::SymbolReport;

use super::CertifiedCoreSymbols;
use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct UpdateCertifiedCoreSymbols;

pub(crate) const TRACKED_FILE: &str = "ferrocene/doc/symbol-report.csv";

impl Step for UpdateCertifiedCoreSymbols {
    type Output = ();
    const DEFAULT: bool = false;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(TRACKED_FILE).alias("update-certified-core-symbols")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(UpdateCertifiedCoreSymbols);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let target = TargetSelection::from_user("x86_64-unknown-linux-gnu");
        let symbol_report_path = builder.ensure(CertifiedCoreSymbols::new(builder, target));

        if builder.config.dry_run() {
            return;
        }

        let symbol_report_content = fs::read_to_string(&symbol_report_path).unwrap();
        let symbol_report = serde_json::from_str::<SymbolReport>(&symbol_report_content).unwrap();

        let qualified_name_list = symbol_report.to_qualified_fn_list();
        std::fs::write(TRACKED_FILE, qualified_name_list.join("\n")).unwrap();

        eprintln!("Updated {TRACKED_FILE}");
    }
}
