//@ run-fail
//@ compile-flags: -Cdebug-assertions=n  -Coverflow-checks=y

// If not specified, overflow checks are enabled if debug-assertions are
// enabled, disabled otherwise.
// With debug-assertions=n and overflow-checks=y, there should be no a panic.

#[allow(arithmetic_overflow)]
fn main() {
    let x: i8 = 100;
    let y: i8 = 100;
    let z: i8 = x + y;
    assert_eq!(z, -56);
}

// ferrocene-annotations: um_rustc_C_overflow_checks
