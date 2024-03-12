//@ run-pass
#![allow(unreachable_code)]
pub fn main() {
    let mut x = 0;

    'foo: loop {
        'bar: loop {
            loop {
                if 1 == 2 {
                    break 'foo;
                }
                else {
                    break 'bar;
                }
            }
            continue 'foo;
        }
        x = 42;
        break;
    }

    println!("{}", x);
    assert_eq!(x, 42);
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sjwrlwvpulp
// Continue Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
