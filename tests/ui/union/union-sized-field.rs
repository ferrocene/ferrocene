use std::mem::ManuallyDrop;

union Foo<T: ?Sized> {
    value: ManuallyDrop<T>,
    //~^ ERROR the size for values of type
}

struct Foo2<T: ?Sized> {
    value: ManuallyDrop<T>,
    //~^ ERROR the size for values of type
    t: u32,
}

enum Foo3<T: ?Sized> {
    Value(ManuallyDrop<T>),
    //~^ ERROR the size for values of type
}

fn main() {}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
