//@ run-pass
#[derive(Debug)]
struct Foo(isize, isize);

pub fn main() {
    let x = Foo(1, 2);
    let Foo(y, z) = x;
    println!("{} {}", y, z);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    let x = Foo(1, 2);
    match x {
        Foo(a, b) => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            println!("{} {}", a, b);
        }
    }

    let x = Foo(1, 2);
    assert_eq!(format!("{x:?}"), "Foo(1, 2)");
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
