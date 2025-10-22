use std::fs::File;

use anyhow::{Context as _, Result};
use build_helper::symbol_report::{Function, SymbolReport};
use llvm_profparser::CoverageReport;

use crate::{FunctionCoverage, ShowCommand, Span};

pub fn coverage(cmd: &ShowCommand, report: &CoverageReport) -> Result<Vec<FunctionCoverage>> {
    let symbol_report: SymbolReport = serde_json::from_reader(
        File::open(&cmd.symbol_report)
            .context(format!("failed to open symbol file {}", cmd.symbol_report.display()))?,
    )?;
    let mut coverage = vec![];
    for Function { qualified_name, filename, start_line, end_line } in symbol_report.symbols {
        let annotations = symbol_report.annotations.get(&filename);
        let span = Span { filename: filename.into(), start_line, end_line };
        coverage.push(super::get_coverage(
            report,
            span,
            &cmd.ferrocene,
            qualified_name,
            annotations,
        )?);
    }
    Ok(coverage)
}
