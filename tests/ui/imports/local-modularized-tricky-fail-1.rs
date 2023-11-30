#![feature(decl_macro)]

macro_rules! define_exported { () => {
    #[macro_export]
    macro_rules! exported {
        () => ()
    }
}}
macro_rules! define_panic { () => {
    #[macro_export]
    macro_rules! panic {
        () => ()
    }
}}
macro_rules! define_include { () => {
    #[macro_export]
    macro_rules! include {
        () => ()
    }
}}

use inner1::*;

mod inner1 {
    pub macro exported() {}
}

exported!(); //~ ERROR `exported` is ambiguous

mod inner2 {
    define_exported!();
}

fn main() {
    panic!(); //~ ERROR `panic` is ambiguous
}

mod inner3 {
    define_panic!();
}

mod inner4 {
    define_include!();
}

include!(); //~ ERROR `include` is ambiguous

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
