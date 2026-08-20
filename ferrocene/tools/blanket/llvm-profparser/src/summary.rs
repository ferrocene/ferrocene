/// The amount to scale cutoffs by to go back to a more readable percentile
pub const CUTOFF_SCALE: u64 = 10_000_000;

/// These are the default cutoffs for profile summary entries. Without cutoffs specified manually
/// llvm-profdata and associated tools will use these. These numbers represent percentiles of
/// counts in the profile data scaled by 10_000_000 (divide by these scales to go back to
/// percentiles)
pub const DEFAULT_CUTOFFS: [u64; 16] = [
    10000, 100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000, 950000, 990000,
    999000, 999900, 999990, 999999,
];

/// The type of the profile summary
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    /// Instrumentation profile
    Instr,
    /// Context Sensitive Instrumentation profile
    CsInstr,
    /// Sample based profile
    Sample,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProfileSummaryEntry {
    pub cutoff: u64,
    pub min_count: u64,
    pub num_counts: u64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProfileSummary {
    pub kind: Kind,
    pub total_count: u64,
    pub max_count: u64,
    pub max_internal_count: u64,
    pub max_function_count: u64,
    pub num_counts: u32,
    pub num_fns: u32,
    pub partial: bool,
    pub partial_profile_ratio: f64,
    pub detailed_summary: Vec<ProfileSummaryEntry>,
}
