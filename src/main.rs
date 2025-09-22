// Derived from https://github.com/xd009642/llvm-profparser/blob/f12a20d33b371f62a3b63f3a19d2320c25aa48b9/src/bin/cov.rs

#![feature(normalize_lexically)]

use anyhow::{Context as _, Result};
use llvm_profparser::*;
use rustdoc_types::{Function, Item, ItemEnum, Span};
use std::fs::{self, File};
use std::path::PathBuf;
use structopt::StructOpt;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

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
    #[structopt(long = "rustdoc-json", short = "j")]
    rustdoc_json: PathBuf,
    #[structopt(long = "ferrocene-src", short = "s")]
    ferrocene: PathBuf,
    /// Turn on debug logging
    #[structopt(long)]
    debug: bool,
}

#[derive(Debug)]
enum CoverageStatus {
    Tested,
    Untested,
    Ignored,
}

#[derive(Debug)]
struct LineCoverage {
    lines: Vec<(usize, CoverageStatus)>,
}

impl LineCoverage {
    fn len(&self) -> usize {
        self.lines.len()
    }
    fn considered(&self) -> usize {
        self.lines.iter().filter(|(_, s)| !matches!(s, CoverageStatus::Ignored)).count()
    }
    fn tested(&self) -> usize {
        self.lines.iter().filter(|(_, s)| matches!(s, CoverageStatus::Tested)).count()
    }
}

#[derive(Debug)]
struct FunctionCoverage {
    source_name: String,
    // symbol_name: String,
    filename: PathBuf,
    lines: LineCoverage,
}

fn get_coverage(report: &CoverageReport, func: &Function, span: Span, ferrocene: &std::path::Path, source_name: String) -> Result<FunctionCoverage> {
    let Span { mut filename, begin: (start_line, _), end: (end_line, _), .. } = span;
    if filename.is_relative() {
        filename = ferrocene.join(filename);
    }
    let filename = if filename.is_absolute() {
        fs::canonicalize(&filename).context(format!("failed to canonicalize {filename:?}"))
    } else {
        panic!();
        // filename.normalize_lexically().context(format!("failed to normalize {filename:?}"))
    }?;
    let source_lines = start_line..=end_line;
    let no_coverage = FunctionCoverage {
        source_name,
        // symbol_name: "TODO sorry".into(),
        filename: filename.clone(),
        // we didn't get any hits from the tool, so we don't know which lines shouldn't be
        // considered. report them all as considered and missing coverage.
        lines: LineCoverage {
            lines: source_lines.clone()
                .map(|i| (i, CoverageStatus::Untested)).collect(),
        },
    };
    let Some(func_coverage) = report.files.get(&filename) else {
        println!("warning: couldn't find source file {} in coverage report", filename.display());
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
    Ok(FunctionCoverage { lines: LineCoverage { lines: covered }, ..no_coverage })
}

impl ShowCommand {
    fn run(&self) -> Result<()> {
        if self.debug {
            let _ = enable_debug_logging();
        }
        let rustdoc: rustdoc_types::Crate = serde_json::from_reader(
            File::open(&self.rustdoc_json).context(format!("failed to open rustdoc-json file {}", self.rustdoc_json.display()))?
        )?;
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

        let mut coverage = vec![];
        for item in rustdoc.index.values() {
            if let ItemEnum::Function(func) = &item.inner {
                let span = item.span.as_ref().expect("TODO: handle no span for rustdoc-json function");
                let name = item.name.clone().expect("no name for rustdoc-json function?");
                coverage.push(get_coverage(&report, func, span.clone(), &self.ferrocene, name)?);
            }
        }

        let mut unconsidered = 0;
        let mut fully_covered = 0;
        for func in &coverage {
            print!("{}: ", func.source_name);
            if func.lines.considered() == 0 {
                println!("BUG: no lines considered");
                unconsidered += 1;
            } else {
                println!("{}/{} covered ({} lines unconsidered)",
                    func.lines.tested(),
                    func.lines.considered(),
                    func.lines.len(),
                );
                fully_covered += (func.lines.tested() == func.lines.considered()) as usize;
            }
        }
        let total = coverage.len();
        println!("{fully_covered}/{}/{unconsidered}/{total} (fully covered / partially covered / unconsidered / total) functions", total - unconsidered - fully_covered);

        // for (path, result) in report.files.iter() {
        //     // Read file to string
        //     if let Ok(source) = fs::read_to_string(path) {
        //         if report.files.len() > 1 {
        //             println!("{}", path.display());
        //         }
        //         for (line, source) in source.lines().enumerate() {
        //             print!("{: >5}|", line + 1);
        //             if let Some(hits) = result.hits_for_line(line + 1) {
        //                 println!("{: >7}|{}", hits, source);
        //             } else {
        //                 println!("       |{}", source);
        //             }
        //         }
        //         println!();
        //     }
        // }
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
