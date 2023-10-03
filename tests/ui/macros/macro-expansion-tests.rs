#![allow(unused_macros)]

mod macros_cant_escape_fns {
    fn f() {
        macro_rules! m { () => { 3 + 4 } }
    }
    fn g() -> i32 { m!() }
    //~^ ERROR cannot find macro
}

mod macros_cant_escape_mods {
    mod f {
        macro_rules! m { () => { 3 + 4 } }
    }
    fn g() -> i32 { m!() }
    //~^ ERROR cannot find macro
}

mod macros_can_escape_flattened_mods_test {
    #[macro_use]
    mod f {
        macro_rules! m { () => { 3 + 4 } }
    }
    fn g() -> i32 { m!() }
}

fn macro_tokens_should_match() {
    macro_rules! m { (a) => { 13 } }
    m!(a);
}

// should be able to use a bound identifier as a literal in a macro definition:
fn self_macro_parsing() {
    macro_rules! foo { (zz) => { 287; } }
    fn f(zz: i32) {
        foo!(zz);
    }
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
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
