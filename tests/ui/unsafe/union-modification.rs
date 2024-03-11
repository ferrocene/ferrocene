//@ run-pass
union Foo {
    bar: i8,
    _blah: isize,
    _zst: (),
}

struct FooHolder {
    inner_foo: Foo
}

fn do_nothing(_x: &mut Foo) {}

pub fn main() {
    let mut foo = Foo { bar: 5 };
    do_nothing(&mut foo);
    foo.bar = 6;
    unsafe { foo.bar += 1; }
    assert_eq!(unsafe { foo.bar }, 7);
    unsafe {
        let Foo { bar: inner } = foo;
        assert_eq!(inner, 7);
    }

    let foo = Foo { bar: 5 };
    let foo = if let 3 = if let true = true { 3 } else { 4 } { foo } else { foo };

    let (_foo2, _random) = (foo, 42);

    let mut foo_holder = FooHolder { inner_foo: Foo { bar: 5 } };
    foo_holder.inner_foo.bar = 4;
    assert_eq!(unsafe { foo_holder.inner_foo.bar }, 4);
    drop(foo_holder);
}

// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_290jmzfh7x4e
// Compound Assignment Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
