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
use core::slice;
use core::char;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
    }
}

use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::um::processthreadsapi;
use winapi::um::dbghelp;
use winapi::um::dbghelp::*;

use SymbolName;
use types::BytesOrWideString;

// Store an OsString on std so we can provide the symbol name and filename.
pub struct Symbol {
    name: Option<&'static [u8]>,
    addr: *mut c_void,
    line: Option<u32>,
    filename: Option<(*mut u16, usize)>,
}

impl Symbol {
    pub fn name(&self) -> Option<SymbolName> {
        self.name.map(SymbolName::new)
    }

    pub fn addr(&self) -> Option<*mut c_void> {
        Some(self.addr as *mut _)
    }

    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        self.filename.map(|(ptr, len)| {
            unsafe {
                BytesOrWideString::Wide(slice::from_raw_parts(ptr, len))
            }
        })
    }

    pub fn lineno(&self) -> Option<u32> {
        self.line
    }
}

#[cfg(feature = "std")]
impl Symbol {
    pub fn filename(&self) -> Option<&OsString> {
        self.filename_cache.as_ref()
    }
}

#[repr(C, align(8))]
struct Aligned8<T>(T);

static mut NAME_BUFFER: [u8; 256] = [0; 256];

pub unsafe fn resolve(addr: *mut c_void, cb: &mut FnMut(&super::Symbol)) {
    const SIZE: usize = 2 * MAX_SYM_NAME + mem::size_of::<SYMBOL_INFOW>();
    let mut data = Aligned8([0u8; SIZE]);
    let data = &mut data.0;
    let info = &mut *(data.as_mut_ptr() as *mut SYMBOL_INFOW);
    info.MaxNameLen = MAX_SYM_NAME as ULONG;
    // the struct size in C.  the value is different to
    // `size_of::<SYMBOL_INFOW>() - MAX_SYM_NAME + 1` (== 81)
    // due to struct alignment.
    info.SizeOfStruct = 88;

    let _c = ::dbghelp_init();

    let mut displacement = 0u64;
    let ret = dbghelp::SymFromAddrW(processthreadsapi::GetCurrentProcess(),
                                    addr as DWORD64,
                                    &mut displacement,
                                    info);
    if ret != TRUE {
        return
    }

    // If the symbol name is greater than MaxNameLen, SymFromAddrW will
    // give a buffer of (MaxNameLen - 1) characters and set NameLen to
    // the real value.
    let name_len = ::core::cmp::min(info.NameLen as usize,
                                    info.MaxNameLen as usize - 1);
    let name_ptr = info.Name.as_ptr() as *const u16;
    let name = slice::from_raw_parts(name_ptr, name_len);

    // Reencode the utf-16 symbol to utf-8 so we can use `SymbolName::new` like
    // all other platforms
    let mut name_overflow = false;
    let mut name_len = 0;
    {
        let mut remaining = &mut NAME_BUFFER[..];
        for c in char::decode_utf16(name.iter().cloned()) {
            let c = c.unwrap_or(char::REPLACEMENT_CHARACTER);
            let len = c.len_utf8();
            if len < remaining.len() {
                c.encode_utf8(remaining);
                let tmp = remaining;
                remaining = &mut tmp[len..];
                name_len += len;
            } else {
                name_overflow = true;
                break
            }
        }
    }
    let name = if name_overflow { None } else { Some(&NAME_BUFFER[..name_len]) };

    let mut line = mem::zeroed::<IMAGEHLP_LINEW64>();
    line.SizeOfStruct = mem::size_of::<IMAGEHLP_LINEW64>() as DWORD;
    let mut displacement = 0;
    let ret = dbghelp::SymGetLineFromAddrW64(processthreadsapi::GetCurrentProcess(),
                                             addr as DWORD64,
                                             &mut displacement,
                                             &mut line);

    let mut filename = None;
    let mut lineno = None;
    if ret == TRUE {
        lineno = Some(line.LineNumber as u32);

        let base = line.FileName;
        let mut len = 0;
        while *base.offset(len) != 0 {
            len += 1;
        }

        let len = len as usize;

        filename = Some((base, len));
    }


    cb(&super::Symbol {
        inner: Symbol {
            name,
            addr: info.Address as *mut _,
            line: lineno,
            filename,
        },
    })
}
