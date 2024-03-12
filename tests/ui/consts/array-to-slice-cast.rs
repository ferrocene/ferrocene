//@ check-pass

fn main() {}

const fn foo() {
    let x = [1, 2, 3, 4, 5];
    let y: &[_] = &x;

    struct Foo<T: ?Sized>(bool, T);

    let x: Foo<[u8; 3]> = Foo(true, [1, 2, 3]);
    let y: &Foo<[u8]> = &x;
}

// ferrocene-annotations: fls_xinykul167l
// Array expressions

// ferrocene-annotations: fls_uj0kpjwyld60
// Array type

// ferrocene-annotations: fls_vpbikb73dw4k
// Slice type
