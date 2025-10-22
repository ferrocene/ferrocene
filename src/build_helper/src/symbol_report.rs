// use serde::{Deserialize as _, Serialize as _};

#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct Symbols(pub Vec<Function>);

#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Function {
    pub qualified_name: String,
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
}
