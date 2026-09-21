use crate::instrumentation_profile::types::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct ProfileSummary {
    num_functions: usize,
    total_count: u64,
    max_count: u64,
    max_function_count: u64,
    max_internal_block_count: u64,
    count_frequencies: BTreeMap<u64, usize>,
}

impl ProfileSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_record(&mut self, record: &InstrProfRecord) {
        if !record.counts.is_empty() {
            self.num_functions += 1;
            self.add_count(record.counts[0]);
            if record.counts[0] > self.max_function_count {
                self.max_function_count = record.counts[0];
            }
            self.add_internal_counts(&record.counts[1..]);
        }
    }

    fn add_count(&mut self, count: u64) {
        self.total_count = self.total_count.saturating_add(count);
        if count > self.max_count {
            self.max_count = count;
        }
        self.count_frequencies
            .entry(count)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    fn add_internal_counts(&mut self, counts: &[u64]) {
        for count in counts {
            self.add_count(*count);
            if *count > self.max_internal_block_count {
                self.max_internal_block_count = *count;
            }
        }
    }

    pub fn num_functions(&self) -> usize {
        self.num_functions
    }

    pub fn max_function_count(&self) -> u64 {
        self.max_function_count
    }

    pub fn max_internal_block_count(&self) -> u64 {
        self.max_internal_block_count
    }
}
