//@ run-pass
// Issue #825: Should recheck the loop condition after continuing
pub fn main() {
    let mut i = 1;
    while i > 0 {
        assert!((i > 0));
        println!("{}", i);
        i -= 1;
        continue;
    }
}

// ferrocene-annotations: fls_sjwrlwvpulp
// Continue Expressions
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
