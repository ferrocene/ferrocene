//@ run-pass
//@ compile-flags: --cfg foo


#[cfg(foo)]
macro_rules! foo { () => (1) }

#[cfg(not(foo))]
macro_rules! foo { () => (2) }

pub fn main() {
    assert_eq!(foo!(), 1);
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: um_rustc_cfg
