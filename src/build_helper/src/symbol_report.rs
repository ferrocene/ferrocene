// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

/// List of symbols in the certified subset
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct Symbols(pub Vec<Function>);

/// A single certified function, identified by its span
#[derive(Clone, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(from = "SerdeFunction")]
#[serde(into = "SerdeFunction")]
pub struct Function {
    /// The fully qualified path to the function, for example:
    /// - `core::ptr::copy`
    /// - `core::cmp::impls::<impl core::cmp::PartialEq for u8>::eq`
    /// - `core::convert::num::ptr_try_from_impls::<impl core::convert::TryFrom<u128> for usize>::try_from`
    pub function_path: String,
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
}

/// Helper struct used for de- and serialiasing
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
struct SerdeFunction(String, String, usize, usize);

impl From<SerdeFunction> for Function {
    fn from(value: SerdeFunction) -> Self {
        Self { function_path: value.0, filename: value.1, start_line: value.2, end_line: value.3 }
    }
}

impl From<Function> for SerdeFunction {
    fn from(Function { function_path, filename, start_line, end_line }: Function) -> Self {
        Self(function_path, filename, start_line, end_line)
    }
}
