struct Foo {
    foo: Vec<u32>,
}

impl Copy for Foo { } //~ ERROR may not be implemented for this type

#[derive(Copy)] //~ ERROR may not be implemented for this type
struct Foo2<'a> {
    ty: &'a mut bool,
}

enum EFoo {
    Bar { x: Vec<u32> },
    Baz,
}

impl Copy for EFoo { } //~ ERROR may not be implemented for this type

#[derive(Copy)] //~ ERROR may not be implemented for this type
enum EFoo2<'a> {
    Bar(&'a mut bool),
    Baz,
}

fn main() {
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
