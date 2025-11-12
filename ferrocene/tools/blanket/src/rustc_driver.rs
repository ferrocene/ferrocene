use std::collections::BTreeMap;
use std::fs::File;

use anyhow::{Context as _, Result, bail};
use build_helper::symbol_report::{Function, SymbolReport};
use llvm_profparser::CoverageReport;

use crate::{Annotation, FunctionCoverage, ShowCommand, Span};

pub fn coverage(cmd: &ShowCommand, report: &CoverageReport) -> Result<Vec<FunctionCoverage>> {
    let SymbolReport { symbols, annotations }: SymbolReport = serde_json::from_reader(
        File::open(&cmd.symbol_report)
            .context(format!("failed to open symbol file {}", cmd.symbol_report.display()))?,
    )?;
    let mut coverage = vec![];

    let mut annotations = annotations
        .into_iter()
        .map(|(filename, annotations)| {
            (
                filename,
                annotations
                    .into_iter()
                    .map(|(start, end)| Annotation { start, end, unused: true })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for Function { qualified_name, filename, start_line, end_line } in symbols {
        let annotations = annotations.get_mut(&filename);
        let span = Span { filename: filename.into(), start_line, end_line };
        coverage.push(super::get_coverage(
            report,
            span,
            &cmd.ferrocene,
            qualified_name,
            annotations,
        )?);
    }

    for (filename, annotations) in annotations {
        for Annotation { start, end: _, unused } in annotations {
            if unused {
                bail!("Unused annotation in {filename}:{start}");
            }
        }
    }

    Ok(coverage)
}
