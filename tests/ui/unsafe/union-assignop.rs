use std::ops::AddAssign;
use std::mem::ManuallyDrop;

struct NonCopy;
impl AddAssign for NonCopy {
    fn add_assign(&mut self, _: Self) {}
}

union Foo {
    a: u8, // non-dropping
    b: ManuallyDrop<NonCopy>,
}

fn main() {
    let mut foo = Foo { a: 42 };
    foo.a += 5; //~ ERROR access to union field is unsafe
    *foo.b += NonCopy; //~ ERROR access to union field is unsafe
    *foo.b = NonCopy; //~ ERROR access to union field is unsafe
    foo.b = ManuallyDrop::new(NonCopy);
    foo.a; //~ ERROR access to union field is unsafe
    let foo = Foo { a: 42 };
    foo.b; //~ ERROR access to union field is unsafe
    let mut foo = Foo { a: 42 };
    foo.b = foo.b;
    //~^ ERROR access to union field is unsafe
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_290jmzfh7x4e
// Compound Assignment Expressions
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
