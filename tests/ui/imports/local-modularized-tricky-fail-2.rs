//@ edition: 2015
// Crate-local macro expanded `macro_export` macros cannot be accessed with module-relative paths.

macro_rules! define_exported { () => {
    #[macro_export]
    macro_rules! exported {
        () => ()
    }
}}

define_exported!();

mod m {
    use exported;
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted
}

fn main() {
    ::exported!();
    //~^ ERROR macro-expanded `macro_export` macros from the current crate cannot
    //~| WARN this was previously accepted
}

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
