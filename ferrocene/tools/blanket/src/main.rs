// Derived from https://github.com/xd009642/llvm-profparser/blob/f12a20d33b371f62a3b63f3a19d2320c25aa48b9/src/bin/cov.rs

use std::fmt;
use std::path::PathBuf;

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
use llvm_profparser::*;
use maud::Render;
use structopt::StructOpt;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

mod html_report;
mod rustc_driver;

#[derive(Clone, Debug, Eq, PartialEq, StructOpt)]
pub struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Clone, Debug, Eq, PartialEq, StructOpt)]
pub enum Command {
    Show {
        #[structopt(flatten)]
        show: ShowCommand,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, StructOpt)]
pub struct ShowCommand {
    /// File with the profile data obtained after an instrumented run. This differs from llvm-cov
    /// in that if multiple profiles are given it will do the equivalent of a llvm-profdata merge
    /// on them.
    #[structopt(long = "instr-profile", short = "p")]
    instr_profile: Vec<PathBuf>,
    /// Coverage executable or object file
    #[structopt(long = "object", short = "o")]
    objects: Vec<PathBuf>,
    /// Pair of paths for a remapping to allow loading files after move. Comma separated in the
    /// order `source,dest`
    #[structopt(long = "path-equivalence")]
    path_remapping: Option<PathRemapping>,
    // #[structopt(long = "rustdoc-json", short = "j")]
    // rustdoc_json: PathBuf,
    #[structopt(long = "report", short = "r")]
    symbol_report: PathBuf,
    #[structopt(long = "ferrocene-src", short = "s")]
    ferrocene: PathBuf,
    /// Turn on debug logging
    #[structopt(long)]
    debug: bool,
    /// Produce a HTML report
    #[structopt(long)]
    html_out: Option<PathBuf>,
}

#[derive(Debug, PartialEq)]
enum LineCoverageStatus {
    Tested,
    Untested,
    Ignored,
}

impl fmt::Display for LineCoverageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineCoverageStatus::Tested => write!(f, "Tested"),
            LineCoverageStatus::Untested => write!(f, "Untested"),
            LineCoverageStatus::Ignored => write!(f, "Ignored"),
        }
    }
}

#[derive(Debug)]
struct LineCoverage {
    lines: Vec<(usize, LineCoverageStatus)>,
}

impl LineCoverage {
    fn unconsidered(&self) -> usize {
        self.lines.iter().filter(|(_, s)| matches!(s, LineCoverageStatus::Ignored)).count()
    }
    fn considered(&self) -> usize {
        self.lines.iter().filter(|(_, s)| !matches!(s, LineCoverageStatus::Ignored)).count()
    }
    fn tested(&self) -> usize {
        self.lines.iter().filter(|(_, s)| matches!(s, LineCoverageStatus::Tested)).count()
    }
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

#[derive(Debug)]
struct FunctionCoverage {
    source_name: String,
    relative_path: PathBuf,
    lines: LineCoverage,
    status: FunctionCoverageStatus,
}

impl FunctionCoverage {
    fn new(source_name: String, filename: PathBuf, lines: LineCoverage) -> Self {
        let status = FunctionCoverageStatus::new(&lines);
        Self { source_name, relative_path: filename, lines, status }
    }
}

struct Span {
    filename: PathBuf,
    start_line: usize,
    end_line: usize,
}

fn get_coverage(
    report: &CoverageReport,
    span: Span,
    ferrocene: &std::path::Path,
    source_name: String,
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
        let no_coverage = LineCoverage {
            lines: source_lines.clone().map(|i| (i, LineCoverageStatus::Untested)).collect(),
        };
        println!(
            "warning: couldn't find source file {} in coverage report",
            absolute_path.display()
        );
        return Ok(FunctionCoverage::new(source_name, filename, no_coverage));
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
    Ok(FunctionCoverage::new(source_name, filename, LineCoverage { lines: covered }))
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
        let mut report = mapping.generate_report();
        if let Some(remapping) = self.path_remapping.as_ref() {
            report.apply_remapping(remapping);
        }

        let mut coverage = rustc_driver::coverage(self, &report)?;
        coverage.sort_by(|f1, f2| f1.source_name.cmp(&f2.source_name));
        let coverage = coverage;

        for func in &coverage {
            print!("{}: ", func.source_name);
            if func.lines.considered() == 0 {
                println!(
                    "BUG: no lines considered (span: {}:{}-{})",
                    func.relative_path.display(),
                    func.lines.lines.first().unwrap().0,
                    func.lines.lines.last().unwrap().0
                );
            } else {
                let missing = func
                    .lines
                    .lines
                    .iter()
                    .filter(|(_, status)| *status == LineCoverageStatus::Untested)
                    .map(|(linenum, _)| format!("{}:{}", func.relative_path.display(), linenum))
                    .collect::<Vec<_>>();
                if self.debug || !missing.is_empty() {
                    println!(
                        "{} / {} covered ({} lines unconsidered)\n\
                        \tMissing lines:\n\t\t{}\n\
                        ",
                        func.lines.tested(),
                        func.lines.considered(),
                        func.lines.unconsidered(),
                        missing.join("\n\t\t"),
                    );
                }
            }
        }
        let total = coverage.len();
        let mut count_fully_tested = 0;
        let mut count_partially_tested = 0;
        let mut count_fully_untested = 0;
        let mut count_fully_ignored = 0;
        for function in &coverage {
            match function.status {
                FunctionCoverageStatus::FullyTested => count_fully_tested += 1,
                FunctionCoverageStatus::PartiallyTested => count_partially_tested += 1,
                FunctionCoverageStatus::FullyUntested => count_fully_untested += 1,
                FunctionCoverageStatus::FullyIgnored => count_fully_ignored += 1,
            };
        }
        println!(
            "---\n\
            Fully Tested: {count_fully_tested}\n\
            Partially tested: {count_partially_tested}\n\
            Fully untested: {count_fully_untested}\n\
            Fully ignored: {count_fully_ignored}\n\
            Total: {total}\n\
            ---"
        );

        if let Some(ref html_out) = self.html_out {
            let html = html_report::generate(&coverage, &self.ferrocene)
                .context("failed to generate HTML report")?;
            std::fs::write(html_out, html.render().into_string())
                .context("failed to write HTML report to disk")?;
            println!("Generated coverage report at {}", html_out.display());
        }

        Ok(())
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
    let opts = Opts::from_args();
    match opts.cmd {
        Command::Show { show } => show.run(),
    }
}
