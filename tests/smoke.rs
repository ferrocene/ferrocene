#![allow(unconditional_recursion)] // FIXME rust-lang/rust#26165

extern crate backtrace;
extern crate libc;

use std::str;

#[test]
fn smoke() {
    a();
    #[inline(never)] fn a() { b() }
    #[inline(never)] fn b() { c() }
    #[inline(never)] fn c() { test() }

    #[inline(never)]
    fn test() {
        let mut v = Vec::new();
        backtrace::trace(&mut |cx| {
            v.push((cx.ip(), cx.symbol_address()));
            true
        });

        if v.len() < 5 {
            assert!(cfg!(not(feature = "libunwind")));
            assert!(cfg!(not(feature = "unix-backtrace")));
            return
        }

        assert_frame(v[0], backtrace::trace as usize, "::trace::");
        assert_frame(v[1], test as usize, "::test::");
        assert_frame(v[2], c as usize, "::c::");
        assert_frame(v[3], b as usize, "::b::");
        assert_frame(v[4], a as usize, "::a::");
        assert_frame(v[5], smoke as usize, "smoke::");
    }

    fn assert_frame((ip, sym): (*mut libc::c_void, *mut libc::c_void),
                    actual_fn_pointer: usize,
                    name: &str) {
        let ip = ip as usize;
        let sym = sym as usize;
        assert!(ip >= sym);
        assert!(sym >= actual_fn_pointer);
        assert!(sym - actual_fn_pointer < 1024);
        let outer_sym = sym as *mut libc::c_void;

        let mut resolved = false;
        backtrace::resolve(outer_sym, &mut |sym| {
            resolved = true;
            if let Some(bytes) = sym.name() {
                let bytes = str::from_utf8(bytes).unwrap();
                let mut demangled = String::new();
                backtrace::demangle(&mut demangled, bytes).unwrap();;
                assert!(demangled.contains(name),
                        "didn't find `{}` in `{}`", name, demangled);
            } else {
                // linux dladdr doesn't work so hot here
                assert!(cfg!(target_os = "linux") ||
                        cfg!(not(feature = "dladdr")));
            }

            if let Some(addr) = sym.addr() {
                assert!(outer_sym as usize >= addr as usize);
            } else {
                assert!(cfg!(not(feature = "dladdr")));
            }
        });

        assert!(resolved || cfg!(not(feature = "dladdr")));
    }
}
