// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{BTreeMap, BTreeSet};

pub use serde_json;

#[derive(Debug, Eq, PartialEq, serde_derive::Deserialize, serde_derive::Serialize)]
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

    pub fn to_qualified_fn_list(&self) -> QualifiedFnList {
        let mut a = self.symbols.iter().map(|f| f.qualified_name.clone()).collect::<Vec<_>>();
        a.sort_unstable();
        QualifiedFnList(a)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde_derive::Deserialize, serde_derive::Serialize)]
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

impl From<SerdeFunction> for Function {
    fn from(SerdeFunction(qualified_name, filename, start_line, end_line): SerdeFunction) -> Self {
        Self { qualified_name, filename, start_line, end_line }
    }
}

/// A single certified function, identified by its span
#[derive(Clone, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(from = "Function")]
pub struct SerdeFunction(String, String, usize, usize);

impl From<Function> for SerdeFunction {
    fn from(func: Function) -> Self {
        Self(func.qualified_name, func.filename, func.start_line, func.end_line)
    }
}

#[derive(PartialEq, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct QualifiedFnList(Vec<String>);
