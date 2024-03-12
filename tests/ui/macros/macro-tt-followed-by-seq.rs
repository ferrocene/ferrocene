//@ run-pass
// Regression test for issue #25436: permit token-trees to be followed
// by sequences, enabling more general parsing.

use self::Join::*;

#[derive(Debug)]
#[allow(dead_code)]
enum Join<A,B> {
  Keep(A,B),
  Skip(A,B),
}

macro_rules! parse_list {
  ( < $a:expr; > $($b:tt)* ) => { Keep(parse_item!($a),parse_list!($($b)*)) };
  ( $a:tt $($b:tt)* ) => { Skip(parse_item!($a), parse_list!($($b)*)) };
  ( ) => { () };
}

macro_rules! parse_item {
  ( $x:expr ) => { $x }
}

fn main() {
    let list = parse_list!(<1;> 2 <3;> 4);
    assert_eq!("Keep(1, Skip(2, Keep(3, Skip(4, ()))))",
               format!("{:?}", list));
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
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
