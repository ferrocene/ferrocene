// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::{c_void, c_char, c_int};
use std::mem;
use std::ffi::CStr;

use {Symbol, SymbolName};

#[repr(C)]
struct Dl_info {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

impl Symbol for Dl_info {
    fn name(&self) -> Option<SymbolName> {
        if self.dli_sname.is_null() {
            None
        } else {
            Some(SymbolName::new(unsafe {
                CStr::from_ptr(self.dli_sname).to_bytes()
            }))
        }
    }
    fn addr(&self) -> Option<*mut c_void> {
        Some(self.dli_saddr)
    }
}

extern {
    fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int;
}

pub fn resolve(addr: *mut c_void, cb: &mut FnMut(&Symbol)) {
    let mut info: Dl_info = unsafe { mem::zeroed() };
    if unsafe { dladdr(addr, &mut info) != 0 } {
        cb(&info)
    }
}
