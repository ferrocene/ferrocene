use anyhow::Result;
use clap::{Parser, Subcommand};
use llvm_profparser::*;
use std::fs;
use std::path::PathBuf;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

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
    #[structopt(long = "instr-profile")]
    instr_profile: Vec<PathBuf>,
    /// Coverage executable or object file
    #[structopt(long = "object")]
    objects: Vec<PathBuf>,
    /// Pair of paths for a remapping to allow loading files after move. Comma separated in the
    /// order `source,dest`
    #[structopt(long = "path-equivalence")]
    path_remapping: Option<PathRemapping>,
    /// Turn on debug logging
    #[structopt(long)]
    debug: bool,
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
        if let Some(remapping) = self.path_remapping.as_ref() {
            report.apply_remapping(remapping);
        }
        for (path, result) in report.files.iter() {
            // Read file to string
            if let Ok(source) = fs::read_to_string(path) {
                if report.files.len() > 1 {
                    println!("{}", path.display());
                }
                for (line, source) in source.lines().enumerate() {
                    print!("{: >5}|", line + 1);
                    if let Some(hits) = result.hits_for_line(line + 1) {
                        println!("{: >7}|{}", hits, source);
                    } else {
                        println!("       |{}", source);
                    }
                }
                println!();
            }
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
    let opts = Opts::parse();
    match opts.cmd {
        Command::Show { show } => show.run(),
    }
}
