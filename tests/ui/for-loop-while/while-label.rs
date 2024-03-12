//@ run-pass
#![allow(unreachable_code)]


pub fn main() {
    let mut i = 100;
    'w: while 1 + 1 == 2 {
        i -= 1;
        if i == 95 {
            break 'w;
            panic!("Should have broken out of loop");
        }
    }
    assert_eq!(i, 95);
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
