//@ build-pass (FIXME(62277): could be check-pass?)
#![allow(dead_code)]

use std::mem::ManuallyDrop;

enum A<'a, T: 'a>
where
    Self: Send, T: PartialEq<Self>
{
    Foo(&'a Self),
    Bar(T),
}

struct B<'a, T: 'a>
where
    Self: Send, T: PartialEq<Self>
{
    foo: &'a Self,
    bar: T,
}

union C<'a, T: 'a>
where
    Self: Send, T: PartialEq<Self>
{
    foo: &'a Self,
    bar: ManuallyDrop<T>,
}

union D<'a, T: 'a>
where
    Self: Send, T: PartialEq<Self> + Copy
{
    foo: &'a Self,
    bar: T,
}

fn main() {}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
