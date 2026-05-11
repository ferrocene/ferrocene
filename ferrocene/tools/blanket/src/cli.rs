use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use crate::{html_report, rustc_driver, Annotated};
use llvm_profparser::{PathRemapping, CoverageMapping, merge_profiles, parse,};
use std::path::PathBuf;
use maud::Render;
use crate::FunctionCoverageStatus;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct Opts {
    #[command(subcommand)]
    cmd: Command,
}

impl Opts {
    pub(crate) fn run(&self) -> Result<()> {
        match &self.cmd {
            Command::Show { show } => show.run(),
        }
    }
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
    path_remapping: Option<PathRemapping>,
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


impl ShowCommand {
    pub(crate) fn run(&self) -> Result<()> {
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

        let mut coverage = rustc_driver::coverage(&self.ferrocene, &self.symbol_report, &report)?;
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
}


fn enable_debug_logging() -> anyhow::Result<()> {
    let fmt = tracing_subscriber::fmt::Layer::default();
    let subscriber = fmt
        .with_filter(filter_fn(|metadata| metadata.target().contains("llvm_profparser")))
        .with_subscriber(Registry::default());
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
