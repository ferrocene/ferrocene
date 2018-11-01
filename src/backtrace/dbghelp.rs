// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
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
use core::prelude::v1::*;

use winapi::shared::minwindef::*;
use winapi::um::processthreadsapi;
use winapi::um::winnt::{self, CONTEXT};
use winapi::um::dbghelp;
use winapi::um::dbghelp::*;

use types::c_void;

pub struct Frame {
    inner: STACKFRAME64,
}

impl Frame {
    pub fn ip(&self) -> *mut c_void {
        self.inner.AddrPC.Offset as *mut _
    }

    pub fn symbol_address(&self) -> *mut c_void {
        self.ip()
    }
}

#[repr(C, align(16))] // required by `CONTEXT`, is a FIXME in winapi right now
struct MyContext(CONTEXT);

#[inline(always)]
pub unsafe fn trace(cb: &mut FnMut(&super::Frame) -> bool) {
    // Allocate necessary structures for doing the stack walk
    let process = processthreadsapi::GetCurrentProcess();
    let thread = processthreadsapi::GetCurrentThread();

    let mut context = mem::zeroed::<MyContext>();
    winnt::RtlCaptureContext(&mut context.0);
    let mut frame = super::Frame {
        inner: Frame { inner: mem::zeroed() },
    };
    let image = init_frame(&mut frame.inner.inner, &context.0);

    // Initialize this process's symbols
    let _c = ::dbghelp_init();

    // And now that we're done with all the setup, do the stack walking!
    while dbghelp::StackWalk64(image as DWORD,
                               process,
                               thread,
                               &mut frame.inner.inner,
                               &mut context.0 as *mut CONTEXT as *mut _,
                               None,
                               Some(dbghelp::SymFunctionTableAccess64),
                               Some(dbghelp::SymGetModuleBase64),
                               None) == TRUE {
        if frame.inner.inner.AddrPC.Offset == frame.inner.inner.AddrReturn.Offset ||
            frame.inner.inner.AddrPC.Offset == 0 ||
                frame.inner.inner.AddrReturn.Offset == 0 {
                    break
                }

        if !cb(&frame) {
            break
        }
    }
}

#[cfg(target_arch = "x86_64")]
fn init_frame(frame: &mut STACKFRAME64, ctx: &CONTEXT) -> WORD {
    frame.AddrPC.Offset = ctx.Rip as u64;
    frame.AddrPC.Mode = AddrModeFlat;
    frame.AddrStack.Offset = ctx.Rsp as u64;
    frame.AddrStack.Mode = AddrModeFlat;
    frame.AddrFrame.Offset = ctx.Rbp as u64;
    frame.AddrFrame.Mode = AddrModeFlat;
    winnt::IMAGE_FILE_MACHINE_AMD64
}

#[cfg(target_arch = "x86")]
fn init_frame(frame: &mut STACKFRAME64, ctx: &CONTEXT) -> WORD {
    frame.AddrPC.Offset = ctx.Eip as u64;
    frame.AddrPC.Mode = AddrModeFlat;
    frame.AddrStack.Offset = ctx.Esp as u64;
    frame.AddrStack.Mode = AddrModeFlat;
    frame.AddrFrame.Offset = ctx.Ebp as u64;
    frame.AddrFrame.Mode = AddrModeFlat;
    winnt::IMAGE_FILE_MACHINE_I386
}
