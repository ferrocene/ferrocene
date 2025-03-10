//@ no-prefer-dynamic
//@ compile-flags: --test -Cpanic=abort -Zpanic_abort_tests
//@ run-flags: --test-threads=1 --nocapture
//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0
//@ normalize-stdout: "finished in \d+\.\d+s" -> "finished in $$TIME"

//@ ignore-android #120567
//@ needs-subprocess

#![cfg(test)]

use std::io::Write;

#[test]
fn it_works() {
    println!("about to succeed");
    assert_eq!(1 + 1, 2);
}

#[test]
#[should_panic]
fn it_panics() {
    println!("about to panic");
    assert_eq!(1 + 1, 4);
}

#[test]
fn it_fails() {
    println!("about to fail");
    assert_eq!(1 + 1, 4);
}

#[test]
fn it_writes_to_stdio() {
    println!("hello, world");
    writeln!(std::io::stdout(), "testing123").unwrap();
    writeln!(std::io::stderr(), "testing321").unwrap();
}

// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_aes2d94g12b9
// Attribute should_panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
//
// ferrocene-annotations: um_rustc_test
//
// ferrocene-annotations: um_rustc_C_panic
