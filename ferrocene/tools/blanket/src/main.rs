// Derived from https://github.com/xd009642/llvm-profparser/blob/f12a20d33b371f62a3b63f3a19d2320c25aa48b9/src/bin/cov.rs

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use llvm_profparser::instrumentation_profile::types::InstrumentationProfile;
use llvm_profparser::*;
use maud::Render;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

mod html_report;
mod rustc_driver;

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct Opts {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Eq, PartialEq, Subcommand)]
pub enum Command {
    Show {
        #[command(flatten)]
        show: ShowCommand,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct ShowCommand {
    /// File with the profile data obtained after an instrumented run. This differs from llvm-cov
    /// in that if multiple profiles are given it will do the equivalent of a llvm-profdata merge
    /// on them.
    #[arg(long = "instr-profile", short = 'p')]
    instr_profile: Vec<PathBuf>,
    /// Coverage executable or object file
    #[arg(long = "object", short = 'o')]
    objects: Vec<PathBuf>,
    /// Pair of paths for a remapping to allow loading files after move. Comma separated in the
    /// order `source,dest`
    #[arg(long = "path-equivalence")]
    path_remapping: Option<String>,
    // #[structopt(long = "rustdoc-json", short = "j")]
    // rustdoc_json: PathBuf,
    #[arg(long = "report", short = 'r')]
    symbol_report: PathBuf,
    #[arg(long = "ferrocene-src", short = 's')]
    ferrocene: PathBuf,
    /// Turn on debug logging
    #[arg(long)]
    debug: bool,
    /// Produce a HTML report
    #[arg(long)]
    html_out: Option<PathBuf>,
}

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

#[derive(Debug, Clone)]
struct ColumnRange {
    start_col: usize,
    /// `None` means the region continues past the end of this line.
    end_col: Option<usize>,
    tested: bool,
    /// Every individual region covering this exact span. There can be more than one: e.g. a
    /// `Branch` region and the `MCDCDecision` region it's part of commonly share the same source
    /// span, and each has its own independent counter(s).
    regions: Vec<RegionInfo>,
}

#[derive(Debug, Clone, Copy)]
struct RegionInfo {
    kind: RegionKind,
    true_count: i64,
    /// Only meaningful for `RegionKind::Branch` and `RegionKind::MCDCBranch`.
    false_count: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum FunctionCoverageStatus {
    FullyTested,
    PartiallyTested,
    FullyUntested,
    FullyIgnored,
}

impl FunctionCoverageStatus {
    fn new(lines: &LineCoverage, regions: &BTreeMap<usize, Vec<ColumnRange>>) -> Self {
        let status = match lines {
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
        };

        // Branch coverage check
        if status == FunctionCoverageStatus::FullyTested && has_untested_branch(regions) {
            FunctionCoverageStatus::PartiallyTested
        } else {
            status
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct BranchCoverage {
    tested: usize,
    untested: usize,
    annotated: usize,
}

#[derive(Debug)]
struct FunctionCoverage {
    source_name: String,
    relative_path: PathBuf,
    lines: LineCoverage,
    // filled in only for partially covered lines
    regions: BTreeMap<usize, Vec<ColumnRange>>,
    status: FunctionCoverageStatus,
    annotated: Annotated,
    branches: BranchCoverage,
}

impl FunctionCoverage {
    fn new(
        source_name: String,
        filename: PathBuf,
        lines: LineCoverage,
        regions: BTreeMap<usize, Vec<ColumnRange>>,
        annotated: Annotated,
        branches: BranchCoverage,
    ) -> Self {
        let status = FunctionCoverageStatus::new(&lines, &regions);
        Self { source_name, relative_path: filename, lines, regions, status, annotated, branches }
    }
}

/// Whether any branch region covering these lines has an arm
/// that was never taken. For example:
///
/// ```ignore
/// fn maybe_add(mut x: u32) -> u32 {
///     if x > 0 {
///         x += 1
///     }
///     x
/// }
///
/// #[test]
/// fn test_maybe_add() {
///     assert_eq!(maybe_add(1), 2)
/// }
/// ```
///
/// Line coverage report will show all lines covered even though the case
/// when `x = 0` is not tested.
fn has_untested_branch(regions: &BTreeMap<usize, Vec<ColumnRange>>) -> bool {
    regions.values().flatten().any(|range| {
        range.regions.iter().any(|info| {
            matches!(info.kind, RegionKind::Branch | RegionKind::MCDCBranch)
                && (info.true_count == 0 || info.false_count.unwrap_or(0) == 0)
        })
    })
}

/// Counts branch arms (true/false, counted separately) within `lines`, split
/// into `(tested, untested, annotated)`.
/// An untested arm counts as `annotated` instead of `untested` when it sits
/// on a line in `annotated_lines`.
fn count_branches(
    file_region_infos: &BTreeMap<SourceLocation, Vec<RegionInfo>>,
    lines: std::ops::RangeInclusive<usize>,
    annotated_lines: &HashSet<usize>,
) -> BranchCoverage {
    let mut branches = BranchCoverage::default();
    for (loc, infos) in file_region_infos {
        if !lines.contains(&loc.line_start) {
            continue;
        }
        for info in infos {
            if matches!(info.kind, RegionKind::Branch | RegionKind::MCDCBranch) {
                for arm_hit in [info.true_count > 0, info.false_count.unwrap_or(0) > 0] {
                    if arm_hit {
                        branches.tested += 1;
                    } else if annotated_lines.contains(&loc.line_start) {
                        branches.annotated += 1;
                    } else {
                        branches.untested += 1;
                    }
                }
            }
        }
    }
    branches
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
    region_infos: &BTreeMap<PathBuf, BTreeMap<SourceLocation, Vec<RegionInfo>>>,
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

        return Ok(FunctionCoverage::new(
            source_name,
            filename,
            no_coverage,
            BTreeMap::new(),
            annotated,
            BranchCoverage::default(),
        ));
    };

    let mut covered = vec![];
    let mut regions = BTreeMap::new();
    let file_region_infos = region_infos.get(&absolute_path);

    for line in source_lines {
        // one more thing to do: within a function, some lines will always be uncovered (e.g. }
        // closing braces). so we do have to trust the coverage tool to report those accurately.
        let line_regions = column_ranges_for_line(func_coverage, line, file_region_infos);
        let status = if line_regions.is_empty() {
            LineCoverageStatus::Ignored
        } else if line_regions.iter().all(|range| range.tested) {
            LineCoverageStatus::Tested
        } else {
            LineCoverageStatus::Untested
        };
        // Keep every region covering this line (not just untested ones) so the report can show
        // what regions cover a given line, regardless of its overall status.
        if !line_regions.is_empty() {
            regions.insert(line, line_regions);
        }
        covered.push((line, status));
    }

    let annotated = get_annotation_status(annotations, &mut covered);

    let annotated_lines: HashSet<usize> = covered
        .iter()
        .filter(|(_, status)| *status == LineCoverageStatus::Annotated)
        .map(|(line, _)| *line)
        .collect();

    let branches = file_region_infos
        .map(|infos| count_branches(infos, start_line..=end_line, &annotated_lines))
        .unwrap_or_default();

    Ok(FunctionCoverage::new(
        source_name,
        filename,
        LineCoverage { lines: covered },
        regions,
        annotated,
        branches,
    ))
}

// Ported from the private `CoverageMapping::get_simple_counters`
fn simple_counters(
    profile: &InstrumentationProfile,
    func: &FunctionRecordV3,
) -> HashMap<Counter, i64> {
    let mut result = HashMap::new();
    result.insert(Counter::default(), 0);
    if let Some(func_record) = profile.find_record_by_hash(func.header.name_hash) {
        for (id, count) in func_record.record.counts.iter().enumerate() {
            result.insert(Counter::instrumentation(id as u64), *count as i64);
        }
    }
    result
}

// Ported from the resolution loop inside `CoverageMapping::generate_subreport`
// because the public `CoverageReport.hits` only shows hits for `true` branch
// for `Branch` regions. `false` branch count is lost in public API.
fn resolve_all_counters(
    profile: &InstrumentationProfile,
    func: &FunctionRecordV3,
) -> HashMap<Counter, i64> {
    let mut region_ids = simple_counters(profile, func);

    let mut pending_exprs = vec![];
    for (expr_index, expr) in func.expressions.iter().enumerate().rev() {
        let lhs = region_ids.get(&expr.lhs).copied();
        let rhs = region_ids.get(&expr.rhs).copied();
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => {
                let count = match expr.kind {
                    ExprKind::Subtract => lhs.saturating_sub(rhs),
                    ExprKind::Add => lhs.saturating_add(rhs),
                };
                region_ids.insert(
                    Counter { kind: CounterType::Expression(expr.kind), id: expr_index as u64 },
                    count,
                );
            }
            _ => {
                // These counters were optimised out, so just add them in as 0.
                if lhs.is_none() && expr.lhs.is_instrumentation() {
                    region_ids.insert(expr.lhs, 0);
                }
                if rhs.is_none() && expr.rhs.is_instrumentation() {
                    region_ids.insert(expr.rhs, 0);
                }
                pending_exprs.push(Some((expr_index, expr)));
            }
        }
    }

    // ``rev()` doesn't guarantee a dependency-respecting order`.
    // So expressions can depend on other expressions that haven't been
    // resolved yet.
    // Retry the unresolved ones until none are left or we run out of attempts.
    let mut index = 0;
    let mut cleared_expressions = 0;
    let mut tries_left = pending_exprs.len() + 1;
    while pending_exprs.len() != cleared_expressions {
        assert!(tries_left > 0, "could not resolve all counter expressions for {func:?}");
        if index >= pending_exprs.len() {
            index = 0;
            tries_left -= 1;
        }
        let Some((expr_index, expr)) = pending_exprs[index] else {
            index += 1;
            continue;
        };
        let lhs = region_ids.get(&expr.lhs).copied();
        let rhs = region_ids.get(&expr.rhs).copied();
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => {
                let count = match expr.kind {
                    ExprKind::Subtract => lhs - rhs,
                    ExprKind::Add => lhs + rhs,
                };
                region_ids.insert(
                    Counter { kind: CounterType::Expression(expr.kind), id: expr_index as u64 },
                    count,
                );
                pending_exprs[index] = None;
                cleared_expressions += 1;
            }
            _ => index += 1,
        }
    }

    region_ids
}

// Ported from `CoverageReport::apply_remapping`
fn remap_coverage_path(path: PathBuf, path_remapping: Option<&str>) -> PathBuf {
    let Some(path_remapping) = path_remapping else { return path };
    let Some((source, dest)) = path_remapping.split_once(',') else { return path };
    match path.strip_prefix(Path::new(source)) {
        Ok(rest) => Path::new(dest).join(rest),
        Err(_) => path,
    }
}

fn column_ranges_for_line(
    func_coverage: &CoverageResult,
    line: usize,
    region_infos: Option<&BTreeMap<SourceLocation, Vec<RegionInfo>>>,
) -> Vec<ColumnRange> {
    let mut ranges: Vec<ColumnRange> = func_coverage
        .hits
        .iter()
        .filter(|(loc, _)| loc.line_start <= line && loc.line_end >= line)
        .map(|(loc, &count)| {
            let start_col = if loc.line_start == line { loc.column_start } else { 1 };
            let end_col = if loc.line_end == line { Some(loc.column_end) } else { None };
            let regions =
                region_infos.and_then(|infos| infos.get(loc)).cloned().unwrap_or_default();
            ColumnRange { start_col, end_col, tested: count > 0, regions }
        })
        .collect();
    ranges.sort_by_key(|range| range.start_col);
    ranges
}

impl ShowCommand {
    fn run(&self) -> Result<()> {
        if self.debug {
            let _ = enable_debug_logging();
        }
        let instr_prof = if self.instr_profile.len() == 1 {
            parse(&self.instr_profile[0])?
        } else if self.instr_profile.len() > 1 {
            merge_profiles(&self.instr_profile)?
        } else {
            panic!("Must provide an instrumentation profile");
        };
        let mapping = CoverageMapping::new(&self.objects, &instr_prof, false)?;
        let mut report = mapping.generate_report()?;
        let remapping = self
            .path_remapping
            .as_ref()
            .map(|s| s.parse::<PathRemapping>())
            .transpose()
            .map_err(|e| anyhow::anyhow!("invalid --path-equivalence value: {e:?}"))?;
        if let Some(remapping) = remapping.as_ref() {
            report.apply_remapping(remapping);
        }

        let region_infos = self.gather_region_infos(&instr_prof, mapping)?;

        let mut coverage =
            rustc_driver::coverage(self, &report, self.path_remapping.as_deref(), &region_infos)?;
        coverage.sort_by(|f1, f2| f1.source_name.cmp(&f2.source_name));
        let coverage = coverage;

        let total = coverage.len();
        let mut count_fully_tested = 0;
        let mut count_fully_annotated = 0;
        let mut count_partially_tested = 0;
        let mut count_fully_untested = 0;
        let mut count_fully_ignored = 0;
        for function in &coverage {
            match function.status {
                FunctionCoverageStatus::FullyTested => count_fully_tested += 1,
                FunctionCoverageStatus::FullyIgnored => count_fully_ignored += 1,
                _ if function.annotated == Annotated::Fully => count_fully_annotated += 1,
                FunctionCoverageStatus::PartiallyTested => count_partially_tested += 1,
                FunctionCoverageStatus::FullyUntested => count_fully_untested += 1,
            };
        }
        println!(
            "---\n\
            Fully Tested: {count_fully_tested}\n\
            Fully Annotated: {count_fully_annotated}\n\
            Partially tested: {count_partially_tested}\n\
            Fully untested: {count_fully_untested}\n\
            Fully ignored: {count_fully_ignored}\n\
            Total: {total}\n\
            ---"
        );

        if let Some(ref html_out) = self.html_out {
            let out = html_out.display();
            let html = html_report::generate(&coverage, &self.ferrocene)
                .context("failed to generate HTML report")?;
            let parent = html_out.parent().unwrap();
            std::fs::create_dir_all(parent)
                .context(format!("Failed to create {}", parent.display()))?;
            std::fs::write(html_out, html.render().into_string())
                .context(format!("failed to write HTML report to {out}"))?;
            println!("Generated coverage report at {out}");
        }

        Ok(())
    }

    fn gather_region_infos(
        &self,
        instr_prof: &InstrumentationProfile,
        mapping: CoverageMapping<'_>,
    ) -> Result<BTreeMap<PathBuf, BTreeMap<SourceLocation, Vec<RegionInfo>>>, anyhow::Error> {
        let mut region_infos: BTreeMap<PathBuf, BTreeMap<SourceLocation, Vec<RegionInfo>>> =
            BTreeMap::new();
        for info in mapping.mapping_info_iter() {
            let info = info?;
            for func in &info.cov_fun {
                let paths = info.get_files_from_id(func.header.filenames_ref);
                if paths.is_empty() {
                    continue;
                }
                let resolved = resolve_all_counters(instr_prof, func);
                for region in &func.regions {
                    let Some(path) = paths.get(region.file_id) else { continue };
                    let path = remap_coverage_path(path.clone(), self.path_remapping.as_deref());
                    let true_count = resolved.get(&region.count).copied().unwrap_or_default();
                    let false_count =
                        matches!(region.kind, RegionKind::Branch | RegionKind::MCDCBranch).then(
                            || resolved.get(&region.false_count).copied().unwrap_or_default(),
                        );

                    let by_loc = region_infos
                        .entry(path)
                        .or_default()
                        .entry(region.loc.clone())
                        .or_default();
                    match by_loc.iter_mut().find(|info| info.kind == region.kind) {
                        Some(existing) => {
                            existing.true_count += true_count;
                            if let Some(fc) = false_count {
                                *existing.false_count.get_or_insert(0) += fc;
                            }
                        }
                        None => {
                            by_loc.push(RegionInfo { kind: region.kind, true_count, false_count })
                        }
                    }
                }
            }
        }
        Ok(region_infos)
    }
}

fn enable_debug_logging() -> anyhow::Result<()> {
    let fmt = tracing_subscriber::fmt::Layer::default();
    let subscriber = fmt
        .with_filter(filter_fn(|metadata| metadata.target().contains("llvm_profparser")))
        .with_subscriber(Registry::default());
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Show { show } => show.run(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code_range(start_col: usize, end_col: usize, tested: bool) -> ColumnRange {
        ColumnRange {
            start_col,
            end_col: Some(end_col),
            tested,
            regions: vec![RegionInfo {
                kind: RegionKind::Code,
                true_count: tested as i64,
                false_count: None,
            }],
        }
    }

    fn branch_column_range(
        start_col: usize,
        end_col: usize,
        true_count: i64,
        false_count: i64,
    ) -> ColumnRange {
        ColumnRange {
            start_col,
            end_col: Some(end_col),
            tested: true_count > 0,
            regions: vec![RegionInfo {
                kind: RegionKind::Branch,
                true_count,
                false_count: Some(false_count),
            }],
        }
    }

    fn all_tested_lines(count: usize) -> LineCoverage {
        LineCoverage { lines: (1..=count).map(|l| (l, LineCoverageStatus::Tested)).collect() }
    }

    #[test]
    fn fully_tested_lines_with_fully_taken_branch_stay_fully_tested() {
        let lines = all_tested_lines(1);
        let mut regions = BTreeMap::new();
        regions.insert(1, vec![branch_column_range(1, 4, 3, 2)]);

        assert_eq!(
            FunctionCoverageStatus::new(&lines, &regions),
            FunctionCoverageStatus::FullyTested
        );
    }

    #[test]
    fn fully_tested_lines_with_untested_branch_arm_become_partially_tested() {
        let lines = all_tested_lines(1);
        let mut regions = BTreeMap::new();
        // `true` arm hit 3 times, `false` arm never taken.
        regions.insert(1, vec![branch_column_range(1, 4, 3, 0)]);

        assert_eq!(
            FunctionCoverageStatus::new(&lines, &regions),
            FunctionCoverageStatus::PartiallyTested
        );
    }

    #[test]
    fn untested_function_with_untested_branch_stays_fully_untested() {
        let lines = LineCoverage { lines: vec![(1, LineCoverageStatus::Untested)] };
        let mut regions = BTreeMap::new();
        regions.insert(1, vec![branch_column_range(1, 4, 0, 0)]);

        assert_eq!(
            FunctionCoverageStatus::new(&lines, &regions),
            FunctionCoverageStatus::FullyUntested
        );
    }

    #[test]
    fn non_branch_regions_never_downgrade_status() {
        let lines = all_tested_lines(1);
        let mut regions = BTreeMap::new();
        regions.insert(1, vec![code_range(1, 4, true)]);

        assert_eq!(
            FunctionCoverageStatus::new(&lines, &regions),
            FunctionCoverageStatus::FullyTested
        );
    }

    fn loc(line_start: usize, line_end: usize) -> SourceLocation {
        SourceLocation { line_start, column_start: 1, line_end, column_end: 1 }
    }

    #[test]
    fn count_branches_counts_both_arms_separately() {
        let mut infos = BTreeMap::new();
        infos.insert(
            loc(2, 2),
            vec![RegionInfo { kind: RegionKind::Branch, true_count: 3, false_count: Some(0) }],
        );

        assert_eq!(
            count_branches(&infos, 1..=5, &HashSet::new()),
            BranchCoverage { tested: 1, untested: 1, annotated: 0 }
        );
    }

    #[test]
    fn count_branches_ignores_locations_outside_the_function() {
        let mut infos = BTreeMap::new();
        infos.insert(
            loc(10, 10),
            vec![RegionInfo { kind: RegionKind::Branch, true_count: 1, false_count: Some(1) }],
        );

        assert_eq!(count_branches(&infos, 1..=5, &HashSet::new()), BranchCoverage::default());
    }

    #[test]
    fn count_branches_counts_a_multiline_branch_region_once() {
        // A branch region spanning lines 2..=3 is deduplicated by its unique
        // `SourceLocation`, not double-counted even though it covers two lines.
        let mut infos = BTreeMap::new();
        infos.insert(
            loc(2, 3),
            vec![RegionInfo { kind: RegionKind::Branch, true_count: 1, false_count: Some(1) }],
        );

        assert_eq!(
            count_branches(&infos, 1..=5, &HashSet::new()),
            BranchCoverage { tested: 2, untested: 0, annotated: 0 }
        );
    }

    #[test]
    fn count_branches_buckets_untested_arm_on_annotated_line_as_annotated() {
        let mut infos = BTreeMap::new();
        infos.insert(
            loc(2, 2),
            vec![RegionInfo { kind: RegionKind::Branch, true_count: 3, false_count: Some(0) }],
        );
        let annotated_lines: HashSet<usize> = [2].into_iter().collect();

        assert_eq!(
            count_branches(&infos, 1..=5, &annotated_lines),
            BranchCoverage { tested: 1, untested: 0, annotated: 1 }
        );
    }
}
