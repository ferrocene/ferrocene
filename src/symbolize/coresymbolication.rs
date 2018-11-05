// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(bad_style)]

use core::mem;
use core::ptr;
use core::slice;
use core::sync::atomic::ATOMIC_USIZE_INIT;

use libc::{self, Dl_info, c_char, c_int};

use SymbolName;
use dylib::Dylib;
use dylib::Symbol as DylibSymbol;
use types::{BytesOrWideString, c_void};

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct CSTypeRef {
    cpp_data: *const c_void,
    cpp_obj: *const c_void
}

const CS_NOW: u64 = 0x80000000;
const CSREF_NULL: CSTypeRef = CSTypeRef {
    cpp_data: 0 as *const c_void,
    cpp_obj: 0 as *const c_void,
};

pub enum Symbol {
    Core {
        path: *const c_char,
        lineno: u32,
        name: *const c_char,
        addr: *mut c_void,
    },
    Dladdr(Dl_info),
}

impl Symbol {
    pub fn name(&self) -> Option<SymbolName> {
        let name = match *self {
            Symbol::Core { name, .. } => name,
            Symbol::Dladdr(ref info) => info.dli_sname,
        };
        if name.is_null() {
            None
        } else {
            Some(SymbolName::new(unsafe {
                let len = libc::strlen(name);
                slice::from_raw_parts(name as *const u8, len)
            }))
        }
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        match *self {
            Symbol::Core { addr, .. } => Some(addr),
            Symbol::Dladdr(ref info) => Some(info.dli_saddr as *mut _),
        }
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        match *self {
            Symbol::Core { path, .. } => {
                if path.is_null() {
                    None
                } else {
                    Some(BytesOrWideString::Bytes(unsafe {
                        let len = libc::strlen(path);
                        slice::from_raw_parts(path as *const u8, len)
                    }))
                }
            }
            Symbol::Dladdr(_) => None,
        }
    }

    pub fn lineno(&self) -> Option<u32> {
        match *self {
            Symbol::Core { lineno: 0, .. } => None,
            Symbol::Core { lineno, .. } => Some(lineno),
            Symbol::Dladdr(_) => None,
        }
    }
}

static CORESYMBOLICATION: Dylib = Dylib { init: ATOMIC_USIZE_INIT };

macro_rules! dlsym {
    (extern {
        $(fn $name:ident($($arg:ident: $t:ty),*) -> $ret:ty;)*
    }) => ($(
        static $name: ::dylib::Symbol<unsafe extern fn($($t),*) -> $ret> =
            ::dylib::Symbol {
                name: concat!(stringify!($name), "\0"),
                addr: ::core::sync::atomic::ATOMIC_USIZE_INIT,
                _marker: ::core::marker::PhantomData,
            };
    )*)
}

dlsym! {
    extern {
        fn CSSymbolicatorCreateWithPid(pid: c_int) -> CSTypeRef;
        fn CSRelease(rf: CSTypeRef) -> c_void;
        fn CSSymbolicatorGetSymbolWithAddressAtTime(
            cs: CSTypeRef, addr: *const c_void, time: u64) -> CSTypeRef;
        fn CSSymbolicatorGetSourceInfoWithAddressAtTime(
            cs: CSTypeRef, addr: *const c_void, time: u64) -> CSTypeRef;
        fn CSSourceInfoGetLineNumber(info: CSTypeRef) -> c_int;
        fn CSSourceInfoGetPath(info: CSTypeRef) -> *const c_char;
        fn CSSourceInfoGetSymbol(info: CSTypeRef) -> CSTypeRef;
        fn CSSymbolGetMangledName(sym: CSTypeRef) -> *const c_char;
        fn CSSymbolGetSymbolOwner(sym: CSTypeRef) -> CSTypeRef;
        fn CSSymbolOwnerGetBaseAddress(symowner: CSTypeRef) -> *mut c_void;
    }
}

unsafe fn get<T>(sym: &DylibSymbol<T>) -> &T {
    CORESYMBOLICATION.get(sym).unwrap()
}

unsafe fn try_resolve(addr: *mut c_void, cb: &mut FnMut(&super::Symbol)) -> bool {
    let path = "/System/Library/PrivateFrameworks/CoreSymbolication.framework\
                /Versions/A/CoreSymbolication\0";
    if !CORESYMBOLICATION.init(path) {
        return false;
    }

    let cs = get(&CSSymbolicatorCreateWithPid)(libc::getpid());
    if cs == CSREF_NULL {
        return false
    }

    let info = get(&CSSymbolicatorGetSourceInfoWithAddressAtTime)(
        cs, addr, CS_NOW);
    let sym = if info == CSREF_NULL {
        get(&CSSymbolicatorGetSymbolWithAddressAtTime)(cs, addr, CS_NOW)
    } else {
        get(&CSSourceInfoGetSymbol)(info)
    };

    let mut rv = false;
    if sym != CSREF_NULL {
        let owner = get(&CSSymbolGetSymbolOwner)(sym);
        if owner != CSREF_NULL {
            cb(&super::Symbol {
                inner: Symbol::Core {
                    path: if info != CSREF_NULL {
                        get(&CSSourceInfoGetPath)(info)
                    } else {
                        ptr::null()
                    },
                    lineno: if info != CSREF_NULL {
                        get(&CSSourceInfoGetLineNumber)(info) as u32
                    } else {
                        0
                    },
                    name: get(&CSSymbolGetMangledName)(sym),
                    addr: get(&CSSymbolOwnerGetBaseAddress)(owner),
                },
            });
            rv = true;
        }
    }
    get(&CSRelease)(cs);

    rv
}

pub unsafe fn resolve(addr: *mut c_void, cb: &mut FnMut(&super::Symbol)) {
    if try_resolve(addr, cb) {
        return
    }
    let mut info: Dl_info = mem::zeroed();
    if libc::dladdr(addr as *mut _, &mut info) != 0 {
        cb(&super::Symbol {
            inner: Symbol::Dladdr(info),
        });
    }
}
