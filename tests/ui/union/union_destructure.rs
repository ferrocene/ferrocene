//@ run-pass
#![allow(unreachable_patterns)]

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Pie {
    slices: u8,
    size: u8,
}

union Foo {
    #[allow(dead_code)]
    bar: i8,
    baz: Pie,
}

fn main() {
    let u = Foo { bar: 5 };
    let (Some(Foo { bar: _ }) | None) = Some(u);
    let u = Foo { bar: 6 };
    let (Some(Foo { bar: _ }) | Some(Foo { bar: _ }) | None) = Some(u);
    unsafe {
        let u = Foo { bar: 7 };
        let (Foo { bar } | Foo { bar }) = u;
        assert_eq!(bar, 7)
    }
    let u = Foo { bar: 8 };
    match Some(u) {
        Some(Foo { bar: _ }) => 3,
        None => 4,
    };

    let u = Foo { bar: 9 };
    unsafe {
        match u {
            Foo { baz: Pie { .. } } => {}
        };
    }
    let u = Foo { bar: 10 };
    unsafe {
        match u {
            Foo { baz: Pie { slices: _, size: _ } } => {}
        };
    }

    let u = Foo { bar: 11 };
    match u {
        Foo { baz: _ } => {}
    };
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
