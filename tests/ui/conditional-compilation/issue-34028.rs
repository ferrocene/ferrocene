//@ check-pass

macro_rules! m {
    () => { #[cfg(any())] fn f() {} }
}

trait T {}
impl T for () { m!(); }

fn main() {}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
