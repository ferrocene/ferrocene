//! Empty symbolication strategy used to compile for platforms that have no
//! support.

use core::ffi::c_void;
use crate::types::BytesOrWideString;
use crate::SymbolName;
use crate::symbolize::ResolveWhat;

pub unsafe fn resolve(_addr: ResolveWhat, _cb: &mut FnMut(&super::Symbol)) {
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

    #[cfg(feature = "std")]
    pub fn filename(&self) -> Option<&::std::path::Path> {
        None
    }

    pub fn lineno(&self) -> Option<u32> {
        None
    }
}
