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
use std::ffi::{CStr, OsStr};
use std::path::Path;
use std::os::unix::prelude::*;
use std::ptr;
use libc::getpid;

use {Symbol, SymbolName};

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
struct CSTypeRef {
    cpp_data: *const c_void,
    cpp_obj: *const c_void
}

const CS_NOW: u64 = 0x80000000;
const CSREF_NULL: CSTypeRef = CSTypeRef {
    cpp_data: 0 as *const c_void,
    cpp_obj: 0 as *const c_void,
};

#[repr(C)]
struct Info {
    path: *const c_char,
    lineno: u32,
    name: *const c_char,
    addr: *mut c_void,
}

impl Symbol for Info {
    fn name(&self) -> Option<SymbolName> {
        if self.name.is_null() {
            None
        } else {
            Some(SymbolName::new(unsafe {
                CStr::from_ptr(self.name).to_bytes()
            }))
        }
    }

    fn addr(&self) -> Option<*mut c_void> {
        Some(self.addr)
    }

    fn filename(&self) -> Option<&Path> {
        if self.path.is_null() {
            None
        } else {
            Some(Path::new(OsStr::from_bytes(unsafe {
                CStr::from_ptr(self.path).to_bytes()
            })))
        }
    }

    fn lineno(&self) -> Option<u32> {
        if self.lineno == 0 {
            None
        } else {
            Some(self.lineno)
        }
    }
}

#[link(name="CoreSymbolication", kind="framework")]
extern {
    fn CSSymbolicatorCreateWithPid(pid: c_int) -> CSTypeRef;
    fn CSRelease(rf: CSTypeRef);
    fn CSSymbolicatorGetSymbolWithAddressAtTime(
        cs: CSTypeRef, addr: *const c_void, time: u64) -> CSTypeRef;
    fn CSSymbolicatorGetSourceInfoWithAddressAtTime(
        cs: CSTypeRef, addr: *const c_void, time: u64) -> CSTypeRef;
    fn CSSourceInfoGetLineNumber(info: CSTypeRef) -> c_int;
    fn CSSourceInfoGetPath(info: CSTypeRef) -> *const c_char;
    fn CSSourceInfoGetSymbol(info: CSTypeRef) -> CSTypeRef;
    fn CSSymbolGetName(sym: CSTypeRef) -> *const c_char;
    fn CSSymbolGetSymbolOwner(sym: CSTypeRef) -> CSTypeRef;
    fn CSSymbolOwnerGetBaseAddress(symowner: CSTypeRef) -> *mut c_void;
}

pub fn resolve(addr: *mut c_void, cb: &mut FnMut(&Symbol)) {
    unsafe {
        let cs = CSSymbolicatorCreateWithPid(getpid());
        if cs == CSREF_NULL {
            return;
        }

        let info = CSSymbolicatorGetSourceInfoWithAddressAtTime(cs, addr, CS_NOW);
        let sym = if info == CSREF_NULL {
            CSSymbolicatorGetSymbolWithAddressAtTime(cs, addr, CS_NOW)
        } else {
            CSSourceInfoGetSymbol(info)
        };

        if sym == CSREF_NULL {
            return;
        }

        let owner = CSSymbolGetSymbolOwner(sym);
        if owner == CSREF_NULL {
            return;
        }

        cb(&Info {
            path: if info != CSREF_NULL {
                CSSourceInfoGetPath(info)
            } else {
                ptr::null()
            },
            lineno: if info != CSREF_NULL {
                CSSourceInfoGetLineNumber(info) as u32
            } else {
                0
            },
            name: CSSymbolGetName(sym),
            addr: CSSymbolOwnerGetBaseAddress(owner),
        });

        CSRelease(cs);
    }
}
