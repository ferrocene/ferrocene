//@ aux-build:two_macros.rs

mod foo {
    extern crate two_macros;
    pub use self::two_macros::m as panic;
}

mod m1 {
    use crate::foo::panic; // ok
    fn f() { panic!(); }
}

mod m2 {
    use crate::foo::*;
    fn f() { panic!(); } //~ ERROR ambiguous
}

mod m3 {
    ::two_macros::m!(use crate::foo::panic;);
    fn f() { panic!(); } //~ ERROR ambiguous
}

mod m4 {
    macro_rules! panic { () => {} } // ok
    panic!();
}

mod m5 {
    macro_rules! m { () => {
        macro_rules! panic { () => {} }
    } }
    m!();
    panic!(); //~ ERROR `panic` is ambiguous
}

#[macro_use(n)]
extern crate two_macros;
mod bar {
    pub use two_macros::m as n;
}

mod m6 {
    use crate::bar::n; // ok
    n!();
}

mod m7 {
    use crate::bar::*;
    n!(); //~ ERROR ambiguous
}

fn main() {}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
