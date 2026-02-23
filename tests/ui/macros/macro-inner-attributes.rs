#![feature(rustc_attrs)]

macro_rules! test { ($nm:ident,
                     #[$a:meta],
                     $i:item) => (mod $nm { #![$a] $i }); }

test!(a, //~ NOTE: found an item that was configured out
      #[cfg(false)], //~ NOTE: the item is gated here
      pub fn bar() { });

test!(b,
      #[cfg(not(FALSE))],
      pub fn bar() { });

#[rustc_dummy]
fn main() {
    a::bar(); //~ ERROR: cannot find module or crate `a`
    //~^ NOTE: use of unresolved module or unlinked crate `a`
    b::bar();
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
