#![feature(core_intrinsics)]

extern crate core;
use core::intrinsics::discriminant_value;

#[repr(usize)]
enum MyWeirdOption<T> {
//~^ ERROR parameter `T` is never used
    None = 0,
    Some = std::mem::size_of::<T>(),
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {
    assert_eq!(discriminant_value(&MyWeirdOption::<u8>::None), 0);
    assert_eq!(discriminant_value(&MyWeirdOption::<u8>::Some), 1);
}

// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
// ferrocene-annotations: fls_66m4rnbssgig
// Constant Expressions
