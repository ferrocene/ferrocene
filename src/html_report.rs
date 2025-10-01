use std::path::Path;

use maud::{DOCTYPE, PreEscaped};

use crate::{FunctionCoverage, FunctionCoverageStatus, LineCoverageStatus};

const CSS: &str = include_str!("../assets/html_report.css");
const JS: &str = include_str!("../assets/html_report.js");

pub(crate) fn generate(
    coverage: &[FunctionCoverage],
    sources: &Path,
) -> std::io::Result<PreEscaped<std::string::String>> {
    let mut functions = Vec::with_capacity(coverage.len());
    for function in coverage {
        let fragment = generate_function(function, sources)?;
        functions.push(fragment);
    }

    let mut fully_tested = vec![];
    let mut partially_tested = vec![];
    let mut fully_untested = vec![];
    let mut fully_ignored = vec![];
    for function in coverage {
        match function.status {
            FunctionCoverageStatus::FullyTested => fully_tested.push(function),
            FunctionCoverageStatus::PartiallyTested => partially_tested.push(function),
            FunctionCoverageStatus::FullyUntested => fully_untested.push(function),
            FunctionCoverageStatus::FullyIgnored => fully_ignored.push(function),
        };
    }
    assert_eq!(fully_tested.len() + partially_tested.len() + fully_untested.len() + fully_ignored.len(), coverage.len());

    let fully_tested_class = FunctionCoverageStatus::FullyTested.to_css_class();
    let partially_tested_class = FunctionCoverageStatus::PartiallyTested.to_css_class();
    let fully_untested_class = FunctionCoverageStatus::FullyUntested.to_css_class();
    let fully_ignored_class = FunctionCoverageStatus::FullyIgnored.to_css_class();

    let summary = maud::html!(
        div class="instructions" {
            "Below is a list of all functions within the certified subset. Use the expander to review line coverage of any function."
            br {}
            "To filter for specific coverage status, select below:"
        }
        div class="summary" {
            button class=(fully_tested_class) data-filter=(fully_tested_class) {
                (fully_tested.len()) " Fully Tested"
            }
            button class=(partially_tested_class) data-filter=(partially_tested_class) {
                (partially_tested.len()) " Partially Tested"
            }
            button class=(fully_untested_class) data-filter=(fully_untested_class) {
                (fully_untested.len()) " Fully Untested"
            }
            button class=(fully_ignored_class) data-filter=(fully_ignored_class) {
                (fully_ignored.len()) " Fully Ignored"
            }
        }
    );

    let sections = [
        generate_section(FunctionCoverageStatus::FullyUntested, fully_untested, sources)?,
        generate_section(FunctionCoverageStatus::PartiallyTested, partially_tested, sources)?,
        generate_section(FunctionCoverageStatus::FullyTested, fully_tested, sources)?,
        generate_section(FunctionCoverageStatus::FullyIgnored, fully_ignored, sources)?,
    ];


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
                div class="functions" {
                    @for section in sections {
                        (section)
                    }
                }
                script defer=(true) {
                    (PreEscaped(JS))
                }
            }
        }
    );
    Ok(html)
}

fn generate_section(
    status: FunctionCoverageStatus,
    functions: Vec<&FunctionCoverage>,
    sources: &Path,
) -> std::io::Result<PreEscaped<std::string::String>> {
    let mut fragments = Vec::with_capacity(functions.len());
    for function in functions {
        assert_eq!(function.status, status);
        let fragment = generate_function(function, sources)?;
        fragments.push(fragment);
    }

    let class = status.to_css_class();
    let human = status.to_human();
    let section = maud::html!(
        section class=(class) data-status=(class)  {
            h1 { (human) }
            div class="list" {
                @for fragment in fragments {
                    (fragment)
                }
            }
        }
    );
    Ok(section)
}

fn generate_function(
    function: &FunctionCoverage,
    sources: &Path,
) -> std::io::Result<PreEscaped<std::string::String>> {
    let line_coverage = &function.lines.lines;
    let source_path = sources.join(&function.relative_path);
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
    let function_css_class = function_status.to_css_class();

    let html = maud::html!(
        details class=(function_css_class) data-status=(function_css_class) {
            summary {
                (function.source_name)
            }
            div {
                div {
                    "File: " (function.relative_path.display())
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
        }
    );
    Ok(html)
}
