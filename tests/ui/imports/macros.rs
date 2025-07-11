//@ aux-build:two_macros.rs

extern crate two_macros; // two identity macros `m` and `n`

mod foo {
    pub use two_macros::n as m;
}

mod m1 {
    m!(use two_macros::*;);
    use crate::foo::m; // This shadows the glob import
}

mod m2 {
    use two_macros::*;
    m! { //~ ERROR ambiguous
        use crate::foo::m;
    }
}

mod m3 {
    use two_macros::m;
    fn f() {
        use two_macros::n as m; // This shadows the above import
        m!();
    }

    fn g() {
        m! { //~ ERROR ambiguous
            use two_macros::n as m;
        }
    }
}

mod m4 {
    macro_rules! m { () => {} }
    use two_macros::m;
    m!();
}

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
