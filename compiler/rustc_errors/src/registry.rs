use rustc_data_structures::fx::FxHashMap;

use crate::ErrCode;

#[derive(Debug)]
pub struct InvalidErrorCode;

#[derive(Clone)]
pub struct Registry {
    long_descriptions: FxHashMap<ErrCode, &'static str>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry { long_descriptions: crate::codes::DIAGNOSTICS.iter().copied().collect() }
    }

    /// Returns `InvalidErrorCode` if the code requested does not exist in the
    /// registry.
    pub fn try_find_description(&self, code: ErrCode) -> Result<&'static str, InvalidErrorCode> {
        self.long_descriptions.get(&code).copied().ok_or(InvalidErrorCode)
    }
}
