use std::collections::HashSet;
use std::path::Path;

use maud::{DOCTYPE, PreEscaped};

use crate::{Annotated, ColumnRange, FunctionCoverage, FunctionCoverageStatus, LineCoverageStatus};

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

    let mut num_lines_tested: f64 = 0.0;
    let mut num_lines_untested: f64 = 0.0;
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

        for (_, status) in &function.lines.lines {
            match status {
                LineCoverageStatus::Tested => num_lines_tested += 1.0,
                LineCoverageStatus::Untested | LineCoverageStatus::Annotated => {
                    num_lines_untested += 1.0
                }
                LineCoverageStatus::Ignored => (),
            }
        }
    }
    assert_eq!(
        fully_tested.len() + partially_tested.len() + fully_untested.len() + fully_ignored.len(),
        coverage.len()
    );

    let fully_tested_class = FunctionCoverageStatus::FullyTested.to_css_class();
    let partially_tested_class = FunctionCoverageStatus::PartiallyTested.to_css_class();
    let fully_untested_class = FunctionCoverageStatus::FullyUntested.to_css_class();
    let fully_ignored_class = FunctionCoverageStatus::FullyIgnored.to_css_class();

    let total_lines = num_lines_tested + num_lines_untested;
    let percentile_lines_tested = (num_lines_tested / (total_lines)) * 100.0;
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
                (format!("{percentile_lines_tested:.2}% ({num_lines_tested}/{total_lines} lines)"))
            }
        }
        div class="instructions" {
            "Below is a list of all functions within the certified subset. Use the expander to review line coverage of any function."
            br {}
            "To filter for specific coverage status, select below:"
        }
        div class="picker-buttons" {
                button class=(fully_tested_class) data-filter=(fully_tested_class) {
                    span class="count" { (fully_tested.len()) } " Fully Tested"
                }
                button class=(partially_tested_class) data-filter=(partially_tested_class) {
                    span class="count" { (partially_tested.len()) } " Partially Tested"
                }
                button class=(fully_untested_class) data-filter=(fully_untested_class) {
                    span class="count" { (fully_untested.len()) } " Fully Untested"
                }
                button class=(fully_ignored_class) data-filter=(fully_ignored_class) {
                    span class="count" { (fully_ignored.len()) } " Fully Ignored"
                }
        }
        div class="misc-checkboxes" {
                input type="checkbox" name="annotated-checkbox" unchecked;
                "count annotated lines as tested"
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
            h1 { span class="count" { (functions.len()) } " " (human) }
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
                                    @for (chunk, tested) in line_runs(line, function.regions.get(linenum)) {
                                        @if tested {
                                            span class="region-tested" { (chunk) }
                                        } @else {
                                            (chunk)
                                        }
                                    }
                                    "\n"
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

fn line_runs(line: &str, regions: Option<&Vec<ColumnRange>>) -> Vec<(String, bool)> {
    let len = line.len();
    let Some(regions) = regions.filter(|regions| !regions.is_empty() && len > 0) else {
        return vec![(line.to_string(), false)];
    };

    let mut tested = vec![false; len];

    let mut by_specificity: Vec<&ColumnRange> = regions.iter().collect();
    by_specificity.sort_by_key(|range| {
        std::cmp::Reverse(range.end_col.unwrap_or(len).saturating_sub(range.start_col))
    });

    for range in by_specificity {
        let start = floor_char_boundary(line, range.start_col.saturating_sub(1).min(len));
        let end = ceil_char_boundary(line, range.end_col.unwrap_or(len).min(len)).max(start);
        tested[start..end].fill(range.tested);
    }

    let mut offset = 0;
    tested
        .chunk_by(|a, b| a == b)
        .map(|chunk| {
            let start = offset;
            offset += chunk.len();
            (line[start..offset].to_string(), chunk[0])
        })
        .collect()
}

fn floor_char_boundary(line: &str, index: usize) -> usize {
    (0..=index).rev().find(|&i| line.is_char_boundary(i)).unwrap_or(0)
}

fn ceil_char_boundary(line: &str, index: usize) -> usize {
    (index..=line.len()).find(|&i| line.is_char_boundary(i)).unwrap_or(line.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(start_col: usize, end_col: Option<usize>, tested: bool) -> ColumnRange {
        ColumnRange { start_col, end_col, tested }
    }

    #[test]
    fn no_regions_is_one_untested_run() {
        let runs = line_runs("    baz();", None);
        assert_eq!(runs, vec![("    baz();".to_string(), false)]);
    }

    #[test]
    fn single_region_covering_whole_line() {
        let regions = vec![range(1, None, true)];
        let runs = line_runs("    baz();", Some(&regions));
        assert_eq!(runs, vec![("    baz();".to_string(), true)]);
    }

    #[test]
    fn splits_covered_arm_from_uncovered_arm() {
        // `if cond { covered() } else { uncovered() }`
        let line = "if cond { covered() } else { uncovered() }";
        let regions = vec![
            range(1, Some(9), true),    // `if cond `
            range(10, Some(21), true),  // `{ covered() }`
            range(22, Some(27), true),  // ` else `
            range(28, Some(42), false), // `{ uncovered() }`
        ];
        let runs = line_runs(line, Some(&regions));
        assert_eq!(
            runs,
            vec![
                ("if cond { covered() } else ".to_string(), true),
                ("{ uncovered() }".to_string(), false),
            ]
        );
    }

    #[test]
    fn narrower_region_overrides_broader_untested_one() {
        // The whole line is nominally untested, but a nested, more specific region within it
        // was hit (e.g. a branch condition that runs even though the statement as a whole
        // never completes).
        let line = "    a && b";
        let regions = vec![
            range(1, None, false),   // whole line, untested
            range(5, Some(5), true), // `a`
        ];
        let runs = line_runs(line, Some(&regions));
        assert_eq!(
            runs,
            vec![
                ("    ".to_string(), false),
                ("a".to_string(), true),
                (" && b".to_string(), false),
            ]
        );
    }

    #[test]
    fn out_of_range_columns_are_clamped_not_panicking() {
        let regions = vec![range(1, Some(9999), true)];
        let runs = line_runs("short", Some(&regions));
        assert_eq!(runs, vec![("short".to_string(), true)]);
    }

    #[test]
    fn generate_function_highlights_covered_fragment_of_untested_line() {
        let dir = std::env::temp_dir().join(format!("blanket-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let relative_path = std::path::PathBuf::from("lib.rs");
        std::fs::write(
            dir.join(&relative_path),
            "fn f(cond: bool) {\n    if cond { covered() } else { uncovered() }\n}\n",
        )
        .unwrap();

        let mut regions = std::collections::BTreeMap::new();
        regions.insert(
            2,
            vec![
                range(5, Some(13), true),   // `if cond `
                range(14, Some(25), true),  // `{ covered() }`
                range(26, Some(31), true),  // ` else `
                range(32, Some(46), false), // `{ uncovered() }`
            ],
        );

        let lines = crate::LineCoverage {
            lines: vec![
                (1, LineCoverageStatus::Ignored),
                (2, LineCoverageStatus::Untested),
                (3, LineCoverageStatus::Ignored),
            ],
        };
        let function =
            FunctionCoverage::new("f".to_string(), relative_path, lines, regions, Annotated::Not);

        let html = generate_function(&function, &dir).unwrap().into_string();
        std::fs::remove_dir_all(&dir).ok();

        assert!(html.contains(r#"<span class="region-tested">if cond { covered() } else </span>"#));
        assert!(html.contains("{ uncovered() }"));
        assert!(!html.contains(r#"<span class="region-tested">{ uncovered() }</span>"#));
    }
}
