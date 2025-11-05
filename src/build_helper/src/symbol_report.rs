use std::collections::{BTreeMap, BTreeSet};

#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
pub struct SymbolReport {
    pub symbols: Vec<Function>,
    pub annotations: BTreeMap<String, BTreeSet<(usize, usize)>>,
}

impl SymbolReport {
    pub fn add_annotation(&mut self, filename: String, start: usize, end: usize) {
        self.annotations.entry(filename).or_default().insert((start, end));
    }

    pub fn new() -> Self {
        Self { symbols: Vec::new(), annotations: BTreeMap::new() }
    }
}

/// A single certified function, identified by its span
#[derive(Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Function {
    /// The fully qualified path to the function, for example:
    /// - `core::ptr::copy`
    /// - `core::cmp::impls::<impl core::cmp::PartialEq for u8>::eq`
    /// - `core::convert::num::ptr_try_from_impls::<impl core::convert::TryFrom<u128> for usize>::try_from`
    pub qualified_name: String,
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
}
