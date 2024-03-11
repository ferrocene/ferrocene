//@ run-pass

macro_rules! mtester {
    ($l:literal) => {
        &format!("macro caught literal: {}", $l)
    };
    ($e:expr) => {
        &format!("macro caught expr: {}", $e)
    };
}

macro_rules! two_negative_literals {
    ($l1:literal $l2:literal) => {
        &format!("macro caught literals: {}, {}", $l1, $l2)
    };
}

macro_rules! only_expr {
    ($e:expr) => {
        &format!("macro caught expr: {}", $e)
    };
}

#[allow(unused_macro_rules)]
macro_rules! mtester_dbg {
    ($l:literal) => {
        &format!("macro caught literal: {:?}", $l)
    };
    ($e:expr) => {
        &format!("macro caught expr: {:?}", $e)
    };
}

macro_rules! catch_range {
    ($s:literal ..= $e:literal) => {
        &format!("macro caught literal: {} ..= {}", $s, $e)
    };
    (($s:expr) ..= ($e:expr)) => { // Must use ')' before '..='
        &format!("macro caught expr: {} ..= {}", $s, $e)
    };
}

macro_rules! pat_match {
    ($s:literal ..= $e:literal) => {
        match 3 {
            $s ..= $e => "literal, in range",
            _ => "literal, other",
        }
    };
    ($s:pat) => {
        match 3 {
            $s => "pat, single",
            _ => "pat, other",
        }
    };
}

macro_rules! match_attr {
    (#[$attr:meta] $e:literal) => {
        "attr matched literal"
    };
    (#[$attr:meta] $e:expr) => {
        "attr matched expr"
    };
}

macro_rules! match_produced_attr {
    ($lit: literal) => {
        // Struct with doc comment passed via $literal
        #[doc = $lit]
        struct LiteralProduced;
    };
    ($expr: expr) => {
        struct ExprProduced;
    };
}

macro_rules! test_user {
    ($s:literal, $e:literal) => {
        {
            let mut v = Vec::new();
            for i in $s .. $e {
                v.push(i);
            }
            "literal"
        }
    };
    ($s:expr, $e: expr) => {
        {
            let mut v = Vec::new();
            for i in $s .. $e {
                v.push(i);
            }
            "expr"
        }
    };
}

pub fn main() {
    // Cases where 'literal' catches
    assert_eq!(mtester!("str"), "macro caught literal: str");
    assert_eq!(mtester!(2), "macro caught literal: 2");
    assert_eq!(mtester!(2.2), "macro caught literal: 2.2");
    assert_eq!(mtester!(1u32), "macro caught literal: 1");
    assert_eq!(mtester!(0x32), "macro caught literal: 50");
    assert_eq!(mtester!('c'), "macro caught literal: c");
    assert_eq!(mtester!(-1.2), "macro caught literal: -1.2");
    assert_eq!(two_negative_literals!(-2 -3), "macro caught literals: -2, -3");
    assert_eq!(catch_range!(2 ..= 3), "macro caught literal: 2 ..= 3");
    assert_eq!(match_attr!(#[attr] 1), "attr matched literal");
    assert_eq!(test_user!(10, 20), "literal");
    assert_eq!(mtester!(false), "macro caught literal: false");
    assert_eq!(mtester!(true), "macro caught literal: true");
    match_produced_attr!("a");
    let _a = LiteralProduced;
    assert_eq!(pat_match!(1 ..= 3), "literal, in range");
    assert_eq!(pat_match!(4 ..= 6), "literal, other");

    // Cases where 'expr' catches
    assert_eq!(mtester!((-1.2)), "macro caught expr: -1.2");
    assert_eq!(only_expr!(-1.2), "macro caught expr: -1.2");
    assert_eq!(mtester!((1 + 3)), "macro caught expr: 4");
    assert_eq!(mtester_dbg!(()), "macro caught expr: ()");
    assert_eq!(catch_range!((1 + 1) ..= (2 + 2)), "macro caught expr: 2 ..= 4");
    assert_eq!(match_attr!(#[attr] (1 + 2)), "attr matched expr");
    assert_eq!(test_user!(10, (20 + 2)), "expr");

    match_produced_attr!((3 + 2));
    let _b = ExprProduced;

    // Cases where 'pat' matched
    assert_eq!(pat_match!(3), "pat, single");
    assert_eq!(pat_match!(6), "pat, other");
}

// ferrocene-annotations: fls_jkab8eevzbte
// Boolean Literals
//
// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
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
// ferrocene-annotations: fls_hv9jtycp0o1y
// Numeric Literals
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_94a8v54bufn8
// Values
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
