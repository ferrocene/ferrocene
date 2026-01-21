use std::collections::HashSet;
use std::path::Path;

use maud::{DOCTYPE, PreEscaped};

use crate::{Annotated, FunctionCoverage, FunctionCoverageStatus, LineCoverageStatus};

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

    let fully_tested_class = FunctionCoverageStatus::FullyTested.to_css_class();
    let partially_tested_class = FunctionCoverageStatus::PartiallyTested.to_css_class();
    let fully_untested_class = FunctionCoverageStatus::FullyUntested.to_css_class();
    let fully_ignored_class = FunctionCoverageStatus::FullyIgnored.to_css_class();

    let summary = maud::html!(
        header {
            div class="header-title" {
                h1 { "Core library line coverage report" }
                a href="../index.html" { "Go back to the documentation index" }
            }
            div class="search-bar" {
                    input type="text" name = "search-bar" placeholder = "Regex Search ...";
                    " "
                    button name="search-button" { "Search" }
            }
        }
        div class="coverage-summary" {
            h1 {
                "Unknown"
            }
        }
        div class="instructions" {
            "Below is a list of all functions within the certified subset. Use the expander to review line coverage of any function."
            br {}
            "To filter for specific coverage status, select below:"
        }
        div class="picker-buttons" {
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
        div class="misc-checkboxes" {
                input type="checkbox" name="annotated-checkbox" unchecked;
                "Line-through annotated functions"
        }
    );

    let sections = [
        generate_section(FunctionCoverageStatus::PartiallyTested, partially_tested, sources)?,
        generate_section(FunctionCoverageStatus::FullyUntested, fully_untested, sources)?,
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
    for function in &functions {
        assert_eq!(function.status, status);
        let fragment = generate_function(function, sources)?;
        fragments.push(fragment);
    }

    let class = status.to_css_class();
    let human = status.to_human();
    let section = maud::html!(
        section class=(class) data-status=(class)  {
            h1 { span class="count" { "" } " " (human) }
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

    let mut class_set = HashSet::new();
    let function_css_class = function.status.to_css_class();

    class_set.insert(function_css_class);

    let mut lines = Vec::with_capacity(line_coverage.len());
    let mut tested_lines = 0;
    let mut untested_lines = 0;
    let mut annotated_lines = 0;
    let mut ignored_lines = 0;
    for (linenum, line) in file.lines().enumerate() {
        let linenum = linenum + 1; // `enumerate()` starts at 0, lines start at 1.
        let maybe_line =
            line_coverage.iter().find(|(covered_linenum, _)| linenum == *covered_linenum);
        if let Some((actual_linenum, status)) = maybe_line {
            lines.push((actual_linenum, line, status));
            match status {
                LineCoverageStatus::Tested => tested_lines += 1,
                LineCoverageStatus::Untested => untested_lines += 1,
                LineCoverageStatus::Annotated => annotated_lines += 1,
                LineCoverageStatus::Ignored => ignored_lines += 1,
            }
        }
    }

    match function.annotated {
        Annotated::Fully => {
            class_set.insert("annotation");
        }
        Annotated::Partially => {
            class_set.insert("partial-annotation");
        }
        _ => (),
    }

    let filename = function.relative_path.display();
    let html = maud::html!(
        details class=(class_set.into_iter().collect::<Vec<_>>().join(" ")) data-status=(function_css_class) {
            summary tested-lines=(tested_lines) untested-lines=(untested_lines) annotated-lines=(annotated_lines) ignored-lines=(ignored_lines) {
                code {
                    (function.source_name)
                }
            }
            div {
                div {
                    "File: " (filename)
                }
                code {
                    pre {
                        @for (linenum, line, status) in lines {
                            @match status {
                                LineCoverageStatus::Tested => span class="line line-tested" data-filename=(filename) data-linenum=(linenum) {
                                    (line) "\n"
                                },
                                LineCoverageStatus::Untested => span class="line line-untested" data-filename=(filename) data-linenum=(linenum) {
                                    (line) "\n"
                                },
                                LineCoverageStatus::Annotated => span class="line line-annotated" data-filename=(filename) data-linenum=(linenum) {
                                    (line) "\n"
                                },
                                LineCoverageStatus::Ignored => span class="line line-ignored" data-filename=(filename) data-linenum=(linenum) {
                                    (line) "\n"
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
