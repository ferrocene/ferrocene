// Derived from https://github.com/xd009642/llvm-profparser/blob/f12a20d33b371f62a3b63f3a19d2320c25aa48b9/src/bin/cov.rs

#[allow(unused_imports)]
use anyhow::{Context as _, Result};
use llvm_profparser::*;
use maud::Render;
use std::fmt;
use std::fs::{self};
use std::path::PathBuf;
use structopt::StructOpt;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

// mod rustdoc;
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        self.lines
            .iter()
            .filter(|(_, s)| matches!(s, LineCoverageStatus::Ignored))
            .count()
    }
    fn considered(&self) -> usize {
        self.lines
            .iter()
            .filter(|(_, s)| !matches!(s, LineCoverageStatus::Ignored))
            .count()
    }
    fn tested(&self) -> usize {
        self.lines
            .iter()
            .filter(|(_, s)| matches!(s, LineCoverageStatus::Tested))
            .count()
    }
}

#[derive(Debug, PartialEq)]
enum FunctionCoverageStatus {
    FullyTested,
    PartiallyTested,
    FullyUntested,
    FullyIngored,
}

impl FunctionCoverageStatus {
    fn new(lines: &LineCoverage) -> Self {
        match lines {
            lines if lines.lines.iter().all(|(_, status)| {
                *status == LineCoverageStatus::Ignored
            }) => FunctionCoverageStatus::FullyIngored,
            lines if lines.lines.iter().all(|(_, status)| {
                *status != LineCoverageStatus::Untested
            }) => FunctionCoverageStatus::FullyTested,
            lines if lines.lines.iter().all(|(_, status)| {
                *status != LineCoverageStatus::Tested
            }) => FunctionCoverageStatus::FullyUntested,
            _ => FunctionCoverageStatus::PartiallyTested,
        }
    }

    fn to_css_class(&self) -> &str {
        match self {
            FunctionCoverageStatus::FullyTested => "fully-tested",
            FunctionCoverageStatus::PartiallyTested => "partially-tested",
            FunctionCoverageStatus::FullyUntested => "fully-untested",
            FunctionCoverageStatus::FullyIngored => "fully-ignored",
        }
    }
}

#[derive(Debug)]
struct FunctionCoverage {
    source_name: String,
    filename: PathBuf,
    lines: LineCoverage,
    status: FunctionCoverageStatus,
}

impl FunctionCoverage {
    fn new(source_name: String, filename: PathBuf, lines: LineCoverage) -> Self {
        let status = FunctionCoverageStatus::new(&lines); 
        Self {
            source_name,
            filename,
            lines,
            status,
        }
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
    let Span {
        mut filename,
        start_line,
        end_line,
    } = span;
    if filename.is_relative() {
        filename = ferrocene.join(filename);
    }
    let filename = if filename.is_absolute() {
        fs::canonicalize(&filename).context(format!("failed to canonicalize {filename:?}"))
    } else {
        panic!("--ferrocene-src was not absolute")
    }?;
    let source_lines = start_line..=end_line;
    let source_name = source_name;
    let filename = filename.clone();

    // we didn't get any hits from the tool, so we don't know which lines shouldn't be
    // considered. report them all as considered and missing coverage.
    let Some(func_coverage) = report.files.get(&filename) else {
        let no_coverage = LineCoverage {
            lines: source_lines
                .clone()
                .map(|i| (i, LineCoverageStatus::Untested))
                .collect(),
        };
        println!(
            "warning: couldn't find source file {} in coverage report",
            filename.display()
        );
        return Ok(FunctionCoverage::new(
            source_name,
            filename,
            no_coverage,
        ))
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
    Ok(FunctionCoverage::new(
        source_name,
        filename,
        LineCoverage { lines: covered },
    ))
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

        // let coverage = rustdoc::coverage(self, &report)?;
        let coverage = rustc_driver::coverage(self, &report)?;
        for func in &coverage {
            print!("{}: ", func.source_name);
            if func.lines.considered() == 0 {
                println!(
                    "BUG: no lines considered (span: {}:{}-{})",
                    func.filename.display(),
                    func.lines.lines.first().unwrap().0,
                    func.lines.lines.last().unwrap().0
                );
            } else {
                let missing = func
                    .lines
                    .lines
                    .iter()
                    .filter(|(_, status)| *status == LineCoverageStatus::Untested)
                    .map(|(linenum, _)| format!("{}:{}", func.filename.display(), linenum))
                    .collect::<Vec<_>>();
                println!(
                    "{} / {} covered ({} lines unconsidered)\
                    {}\
                ",
                    func.lines.tested(),
                    func.lines.considered(),
                    func.lines.unconsidered(),
                    if !missing.is_empty() {
                        format!("\n\tMissing lines:\n\t\t{}\n", missing.join("\n\t\t"))
                    } else {
                        "".into()
                    },
                );
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
                FunctionCoverageStatus::FullyIngored => count_fully_ignored += 1,
            };
        }
        println!("\
            ---\n\
            Fully Tested: {count_fully_tested}\n\
            Partially tested: {count_partially_tested}\n\
            Fully untested: {count_fully_untested}\n\
            Fully ignored: {count_fully_ignored}\n\
            Total: {total}\n\
            ---\
        ");
        println!(
            "hits for <u8 as PartialOrd>::partial_cmp: {:?}",
            report
                .files
                .get(&self.ferrocene.join("library/core/src/cmp.rs"))
                .unwrap()
                .hits_for_line(1978)
        );

        if let Some(ref html_out) = self.html_out {
            let html = html_report::generate(coverage, &self.ferrocene)?;
            std::fs::write(html_out, html.render().into_string())?;
        }

        Ok(())
    }
}

fn enable_debug_logging() -> anyhow::Result<()> {
    let fmt = tracing_subscriber::fmt::Layer::default();
    let subscriber = fmt
        .with_filter(filter_fn(|metadata| {
            metadata.target().contains("llvm_profparser")
        }))
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
