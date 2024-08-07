enum T { A, B }

fn main() {
    let x = T::A;
    match x { T::B => { } } //~ ERROR non-exhaustive patterns: `T::A` not covered
    match true { //~ ERROR non-exhaustive patterns: `false` not covered
      true => {}
    }
    match Some(10) { //~ ERROR non-exhaustive patterns: `Some(_)` not covered
      None => {}
    }
    match (2, 3, 4) { //~ ERROR non-exhaustive patterns: `(_, _, i32::MIN..=3_i32)`
                      //  and `(_, _, 5_i32..=i32::MAX)` not covered
      (_, _, 4) => {}
    }
    match (T::A, T::A) { //~ ERROR non-exhaustive patterns: `(T::A, T::A)` and `(T::B, T::B)` not covered
      (T::A, T::B) => {}
      (T::B, T::A) => {}
    }
    match T::A { //~ ERROR non-exhaustive patterns: `T::B` not covered
      T::A => {}
    }
    // This is exhaustive, though the algorithm got it wrong at one point
    match (T::A, T::B) {
      (T::A, _) => {}
      (_, T::A) => {}
      (T::B, T::B) => {}
    }
    let vec = vec![Some(42), None, Some(21)];
    let vec: &[Option<isize>] = &vec;
    match *vec { //~ ERROR non-exhaustive patterns: `[]` not covered
        [Some(..), None, ref tail @ ..] => {}
        [Some(..), Some(..), ref tail @ ..] => {}
        [None] => {}
    }
    let vec = vec![1];
    let vec: &[isize] = &vec;
    match *vec {
        [_, ref tail @ ..] => (),
        [] => ()
    }
    let vec = vec![0.5f32];
    let vec: &[f32] = &vec;
    match *vec { //~ ERROR non-exhaustive patterns: `[_, _, _, _, ..]` not covered
        [0.1, 0.2, 0.3] => (),
        [0.1, 0.2] => (),
        [0.1] => (),
        [] => ()
    }
    let vec = vec![Some(42), None, Some(21)];
    let vec: &[Option<isize>] = &vec;
    match *vec {
        [Some(..), None, ref tail @ ..] => {}
        [Some(..), Some(..), ref tail @ ..] => {}
        [None, None, ref tail @ ..] => {}
        [None, Some(..), ref tail @ ..] => {}
        [Some(_)] => {}
        [None] => {}
        [] => {}
    }
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
