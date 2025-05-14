#![feature(rustc_attrs)]

macro_rules! test { ($nm:ident,
                     #[$a:meta],
                     $i:item) => (mod $nm { #[$a] $i }); }

test!(a,
      #[cfg(false)],
      pub fn bar() { });

test!(b,
      #[cfg(not(FALSE))],
      pub fn bar() { });

// test1!(#[bar])
#[rustc_dummy]
fn main() {
    a::bar(); //~ ERROR cannot find function `bar` in module `a`
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
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
