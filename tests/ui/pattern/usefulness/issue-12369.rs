#![deny(unreachable_patterns)]

fn main() {
    let sl = vec![1,2,3];
    let v: isize = match &*sl {
        &[] => 0,
        &[a,b,c] => 3,
        &[a, ref rest @ ..] => a,
        &[10,a, ref rest @ ..] => 10 //~ ERROR: unreachable pattern
    };
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
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
