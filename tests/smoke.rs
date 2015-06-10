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

        assert!(v.len() > 5);

        for &(ip, sym) in &v {
            println!("{:?} {:?} {:#x}", ip, sym, test as usize);
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
        backtrace::resolve(outer_sym, &mut |sym| {
            if let Some(bytes) = sym.name() {
                let bytes = str::from_utf8(bytes).unwrap();
                let mut demangled = String::new();
                backtrace::demangle(&mut demangled, bytes).unwrap();;
                assert!(demangled.contains(name),
                        "didn't find `{}` in `{}`", name, demangled);
            }

            if let Some(addr) = sym.addr() {
                assert!(outer_sym as usize >= addr as usize);
            }
        });
    }
}
