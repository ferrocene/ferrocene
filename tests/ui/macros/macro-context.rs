// (typeof used because it's surprisingly hard to find an unparsed token after a stmt)
macro_rules! m {
    () => ( i ; typeof );   //~ ERROR expected expression, found reserved keyword `typeof`
                            //~| ERROR macro expansion ignores reserved keyword `typeof`
                            //~| ERROR macro expansion ignores `;`
                            //~| ERROR macro expansion ignores `;`
                            //~| ERROR cannot find type `i` in this scope
                            //~| ERROR cannot find value `i` in this scope
                            //~| WARN trailing semicolon in macro
                            //~| WARN this was previously
}

fn main() {
    let a: m!();
    let i = m!();
    match 0 {
        m!() => {}
    }

    m!();
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
