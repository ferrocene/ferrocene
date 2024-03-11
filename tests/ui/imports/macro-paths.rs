//@ aux-build:two_macros.rs

extern crate two_macros;

mod foo {
    pub mod bar {
        pub use two_macros::m;
    }
}

fn f() {
    use foo::*;
    bar::m! { //~ ERROR ambiguous
        mod bar { pub use two_macros::m; }
    }
}

pub mod baz {
    pub use two_macros::m;
}

fn g() {
    baz::m! { //~ ERROR ambiguous
        mod baz { pub use two_macros::m; }
    }
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
