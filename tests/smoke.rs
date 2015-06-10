#![allow(unconditional_recursion)] // FIXME rust-lang/rust#26165

extern crate backtrace;
extern crate libc;

use std::str;

#[test]
fn smoke() {
    a(line!());
    #[inline(never)] fn a(start_line: u32) { b(start_line) }
    #[inline(never)] fn b(start_line: u32) { c(start_line) }
    #[inline(never)] fn c(start_line: u32) { test(start_line) }
    #[inline(never)] fn test(start_line: u32) {
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

        assert_frame(v[0], backtrace::trace as usize, "::trace", "", 0);
        assert_frame(v[1], test as usize, "::test",
                     "tests/smoke.rs", start_line + 6);
        assert_frame(v[2], c as usize, "::c", "tests/smoke.rs", start_line + 3);
        assert_frame(v[3], b as usize, "::b", "tests/smoke.rs", start_line + 2);
        assert_frame(v[4], a as usize, "::a", "tests/smoke.rs", start_line + 1);
        assert_frame(v[5], smoke as usize, "smoke::", "", 0);
    }

    fn assert_frame((ip, sym): (*mut libc::c_void, *mut libc::c_void),
                    actual_fn_pointer: usize,
                    expected_name: &str,
                    expected_file: &str,
                    expected_line: u32) {
        let ip = ip as usize;
        let sym = sym as usize;
        assert!(ip >= sym);
        assert!(sym >= actual_fn_pointer);
        assert!(sym - actual_fn_pointer < 1024);

        let mut resolved = 0;
        let libbacktrace = cfg!(feature = "libbacktrace") &&
                           !cfg!(target_os = "macos");
        let dladdr = cfg!(feature = "dladdr");
        let can_resolve = dladdr || libbacktrace;

        let mut name = None;
        let mut addr = None;
        let mut line = None;
        let mut file = None;
        backtrace::resolve(ip as *mut libc::c_void, &mut |sym| {
            resolved += 1;
            name = sym.name().map(|v| v.to_vec());
            addr = sym.addr();
            line = sym.lineno();
            file = sym.filename().map(|v| v.to_vec());
        });

        match resolved {
            0 => return assert!(!can_resolve),
            _ => {}
        }

        // linux dladdr doesn't work, but everything else should
        if can_resolve && cfg!(not(target_os = "linux")) {
            let bytes = name.expect("didn't find a name");
            let bytes = str::from_utf8(&bytes).unwrap();
            let mut demangled = String::new();
            backtrace::demangle(&mut demangled, bytes).unwrap();;
            assert!(demangled.contains(expected_name),
                    "didn't find `{}` in `{}`", expected_name, demangled);
        }

        if can_resolve {
            addr.expect("didn't find a symbol");
        }

        if libbacktrace && cfg!(debug_assertions) {
            let line = line.expect("didn't find a line number");
            let file = file.expect("didn't find a line number");
            if !expected_file.is_empty() {
                assert_eq!(str::from_utf8(&file).unwrap(), expected_file);
            }
            if expected_line != 0 {
                assert!(line == expected_line,
                        "bad line number on frame for `{}`: {} != {}",
                        expected_name, line, expected_line);
            }
        }
    }
}
