// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::{mem, slice};

use types::{BytesOrWideString, c_void};
use libc::{self, Dl_info};

use SymbolName;

pub struct Symbol {
    inner: Dl_info,
}

impl Symbol {
    pub fn name(&self) -> Option<SymbolName> {
        if self.inner.dli_sname.is_null() {
            None
        } else {
            let ptr = self.inner.dli_sname as *const u8;
            unsafe {
                let len = libc::strlen(self.inner.dli_sname);
                Some(SymbolName::new(slice::from_raw_parts(ptr, len)))
            }
        }
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        Some(self.inner.dli_saddr as *mut _)
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        None
    }

    pub fn lineno(&self) -> Option<u32> {
        None
    }
}

pub unsafe fn resolve(addr: *mut c_void, cb: &mut FnMut(&super::Symbol)) {
    let mut info: super::Symbol = super::Symbol {
        inner: Symbol {
            inner: mem::zeroed(),
        },
    };
    if libc::dladdr(addr as *mut _, &mut info.inner.inner) != 0 {
        cb(&info)
    }
}
