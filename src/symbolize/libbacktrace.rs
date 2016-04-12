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

extern crate backtrace_sys as bt;

use libc::uintptr_t;
use std::os::raw::{c_void, c_char, c_int};
use std::env;
use std::ffi::CStr;
use std::path::Path;
use std::ptr;
use std::sync::{ONCE_INIT, Once};

use {Symbol, SymbolName};

type FileLine = (*const c_char, c_int);

extern fn error_cb(_data: *mut c_void, _msg: *const c_char,
                   _errnum: c_int) {
    // do nothing for now
}

extern fn syminfo_cb(data: *mut c_void,
                     pc: uintptr_t,
                     symname: *const c_char,
                     _symval: uintptr_t,
                     _symsize: uintptr_t) {
    struct SyminfoSymbol {
        pc: uintptr_t,
        symname: *const c_char,
    }
    impl Symbol for SyminfoSymbol {
        fn name(&self) -> Option<SymbolName> {
            if self.symname.is_null() {
                None
            } else {
                Some(SymbolName::new(unsafe {
                    CStr::from_ptr(self.symname).to_bytes()
                }))
            }
        }

        fn addr(&self) -> Option<*mut c_void> {
            if self.pc == 0 {None} else {Some(self.pc as *mut _)}
        }
    }
    unsafe {
        call(data, &SyminfoSymbol {
            pc: pc,
            symname: symname,
        });
    }
}

extern fn pcinfo_cb(data: *mut c_void,
                    pc: uintptr_t,
                    filename: *const c_char,
                    lineno: c_int,
                    function: *const c_char) -> c_int {
    struct PcinfoSymbol {
        pc: uintptr_t,
        filename: *const c_char,
        lineno: c_int,
        function: *const c_char,
    }
    impl Symbol for PcinfoSymbol {
        fn name(&self) -> Option<SymbolName> {
            if self.function.is_null() {
                None
            } else {
                Some(SymbolName::new(unsafe {
                    CStr::from_ptr(self.function).to_bytes()
                }))
            }
        }

        fn addr(&self) -> Option<*mut c_void> {
            if self.pc == 0 {None} else {Some(self.pc as *mut _)}
        }

        fn filename(&self) -> Option<&[u8]> {
            Some(unsafe { CStr::from_ptr(self.filename).to_bytes() })
        }

        fn lineno(&self) -> Option<u32> {
            Some(self.lineno as u32)
        }
    }

    unsafe {
        if filename.is_null() || function.is_null() {
            return -1
        }
        call(data, &PcinfoSymbol {
            pc: pc,
            filename: filename,
            lineno: lineno,
            function: function,
        });
        return 0
    }
}

unsafe fn call(data: *mut c_void, sym: &Symbol) {
    let cb = data as *mut &mut FnMut(&Symbol);
    let mut bomb = ::Bomb { enabled: true };
    (*cb)(sym);
    bomb.enabled = false;
}

// The libbacktrace API supports creating a state, but it does not
// support destroying a state. I personally take this to mean that a
// state is meant to be created and then live forever.
//
// I would love to register an at_exit() handler which cleans up this
// state, but libbacktrace provides no way to do so.
//
// With these constraints, this function has a statically cached state
// that is calculated the first time this is requested. Remember that
// backtracing all happens serially (one global lock).
//
// An additional oddity in this function is that we initialize the
// filename via self_exe_name() to pass to libbacktrace. It turns out
// that on Linux libbacktrace seamlessly gets the filename of the
// current executable, but this fails on freebsd. by always providing
// it, we make sure that libbacktrace never has a reason to not look up
// the symbols. The libbacktrace API also states that the filename must
// be in "permanent memory", so we copy it to a static and then use the
// static as the pointer.
unsafe fn init_state() -> *mut bt::backtrace_state {
    static mut STATE: *mut bt::backtrace_state = 0 as *mut _;
    static mut LAST_FILENAME: [c_char; 256] = [0; 256];
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| {
        let selfname = if cfg!(target_os = "freebsd") ||
                          cfg!(target_os = "dragonfly") ||
                          cfg!(target_os = "bitrig") ||
                          cfg!(target_os = "openbsd") {
            env::current_exe().ok()
        } else {
            None
        };
        let filename = match selfname.as_ref().and_then(|p| path2bytes(p)) {
            Some(bytes) => {
                if bytes.len() < LAST_FILENAME.len() {
                    let i = bytes.iter();
                    for (slot, val) in LAST_FILENAME.iter_mut().zip(i) {
                        *slot = *val as c_char;
                    }
                    LAST_FILENAME.as_ptr()
                } else {
                    ptr::null()
                }
            }
            None => ptr::null(),
        };
        // Our libbacktrace may not have multithreading support, so
        // set `threaded = 0` and synchronize ourselves.
        STATE = bt::backtrace_create_state(filename, 0, error_cb,
                                           ptr::null_mut());
    });

    STATE
}

#[cfg(unix)]
fn path2bytes(p: &Path) -> Option<&[u8]> {
    use std::os::unix::prelude::*;
    Some(p.as_os_str().as_bytes())
}

#[cfg(windows)]
fn path2bytes(p: &Path) -> Option<&[u8]> {
    p.to_str().map(|s| s.as_bytes())
}

pub fn resolve(symaddr: *mut c_void, mut cb: &mut FnMut(&Symbol)) {
    let _guard = ::lock::lock();

    // backtrace errors are currently swept under the rug
    unsafe {
        let state = init_state();
        if state.is_null() {
            return
        }

        let ret = bt::backtrace_pcinfo(state, symaddr as uintptr_t,
                                       pcinfo_cb, error_cb,
                                       &mut cb as *mut _ as *mut _);
        if ret != 0 {
            bt::backtrace_syminfo(state, symaddr as uintptr_t,
                                  syminfo_cb, error_cb,
                                  &mut cb as *mut _ as *mut _);
        }
    }
}
