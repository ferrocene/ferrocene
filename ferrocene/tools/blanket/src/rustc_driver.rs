use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context as _, Result, bail};
use build_helper::symbol_report::{Function, SymbolReport};
use llvm_profparser::CoverageReport;

use crate::{Annotation, FunctionCoverage, ShowCommand, Span};

/// When a macro is defined in `core` but is used in `alloc` the resulting
/// expanded items will have their paths in debuginfo in a form like this:
///
/// `/rustc/<sha>/library/core/src/ub_checks.rs`.
///
/// For coverage we need to convert these paths to relative.
fn remap_filename(filename: PathBuf, path_remapping: Option<&str>) -> PathBuf {
    let Some(path_remapping) = path_remapping else { return filename };
    let Some((source, _dest)) = path_remapping.split_once(',') else { return filename };
    match filename.strip_prefix(source) {
        Ok(rest) => rest.to_path_buf(),
        Err(_) => filename,
    }
}

pub fn coverage(
    cmd: &ShowCommand,
    report: &CoverageReport,
    path_remapping: Option<&str>,
) -> Result<Vec<FunctionCoverage>> {
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
    for Function { qualified_name, filename, start_line, end_line, linkage_name: _ } in symbols {
        let annotations = annotations.get_mut(&filename);
        let span =
            Span { filename: remap_filename(filename.into(), path_remapping), start_line, end_line };
        coverage.push(super::get_coverage(
            report,
            span,
            &cmd.ferrocene,
            qualified_name,
            annotations,
        )?);
    }

    let mut saw_unused = false;
    for (filename, annotations) in annotations {
        for Annotation { start, end: _, unused } in annotations {
            if unused {
                eprintln!("Unused annotation in {filename}:{start}");
            }
            saw_unused |= unused;
        }
    }

    if saw_unused {
        bail!("saw 1 or more unused annotations");
    }

    Ok(coverage)
}
