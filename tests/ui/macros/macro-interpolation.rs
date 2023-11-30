// run-pass

macro_rules! overly_complicated {
    ($fnname:ident, $arg:ident, $ty:ty, $body:block, $val:expr, $pat:pat, $res:path) =>
    ({
        fn $fnname($arg: $ty) -> Option<$ty> $body
        match $fnname($val) {
          Some($pat) => {
            $res
          }
          _ => { panic!(); }
        }
    })

}

macro_rules! qpath {
    (path, <$type:ty as $trait:path>::$name:ident) => {
        <$type as $trait>::$name
    };

    (ty, <$type:ty as $trait:ty>::$name:ident) => {
        <$type as $trait>::$name
    };
}

pub fn main() {
    let _: qpath!(path, <str as ToOwned>::Owned);
    let _: qpath!(ty, <str as ToOwned>::Owned);

    assert!(overly_complicated!(f, x, Option<usize>, { return Some(x); },
                               Some(8), Some(y), y) == 8)
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
