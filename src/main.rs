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
enum CoverageStatus {
    Tested,
    Untested,
    Ignored,
}

impl fmt::Display for CoverageStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoverageStatus::Tested => write!(f, "Tested"),
            CoverageStatus::Untested => write!(f, "Untested"),
            CoverageStatus::Ignored => write!(f, "Ignored"),
        }
    }
}

#[derive(Debug)]
struct LineCoverage {
    lines: Vec<(usize, CoverageStatus)>,
}

impl LineCoverage {
    fn len(&self) -> usize {
        self.lines.len()
    }
    fn unconsidered(&self) -> usize {
        self.lines
            .iter()
            .filter(|(_, s)| matches!(s, CoverageStatus::Ignored))
            .count()
    }
    fn considered(&self) -> usize {
        self.lines
            .iter()
            .filter(|(_, s)| !matches!(s, CoverageStatus::Ignored))
            .count()
    }
    fn tested(&self) -> usize {
        self.lines
            .iter()
            .filter(|(_, s)| matches!(s, CoverageStatus::Tested))
            .count()
    }
}

#[derive(Debug)]
struct FunctionCoverage {
    source_name: String,
    #[allow(dead_code)]
    filename: PathBuf,
    lines: LineCoverage,
}

#[derive(Clone)]
enum FunctionType {
    Canonical(String), // qualified path
    ProvidedDefault,   // appears in traits
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
    let no_coverage = FunctionCoverage {
        source_name,
        // symbol_name: "TODO sorry".into(),
        filename: filename.clone(),
        // we didn't get any hits from the tool, so we don't know which lines shouldn't be
        // considered. report them all as considered and missing coverage.
        lines: LineCoverage {
            lines: source_lines
                .clone()
                .map(|i| (i, CoverageStatus::Untested))
                .collect(),
        },
    };
    let Some(func_coverage) = report.files.get(&filename) else {
        println!(
            "warning: couldn't find source file {} in coverage report",
            filename.display()
        );
        return Ok(no_coverage);
    };
    let mut covered = vec![];
    for line in source_lines {
        // one more thing to do: within a function, some lines will always be uncovered (e.g. }
        // closing braces). so we do have to trust the coverage tool to report those accurately.
        let status = match func_coverage.hits_for_line(line) {
            None => CoverageStatus::Ignored,
            Some(0) => CoverageStatus::Untested,
            Some(_) => CoverageStatus::Tested,
        };
        covered.push((line, status));
    }
    Ok(FunctionCoverage {
        lines: LineCoverage { lines: covered },
        ..no_coverage
    })
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
        let mut unconsidered = 0;
        let mut fully_covered = 0;
        for func in &coverage {
            print!("{}: ", func.source_name);
            if func.lines.considered() == 0 {
                println!(
                    "BUG: no lines considered (span: {}:{}-{})",
                    func.filename.display(),
                    func.lines.lines.first().unwrap().0,
                    func.lines.lines.last().unwrap().0
                );
                unconsidered += 1;
            } else {
                let missing = func
                    .lines
                    .lines
                    .iter()
                    .filter(|(_, status)| *status == CoverageStatus::Untested)
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
                fully_covered += (func.lines.tested() == func.lines.considered()) as usize;
            }
        }
        let total = coverage.len();
        println!(
            "{fully_covered}/{}/{unconsidered}/{total} (fully covered / partially covered / unconsidered / total) functions",
            total - unconsidered - fully_covered
        );
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
