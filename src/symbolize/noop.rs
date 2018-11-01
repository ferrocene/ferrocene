use types::{BytesOrWideString, c_void};
use SymbolName;

pub unsafe fn resolve(_addr: *mut c_void, _cb: &mut FnMut(&super::Symbol)) {
}

pub struct Symbol;

impl Symbol {
    pub fn name(&self) -> Option<SymbolName> {
        None
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        None
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        None
    }

    pub fn lineno(&self) -> Option<u32> {
        None
    }
}
