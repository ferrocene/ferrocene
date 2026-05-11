// Derived from https://github.com/xd009642/llvm-profparser/blob/f12a20d33b371f62a3b63f3a19d2320c25aa48b9/src/bin/cov.rs

use std::fmt;
use std::path::PathBuf;

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
use clap::{Parser};
use llvm_profparser::*;
use crate::cli::{Opts};

mod html_report;
mod rustc_driver;
mod cli;


#[derive(Debug, PartialEq)]
enum LineCoverageStatus {
    Tested,
    Untested,
    Annotated,
    Ignored,
}

impl fmt::Display for LineCoverageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineCoverageStatus::Tested => write!(f, "Tested"),
            LineCoverageStatus::Untested => write!(f, "Untested"),
            LineCoverageStatus::Annotated => write!(f, "Annotated"),
            LineCoverageStatus::Ignored => write!(f, "Ignored"),
        }
    }
}

#[derive(Debug)]
struct LineCoverage {
    lines: Vec<(usize, LineCoverageStatus)>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum FunctionCoverageStatus {
    FullyTested,
    PartiallyTested,
    FullyUntested,
    FullyIgnored,
}

impl FunctionCoverageStatus {
    fn new(lines: &LineCoverage) -> Self {
        match lines {
            // If all lines are ignored, the function is ignored.
            lines
                if lines.lines.iter().all(|(_, status)| *status == LineCoverageStatus::Ignored) =>
            {
                FunctionCoverageStatus::FullyIgnored
            }
            // If at least one line is covered and all other lines are either covered or ignored, the function is fully tested.
            lines
                if lines.lines.iter().all(|(_, status)| {
                    *status == LineCoverageStatus::Ignored || *status == LineCoverageStatus::Tested
                }) =>
            {
                FunctionCoverageStatus::FullyTested
            }
            // If at least one line is uncovered and all other lines are uncovered or ignored, the function is fully untested
            lines
                if lines.lines.iter().all(|(_, status)| {
                    *status == LineCoverageStatus::Untested
                        || *status == LineCoverageStatus::Annotated
                        || *status == LineCoverageStatus::Ignored
                }) =>
            {
                FunctionCoverageStatus::FullyUntested
            }
            // Otherwise, the function mixes uncovered and covered lines.
            _ => FunctionCoverageStatus::PartiallyTested,
        }
    }

    fn to_css_class(&self) -> &str {
        match self {
            FunctionCoverageStatus::FullyTested => "fully-tested",
            FunctionCoverageStatus::PartiallyTested => "partially-tested",
            FunctionCoverageStatus::FullyUntested => "fully-untested",
            FunctionCoverageStatus::FullyIgnored => "fully-ignored",
        }
    }

    fn to_human(&self) -> &str {
        match self {
            FunctionCoverageStatus::FullyTested => "Fully Tested",
            FunctionCoverageStatus::PartiallyTested => "Partially Tested",
            FunctionCoverageStatus::FullyUntested => "Fully Untested",
            FunctionCoverageStatus::FullyIgnored => "Fully Ignored",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Annotated {
    Fully,
    Partially,
    Not,
}

#[derive(Debug)]
struct FunctionCoverage {
    source_name: String,
    relative_path: PathBuf,
    lines: LineCoverage,
    status: FunctionCoverageStatus,
    annotated: Annotated,
}

impl FunctionCoverage {
    fn new(
        source_name: String,
        filename: PathBuf,
        lines: LineCoverage,
        annotated: Annotated,
    ) -> Self {
        let status = FunctionCoverageStatus::new(&lines);
        Self { source_name, relative_path: filename, lines, status, annotated }
    }
}

struct Span {
    filename: PathBuf,
    start_line: usize,
    end_line: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Annotation {
    start: usize,
    end: usize,
    unused: bool,
}

fn get_annotation_status(
    annotations: Option<&mut Vec<Annotation>>,
    lines: &mut Vec<(usize, LineCoverageStatus)>,
) -> Annotated {
    annotations.map_or(Annotated::Not, |annotations| {
        let mut untested_count = 0;
        let mut annotated_count = 0;

        for (line, status) in lines {
            if *status != LineCoverageStatus::Untested {
                continue;
            }
            untested_count += 1;

            for Annotation { start, end, unused } in annotations.iter_mut() {
                if (*start..=*end).contains(line) {
                    *unused = false;
                    *status = LineCoverageStatus::Annotated;
                    annotated_count += 1;

                    break;
                }
            }
        }

        if annotated_count == 0 {
            // If no lines are annotated, the function is not annotated.
            Annotated::Not
        } else if annotated_count < untested_count {
            // If there are less annotated lines than untested lines, the function is partially
            // annotated.
            Annotated::Partially
        } else {
            // Otherwise the function is fully annotated
            Annotated::Fully
        }
    })
}

fn get_coverage(
    report: &CoverageReport,
    span: Span,
    ferrocene: &std::path::Path,
    source_name: String,
    annotations: Option<&mut Vec<Annotation>>,
) -> Result<FunctionCoverage> {
    let Span { filename, start_line, end_line } = span;
    let absolute_path = if filename.is_relative() {
        ferrocene
            .join(&filename)
            .canonicalize()
            .context(format!("failed to canonicalize {filename:?}"))?
    } else {
        return Err(anyhow::anyhow!("Absolute paths are forbidden, they break reproducability."));
    };

    let source_lines = start_line..=end_line;
    let source_name = source_name;

    // we didn't get any hits from the tool, so we don't know which lines shouldn't be
    // considered. report them all as considered and missing coverage.
    let Some(func_coverage) = report.files.get(&absolute_path) else {
        let mut no_coverage = LineCoverage {
            lines: source_lines.clone().map(|i| (i, LineCoverageStatus::Untested)).collect(),
        };
        println!(
            "warning: couldn't find source file {} in coverage report",
            absolute_path.display()
        );

        // All lines require annotations as we didn't get any hits from the tool.
        let annotated = get_annotation_status(annotations, &mut no_coverage.lines);

        return Ok(FunctionCoverage::new(source_name, filename, no_coverage, annotated));
    };

    let mut covered = vec![];

    for line in source_lines {
        // one more thing to do: within a function, some lines will always be uncovered (e.g. }
        // closing braces). so we do have to trust the coverage tool to report those accurately.
        let status = match func_coverage.hits_for_line(line) {
            None => LineCoverageStatus::Ignored,
            Some(0) => LineCoverageStatus::Untested,
            Some(_) => LineCoverageStatus::Tested,
        };
        covered.push((line, status));
    }

    let annotated = get_annotation_status(annotations, &mut covered);

    Ok(FunctionCoverage::new(source_name, filename, LineCoverage { lines: covered }, annotated))
}



fn main() -> Result<()> {
    Opts::parse().run()
}
