//@ run-pass

macro_rules! compiles_fine {
    (#[$at:meta]) => {
        // test that the different types of attributes work
        #[attribute]
        /// Documentation!
        #[$at]

        // check that the attributes are recognised by requiring this
        // to be removed to avoid a compile error
        #[cfg(FALSE)]
        static MISTYPED: () = "foo";
    }
}

// item
compiles_fine!(#[foo]);

pub fn main() {
    // statement
    compiles_fine!(#[bar]);
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
