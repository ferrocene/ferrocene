//@ run-pass
#![allow(dead_code)]
//@ ignore-wasm32-bare no libc
//@ ignore-sgx no libc

#![feature(rustc_private)]

extern crate libc;

type DWORD = u32;
type HANDLE = *mut u8;

#[cfg(windows)]
extern "system" {
    fn GetStdHandle(which: DWORD) -> HANDLE;
    fn CloseHandle(handle: HANDLE) -> i32;
}

#[cfg(windows)]
fn close_stdout() {
    const STD_OUTPUT_HANDLE: DWORD = -11i32 as DWORD;
    unsafe { CloseHandle(GetStdHandle(STD_OUTPUT_HANDLE)); }
}

#[cfg(not(windows))]
fn close_stdout() {
    unsafe { libc::close(1); }
}

fn main() {
    close_stdout();
    println!("hello");
    println!("world");
}

// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
//
// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
