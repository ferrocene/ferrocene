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
                                    @for segment in line_segments(line, function.regions.get(linenum)) {
                                        @match segment {
                                            LineSegment::Text(text, _) => (text),
                                            LineSegment::Branch { text, true_count, false_count, .. } => {
                                                span class="branch-group" {
                                                    span class="branch-text" { (text) }
                                                    (branch_marker_markup(true_count, false_count))
                                                }
                                                " "
                                            },
                                        }
                                    }
                                    "\n"
                                },
                                LineCoverageStatus::Untested => span class="line line-untested" data-filename=(filename) data-linenum=(linenum) {
                                    @for segment in line_segments(line, function.regions.get(linenum)) {
                                        @match segment {
                                            LineSegment::Text(chunk, tested) => {
                                                @let (leading, content) = {
                                                    let content = chunk.trim_start();
                                                    let leading_len = chunk.len() - content.len();
                                                    chunk.split_at(leading_len)
                                                };
                                                (leading)
                                                @if tested {
                                                    span class="region-tested" { (content) }
                                                } @else {
                                                    span class="region-untested" { (content) }
                                                }
                                            },
                                            LineSegment::Branch { text, tested, true_count, false_count } => {
                                                @let (leading, content) = {
                                                    let content = text.trim_start();
                                                    let leading_len = text.len() - content.len();
                                                    text.split_at(leading_len)
                                                };
                                                (leading)
                                                span class="branch-group" {
                                                    @if tested {
                                                        span class="branch-text region-tested" { (content) }
                                                    } @else {
                                                        span class="branch-text region-untested" { (content) }
                                                    }
                                                    (branch_marker_markup(true_count, false_count))
                                                }
                                                " "
                                            },
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

/// Per-byte coverage flag for every byte in `line`
fn tested_mask(line: &str, regions: &[ColumnRange]) -> Vec<bool> {
    let len = line.len();
    let mut tested = vec![false; len];

    // Wider regions first, narrower regions last
    // So if a wider region has hits but a smaller region within it is not hit
    // we show it as untested properly and not blindly assume that we cover it.
    let mut by_specificity: Vec<&ColumnRange> = regions.iter().collect();
    by_specificity.sort_by_key(|range| {
        std::cmp::Reverse(range.end_col.unwrap_or(len).saturating_sub(range.start_col))
    });

    for range in by_specificity {
        let start = floor_char_boundary(line, range.start_col.saturating_sub(1).min(len));
        let end = ceil_char_boundary(line, range.end_col.unwrap_or(len).min(len)).max(start);
        tested[start..end].fill(range.tested);
    }

    tested
}

fn line_runs(line: &str, regions: Option<&Vec<ColumnRange>>) -> Vec<(String, bool)> {
    let len = line.len();
    let Some(regions) = regions.filter(|regions| !regions.is_empty() && len > 0) else {
        return vec![(line.to_string(), false)];
    };

    chunk_runs(&tested_mask(line, regions), line, 0)
}

/// Groups consecutive tested / untested bytes into `(text, tested)` runs.
fn chunk_runs(mask: &[bool], line: &str, offset: usize) -> Vec<(String, bool)> {
    let mut local_offset = 0;
    mask.chunk_by(|a, b| a == b)
        .map(|chunk| {
            let start = offset + local_offset;
            local_offset += chunk.len();
            (line[start..offset + local_offset].to_string(), chunk[0])
        })
        .collect()
}

fn floor_char_boundary(line: &str, index: usize) -> usize {
    (0..=index).rev().find(|&i| line.is_char_boundary(i)).unwrap_or(0)
}

fn ceil_char_boundary(line: &str, index: usize) -> usize {
    (index..=line.len()).find(|&i| line.is_char_boundary(i)).unwrap_or(line.len())
}

/// A piece of a rendered line
#[derive(Debug, PartialEq)]
enum LineSegment {
    Text(String, bool),
    Branch { text: String, tested: bool, true_count: i64, false_count: i64 },
}

struct BranchSpan {
    start: usize,
    end: usize,
    tested: bool,
    true_count: i64,
    false_count: i64,
}

fn branch_spans_for_line(line: &str, regions: &[ColumnRange]) -> Vec<BranchSpan> {
    let len = line.len();
    let mut spans: Vec<BranchSpan> = regions
        .iter()
        .filter_map(|range| {
            let info = range.regions.iter().find(|info| info.false_count.is_some())?;
            let start = floor_char_boundary(line, range.start_col.saturating_sub(1).min(len));
            let end = ceil_char_boundary(line, range.end_col.unwrap_or(len).min(len)).max(start);
            Some(BranchSpan {
                start,
                end,
                tested: range.tested,
                true_count: info.true_count,
                false_count: info.false_count.unwrap(),
            })
        })
        .collect();
    spans.sort_by_key(|span| span.start);

    // overlapping should not happen, but if it does we drop the overlap
    let mut non_overlapping = Vec::with_capacity(spans.len());
    let mut end_so_far = 0;
    for span in spans.drain(..) {
        if span.start < end_so_far {
            continue;
        }
        end_so_far = span.end;
        non_overlapping.push(span);
    }
    non_overlapping
}

/// Splits a line into text segments
fn line_segments(line: &str, regions: Option<&Vec<ColumnRange>>) -> Vec<LineSegment> {
    let len = line.len();
    let Some(regions) = regions.filter(|regions| !regions.is_empty() && len > 0) else {
        return vec![LineSegment::Text(line.to_string(), false)];
    };

    let branch_spans = branch_spans_for_line(line, regions);
    if branch_spans.is_empty() {
        return line_runs(line, Some(regions))
            .into_iter()
            .map(|(text, tested)| LineSegment::Text(text, tested))
            .collect();
    }

    let mask = tested_mask(line, regions);
    let mut result = Vec::new();
    let mut pos = 0;
    for span in &branch_spans {
        if span.start > pos {
            for (text, tested) in chunk_runs(&mask[pos..span.start], line, pos) {
                result.push(LineSegment::Text(text, tested));
            }
        }
        result.push(LineSegment::Branch {
            text: line[span.start..span.end].to_string(),
            tested: span.tested,
            true_count: span.true_count,
            false_count: span.false_count,
        });
        pos = span.end;
    }
    if pos < len {
        for (text, tested) in chunk_runs(&mask[pos..len], line, pos) {
            result.push(LineSegment::Text(text, tested));
        }
    }

    // There should be a space after `T F`
    let mut i = 0;
    while i < result.len() {
        if matches!(result[i], LineSegment::Branch { .. })
            && let Some(LineSegment::Text(text, _)) = result.get_mut(i + 1)
        {
            let trimmed = text.trim_start();
            if trimmed.is_empty() {
                result.remove(i + 1);
            } else if trimmed.len() != text.len() {
                *text = trimmed.to_string();
            }
        }
        i += 1;
    }

    result
}

fn branch_marker_markup(true_count: i64, false_count: i64) -> maud::Markup {
    let true_class =
        if true_count > 0 { "branch-marker-true tested" } else { "branch-marker-true untested" };
    let false_class =
        if false_count > 0 { "branch-marker-false tested" } else { "branch-marker-false untested" };
    maud::html!(
        span class="branch-marker" {
            span class=(true_class) title=(format!("true: {true_count}")) {}
            span class=(false_class) title=(format!("false: {false_count}")) {}
        }
    )
}

#[cfg(test)]
mod tests {
    use llvm_profparser::RegionKind;

    use super::*;
    use crate::RegionInfo;

    fn range(start_col: usize, end_col: Option<usize>, tested: bool) -> ColumnRange {
        ColumnRange {
            start_col,
            end_col,
            tested,
            regions: vec![RegionInfo {
                kind: RegionKind::Code,
                true_count: tested as i64,
                false_count: None,
            }],
        }
    }

    fn branch_range(
        start_col: usize,
        end_col: Option<usize>,
        true_count: i64,
        false_count: i64,
    ) -> ColumnRange {
        ColumnRange {
            start_col,
            end_col,
            tested: true_count > 0,
            regions: vec![RegionInfo {
                kind: RegionKind::Branch,
                true_count,
                false_count: Some(false_count),
            }],
        }
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
    fn generate_function_highlights_uncovered_fragment_of_untested_line() {
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

        assert!(html.contains("if cond { covered() } else "));
        assert!(html.contains(r#"<span class="region-untested">{ uncovered() }</span>"#));
        assert!(
            !html.contains(r#"<span class="region-untested">if cond { covered() } else </span>"#)
        );
    }

    #[test]
    fn branch_spans_cover_each_operand() {
        let line = "a && b";
        let regions = vec![
            branch_range(1, Some(1), 5, 3), // `a`
            branch_range(6, Some(6), 5, 0), // `b`
        ];
        let spans = branch_spans_for_line(line, &regions);
        let spans: Vec<_> =
            spans.iter().map(|s| (s.start, s.end, s.true_count, s.false_count)).collect();
        assert_eq!(spans, vec![(0, 1, 5, 3), (5, 6, 5, 0)]);
    }

    #[test]
    fn non_branch_regions_produce_no_spans() {
        let regions = vec![range(1, None, true)];
        assert_eq!(branch_spans_for_line("a && b", &regions).len(), 0);
    }

    #[test]
    fn line_segments_carves_out_branch_operands_from_tested_run() {
        let line = "a && b";
        let regions = vec![
            range(1, None, true),           // whole line, tested (Code region)
            branch_range(1, Some(1), 5, 3), // `a`
            branch_range(6, Some(6), 5, 0), // `b`
        ];
        let segments = line_segments(line, Some(&regions));
        assert_eq!(
            segments,
            vec![
                LineSegment::Branch {
                    text: "a".to_string(),
                    tested: true,
                    true_count: 5,
                    false_count: 3
                },
                LineSegment::Text("&& ".to_string(), true),
                LineSegment::Branch {
                    text: "b".to_string(),
                    tested: true,
                    true_count: 5,
                    false_count: 0
                },
            ]
        );
    }

    #[test]
    fn line_segments_without_markers_matches_line_runs() {
        let line = "    baz();";
        assert_eq!(line_segments(line, None), vec![LineSegment::Text(line.to_string(), false)]);
    }

    #[test]
    fn generate_function_shows_branch_marker_on_fully_tested_line() {
        let dir = std::env::temp_dir().join(format!("blanket-test-branch-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let relative_path = std::path::PathBuf::from("lib.rs");
        std::fs::write(
            dir.join(&relative_path),
            "fn f(cond: bool) -> bool {\n    cond && other()\n}\n",
        )
        .unwrap();

        let mut regions = std::collections::BTreeMap::new();
        regions.insert(
            2,
            vec![
                range(1, None, true),           // whole line, tested
                branch_range(5, Some(8), 3, 0), // `cond`, true arm hit, false arm never hit
            ],
        );

        let lines = crate::LineCoverage {
            lines: vec![
                (1, LineCoverageStatus::Ignored),
                (2, LineCoverageStatus::Tested),
                (3, LineCoverageStatus::Ignored),
            ],
        };
        let function =
            FunctionCoverage::new("f".to_string(), relative_path, lines, regions, Annotated::Not);

        let html = generate_function(&function, &dir).unwrap().into_string();
        std::fs::remove_dir_all(&dir).ok();

        assert!(
            html.contains(r#"<span class="branch-marker-true tested" title="true: 3"></span>"#)
        );
        assert!(
            html.contains(r#"<span class="branch-marker-false untested" title="false: 0"></span>"#)
        );
        // The branch's own text is rendered inside the `branch-group`,
        // followed by its marker with no gap, and exactly one space afterwards
        // before the next token (`&&` is HTML-escaped to `&amp;&amp;`). The T/F letters
        // themselves are drawn by CSS `::before` content, not real text, so a user copying
        // this line's text never picks them up.
        assert!(html.contains(r#"<span class="branch-text">cond</span>"#));
        assert!(html.contains("</span></span> &amp;&amp;"));
    }
}
