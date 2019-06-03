// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Common support for resolving with `dladdr`, often used as a fallback if
//! other strategies don't work.

#![allow(dead_code)]

cfg_if! {
    if #[cfg(all(unix, not(target_os = "emscripten"), feature = "dladdr"))] {
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

            #[cfg(feature = "std")]
            pub fn filename(&self) -> Option<&::std::path::Path> {
                None
            }

            pub fn lineno(&self) -> Option<u32> {
                None
            }
        }

        pub unsafe fn resolve(addr: *mut c_void, cb: &mut FnMut(Symbol)) {
            let mut info = Symbol {
                inner: mem::zeroed(),
            };
            if libc::dladdr(addr as *mut _, &mut info.inner) != 0 {
                cb(info)
            }
        }
    } else {
        use types::{BytesOrWideString, c_void};
        use symbolize::SymbolName;

        pub enum Symbol {}

        impl Symbol {
            pub fn name(&self) -> Option<SymbolName> {
                match *self {}
            }

            pub fn addr(&self) -> Option<*mut c_void> {
                match *self {}
            }

            pub fn filename_raw(&self) -> Option<BytesOrWideString> {
                match *self {}
            }

            #[cfg(feature = "std")]
            pub fn filename(&self) -> Option<&::std::path::Path> {
                match *self {}
            }

            pub fn lineno(&self) -> Option<u32> {
                match *self {}
            }
        }

        pub unsafe fn resolve(addr: *mut c_void, cb: &mut FnMut(Symbol)) {
            drop((addr, cb));
        }
    }
}
