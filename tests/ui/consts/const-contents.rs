//@ run-pass
// Issue #570
#![allow(non_upper_case_globals)]

static lsl : isize = 1 << 2;
static add : isize = 1 + 2;
static addf : f64 = 1.0 + 2.0;
static not : isize = !0;
static notb : bool = !true;
static neg : isize = -(1);

pub fn main() {
    assert_eq!(lsl, 4);
    assert_eq!(add, 3);
    assert_eq!(addf, 3.0);
    assert_eq!(not, -1);
    assert_eq!(notb, false);
    assert_eq!(neg, -1);
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic expressions
//
// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit expressions
//
// ferrocene-annotations: fls_wrecura8u5ar
// Negation expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
