use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context as _, Result};
use llvm_profparser::CoverageReport;

use crate::{FunctionCoverage, ShowCommand, Span};

#[derive(serde::Deserialize)]
#[serde(transparent)]
struct Symbols(Vec<Function>);

#[derive(serde::Deserialize)]
struct Function(String, PathBuf, usize, usize);

pub fn coverage(cmd: &ShowCommand, report: &CoverageReport) -> Result<Vec<FunctionCoverage>> {
    let symbols: Symbols = serde_json::from_reader(
        File::open(&cmd.symbol_report)
            .context(format!("failed to open symbol file {}", cmd.symbol_report.display()))?,
    )?;
    let mut coverage = vec![];
    for Function(qualified_name, filename, start_line, end_line) in symbols.0 {
        let span = Span { filename, start_line, end_line };
        coverage.push(super::get_coverage(report, span, &cmd.ferrocene, qualified_name)?);
    }
    Ok(coverage)
}
