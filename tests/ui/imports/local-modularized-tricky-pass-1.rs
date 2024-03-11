//@ build-pass (FIXME(62277): could be check-pass?)

macro_rules! define_exported { () => {
    #[macro_export]
    macro_rules! exported {
        () => ()
    }
}}

mod inner1 {
    use super::*;
    exported!();
}

mod inner2 {
    define_exported!();
}

fn main() {}

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
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
