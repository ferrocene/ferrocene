use std::path::Path;

use maud::{DOCTYPE, PreEscaped};

use crate::{FunctionCoverage, FunctionCoverageStatus, LineCoverageStatus};

const CSS: &str = include_str!("../assets/html_report.css");

pub(crate) fn generate(
    coverage: Vec<FunctionCoverage>,
    sources: &Path,
) -> std::io::Result<PreEscaped<std::string::String>> {
    let mut fragments = Vec::with_capacity(coverage.len());
    for function in &coverage {
        let fragment = generate_function(function, sources)?;
        fragments.push(fragment);
    }

    let mut count_fully_tested = 0;
    let mut count_partially_tested = 0;
    let mut count_fully_untested = 0;
    let mut count_fully_ignored = 0;
    for function in coverage {
        match function.status {
            FunctionCoverageStatus::FullyTested => count_fully_tested += 1,
            FunctionCoverageStatus::PartiallyTested => count_partially_tested += 1,
            FunctionCoverageStatus::FullyUntested => count_fully_untested += 1,
            FunctionCoverageStatus::FullyIngored => count_fully_ignored += 1,
        };
    }
    let summary = maud::html!(
        div class="summary" {
            (count_fully_tested) " Fully Tested, "
            (count_partially_tested) " Partially Tested, "
            (count_fully_untested) " Fully Untested, "
            (count_fully_ignored) " Fully Ignored"
        }
    );

    let html = maud::html!(
        (DOCTYPE)
        html {
            head {
                style {
                    (PreEscaped(CSS))
                }
            }
            body {
                (summary)
                @for fragment in fragments {
                    (fragment)
                }
            }
        }
    );
    Ok(html)
}

fn generate_function(
    function: &FunctionCoverage,
    sources: &Path,
) -> std::io::Result<PreEscaped<std::string::String>> {
    let line_coverage = &function.lines.lines;
    let source_path = sources.join(&function.filename);
    let file = std::fs::read_to_string(&source_path)?;

    let mut lines = Vec::with_capacity(line_coverage.len());
    for (linenum, line) in file.lines().enumerate() {
        let linenum = linenum + 1; // `enumerate()` starts at 0, lines start at 1.
        let maybe_line = line_coverage
            .iter()
            .find(|(covered_linenum, _)| linenum == *covered_linenum);
        if let Some((actual_linenum, status)) = maybe_line {
            lines.push((actual_linenum, line, status))
        }
    }

    let function_status = &function.status;

    let html = maud::html!(
        details class=(function_status.to_css_class()) {
            summary {
                (function.source_name)
            }
            code {
                pre {
                    @for (linenum, line, status) in lines {
                        @match status {
                            LineCoverageStatus::Tested => span class="tested" {
                                (linenum) "|" (line) "\n"
                            },
                            LineCoverageStatus::Untested => span class="untested" {
                                (linenum) "|" (line) "\n"
                            },
                            LineCoverageStatus::Ignored => {
                                (linenum) "|" (line) "\n"
                            },
                        }
                    }
                }
            }
        }
    );
    Ok(html)
}
