/// List of symbols in the certified subset
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(transparent)]
pub struct Symbols(pub Vec<Function>);

/// A single certified function, identified by its span
#[derive(Clone, serde_derive::Deserialize, serde_derive::Serialize)]
#[serde(from = "SerdeFunction")]
#[serde(into = "SerdeFunction")]
pub struct Function {
    pub module_path: String,
    pub filename: String,
    pub start_line: usize,
    pub end_line: usize,
}

/// Helper struct used for de- and serialiasing
#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
struct SerdeFunction(String, String, usize, usize);

impl From<SerdeFunction> for Function {
    fn from(value: SerdeFunction) -> Self {
        Self { module_path: value.0, filename: value.1, start_line: value.2, end_line: value.3 }
    }
}

impl From<Function> for SerdeFunction {
    fn from(Function { module_path, filename, start_line, end_line }: Function) -> Self {
        Self(module_path, filename, start_line, end_line)
    }
}
