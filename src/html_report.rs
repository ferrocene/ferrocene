use std::{collections::HashSet, fs, path::Path};

use maud::{DOCTYPE, Markup, PreEscaped, Render};

use crate::{CoverageStatus, FunctionCoverage};

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

    let html = maud::html!(
        (DOCTYPE)
        html {
            head {
                style {
                    (PreEscaped(CSS))
                }
            }
            body {
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

    let function_status = if lines
        .iter()
        .all(|(_, _, status)| **status == CoverageStatus::Ignored)
    {
        // This is the bad place, the function was in the subset but entirely ignored.
        "fully-ignored"
    } else if lines
        .iter()
        .all(|(_, _, status)| **status != CoverageStatus::Tested)
    {
        // The function is completely untested
        "fully-untested"
    } else if lines
        .iter()
        .all(|(_, _, status)| **status != CoverageStatus::Untested)
    {
        // The function is completely tested
        "fully-tested"
    } else {
        // The function is mixed
        "partially-tested"
    };

    let html = maud::html!(
        details class=(function_status) {
            summary {
                (function.source_name)
            }
            code {
                pre {
                    @for (linenum, line, status) in lines {
                        @match status {
                            CoverageStatus::Tested => span class="tested" {
                                (linenum) "|" (line) "\n"
                            },
                            CoverageStatus::Untested => span class="untested" {
                                (linenum) "|" (line) "\n"
                            },
                            CoverageStatus::Ignored => {
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
