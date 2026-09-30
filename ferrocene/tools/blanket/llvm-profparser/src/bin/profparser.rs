use anyhow::Result;
use clap::Parser;
use llvm_profparser::instrumentation_profile::summary::*;
use llvm_profparser::instrumentation_profile::types::*;
use llvm_profparser::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path::PathBuf;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::{Layer, Registry};

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub enum Command {
    Show {
        #[command(flatten)]
        show: ShowCommand,
    },
    Merge {
        #[command(flatten)]
        merge: MergeCommand,
    },
    Overlap {
        #[command(flatten)]
        overlap: OverlapCommand,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct ShowCommand {
    /// Input profraw file to show some information about
    #[structopt(name = "<filename...>", long = "input", short = 'i')]
    input: PathBuf,
    /// Show counter values for shown functions
    #[structopt(long = "counts")]
    show_counts: bool,
    /// Details for every function
    #[structopt(long = "all-functions")]
    all_functions: bool,
    /// Show instr profile data in text dump format
    #[structopt(long = "text")]
    text: bool,
    /// Show detailed profile summary
    #[structopt(long = "show_detailed_summary")]
    show_detailed_summary: bool,
    /// Cutoff percentages (times 10000) for generating detailed summary
    #[structopt(long = "detailed_summary_cutoffs")]
    detailed_summary_cutoffs: Vec<usize>,
    /// Show profile summary of a list of hot functions
    #[structopt(long = "show_hot_fn_list")]
    show_hot_fn_list: bool,
    /// Show context sensitive counts
    #[structopt(long = "showcs")]
    showcs: bool,
    /// Details for matching functions
    #[structopt(long = "function")]
    function: Option<String>,
    /// Output file
    #[structopt(long = "output", short = 'o')]
    output: Option<String>,
    /// Show the list of functions with the largest internal counts
    #[structopt(long = "topn")]
    topn: Option<usize>,
    /// Set the count value cutoff. Functions with the maximum count less than
    /// this value will not be printed out. (Default is 0)
    #[structopt(long = "value_cutoff", default_value = "0")]
    value_cutoff: u64,
    /// Set the count value cutoff. Functions with the maximum count below the
    /// cutoff value
    #[structopt(long = "only_list_below")]
    only_list_below: bool,
    /// Show profile symbol list if it exists in the profile.
    #[structopt(long = "show_profile_sym_list")]
    show_profile_sym_list: bool,
    /// Show the information of each section in the sample profile. The flag is
    /// only usable when the sample profile is in extbinary format
    #[structopt(long = "show_section_info_only")]
    show_section_info_only: bool,
    /// Turn on debug logging
    #[structopt(long)]
    debug: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct MergeCommand {
    /// Input files to merge
    #[structopt(name = "<filename...>", long = "input", short = 'i')]
    input: Vec<PathBuf>,
    /// Output file
    #[structopt(long = "output", short = 'o')]
    output: PathBuf,
    /// List of weights and filenames in `<weight>,<filename>` format
    #[structopt(long = "weighted-input", value_parser=try_parse_weighted)]
    weighted_input: Vec<(u64, String)>,
    /// Number of merge threads to use (will autodetect by default)
    #[structopt(long = "num-threads", short = 'j')]
    jobs: Option<usize>,
    /// Turn on debug logging
    #[structopt(long)]
    debug: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct OverlapCommand {
    #[structopt(name = "<base profile file>")]
    base_file: PathBuf,
    #[structopt(name = "<test profile file>")]
    test_file: PathBuf,
    #[structopt(long = "output", short = 'o')]
    output: Option<PathBuf>,
    /// For context sensitive counts
    #[structopt(long = "cs")]
    context_sensitive_counts: bool,
    /// Function level overlap information for every function in test profile with max count value
    /// greater than the parameter value
    #[structopt(long = "value-cutoff")]
    value_cutoff: Option<usize>,
    /// Function level overlap information for matching functions
    #[structopt(long = "function")]
    function: Option<String>,
    /// Generate a sparse profile
    #[structopt(long = "sparse")]
    sparse: bool,
    /// Turn on debug logging
    #[structopt(long)]
    debug: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Parser)]
pub struct Opts {
    #[command(subcommand)]
    cmd: Command,
}

impl Opts {
    fn debug(&self) -> bool {
        match &self.cmd {
            &Command::Show { ref show } => show.debug,
            &Command::Merge { ref merge } => merge.debug,
            &Command::Overlap { ref overlap } => overlap.debug,
        }
    }
}

fn try_parse_weighted(input: &str) -> Result<(u64, String), String> {
    if !input.contains(',') {
        Ok((1, input.to_string()))
    } else {
        let parts = input.split(',').collect::<Vec<_>>();
        if parts.len() != 2 {
            Err("Unexpected weighting format, expected $weight,$name or just $name".to_string())
        } else {
            let weight = parts[0]
                .parse()
                .map_err(|e| format!("Invalid weight: {}", e))?;
            if weight < 1 {
                Err("Weight must be positive integer".to_string())
            } else {
                Ok((weight, parts[1].to_string()))
            }
        }
    }
}

fn check_function(name: Option<&String>, pattern: Option<&String>) -> bool {
    match pattern {
        Some(pat) => name.map(|x| x.contains(pat)).unwrap_or(false),
        None => false,
    }
}

#[derive(Clone, Debug, Eq)]
struct HotFn {
    name: String,
    count: u64,
}

impl PartialOrd for HotFn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HotFn {
    fn cmp(&self, other: &Self) -> Ordering {
        // Do the reverse here
        other.count.cmp(&self.count)
    }
}

impl PartialEq for HotFn {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl ShowCommand {
    pub fn run(&self) -> Result<()> {
        let profile = parse(&self.input)?;
        let mut summary = ProfileSummary::new();

        let is_ir_instr = profile.is_ir_level_profile();
        let mut hotties =
            BinaryHeap::<HotFn>::with_capacity(self.topn.unwrap_or_default() as usize);
        let mut shown_funcs = 0;
        let mut below_cutoff_funcs = 0;
        let topn = self.topn.unwrap_or_default();
        for func in profile.records() {
            if func.name.is_none() || func.hash.is_none() {
                continue;
            }
            if is_ir_instr && func.has_cs_flag() != self.showcs {
                continue;
            }
            let show =
                self.all_functions || check_function(func.name.as_ref(), self.function.as_ref());

            if show && self.text {
                // TODO text format dump
                continue;
            }
            summary.add_record(&func.record);

            let (func_max, func_sum) = func.counts().iter().fold((0, 0u64), |acc, x| {
                (*x.max(&acc.0), acc.1.saturating_add(*x))
            });
            if func_max < self.value_cutoff {
                below_cutoff_funcs += 1;
                if self.only_list_below {
                    println!(
                        "  {}: (Max = {} Sum = {})",
                        func.name.as_ref().unwrap(),
                        func_max,
                        func_sum
                    );
                    continue;
                }
            } else if self.only_list_below {
                continue;
            }
            if topn > 0 {
                if hotties.len() == topn {
                    let top = hotties.peek().unwrap();
                    if top.count < func_max {
                        hotties.pop();
                        hotties.push(HotFn {
                            name: func.name.as_ref().unwrap().to_string(),
                            count: func_max,
                        });
                    }
                } else {
                    hotties.push(HotFn {
                        name: func.name.as_ref().unwrap().to_string(),
                        count: func_max,
                    });
                }
            }
            if show {
                if shown_funcs == 0 {
                    println!("Counters:");
                }
                shown_funcs += 1;
                println!("  {}:", func.name.as_ref().unwrap());
                println!("    Hash: {:#018x}", func.hash.unwrap());
                println!("    Counters: {}", func.counts().len());
                if !is_ir_instr {
                    let counts = if func.counts().is_empty() {
                        0
                    } else {
                        func.counts()[0]
                    };
                    println!("    Function count: {}", counts);
                }
                if self.show_counts {
                    let start = if is_ir_instr { 0 } else { 1 };
                    let counts = func
                        .counts()
                        .iter()
                        .skip(start)
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    println!("    Block counts: [{}]", counts);
                }
            }
        }
        if profile.get_level() == InstrumentationLevel::Ir {
            println!(
                "Instrumentation level: {}  entry_first = {}",
                profile.get_level(),
                // NOTE: in llvm 11 this is always false
                profile.is_entry_first() as usize
            );
        } else {
            println!("Instrumentation level: {}", profile.get_level());
        }
        if self.all_functions || self.function.is_some() {
            println!("Functions shown: {}", shown_funcs);
        }
        println!("Total functions: {}", summary.num_functions());
        if self.value_cutoff > 0 {
            println!(
                "Number of functions with maximum count (< {} ): {}",
                self.value_cutoff, below_cutoff_funcs
            );
            println!(
                "Number of functions with maximum count (>= {}): {}",
                self.value_cutoff,
                summary.num_functions() - below_cutoff_funcs
            );
        }
        println!("Maximum function count: {}", summary.max_function_count());
        println!(
            "Maximum internal block count: {}",
            summary.max_internal_block_count()
        );
        if let Some(topn) = self.topn {
            println!(
                "Top {} functions with the largest internal block counts: ",
                topn
            );
            let hotties = hotties.into_sorted_vec();
            for f in hotties.iter() {
                println!("  {}, max count = {}", f.name, f.count);
            }
        }

        if self.show_detailed_summary {
            println!("Total number of blocks: ?");
            println!("Total count: ?");
        }
        Ok(())
    }
}

impl MergeCommand {
    fn run(&self) -> Result<()> {
        assert!(
            !self.input.is_empty(),
            "No input files selected. See merge --help"
        );
        let profile = merge_profiles(&self.input)?;
        // Now to write it out?
        println!("{:#?}", profile);
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
    if opts.debug() {
        let _ = enable_debug_logging();
    }
    match opts.cmd {
        Command::Show { show } => show.run(),
        Command::Merge { merge } => merge.run(),
        _ => {
            panic!("Unsupported command");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_arg_parsing() {
        // Examples taken from LLVM docs
        let foo_10 = "10,foo.profdata";
        let bar_1 = "1,bar.profdata";

        assert_eq!(
            Ok((10, "foo.profdata".to_string())),
            try_parse_weighted(foo_10)
        );
        assert_eq!(
            Ok((1, "bar.profdata".to_string())),
            try_parse_weighted(bar_1)
        );
        assert_eq!(
            Ok((1, "foo.profdata".to_string())),
            try_parse_weighted("foo.profdata")
        );
        assert!(try_parse_weighted("foo.profdata,1").is_err());
        assert!(try_parse_weighted("1,1,foo.profdata").is_err());
    }
}
