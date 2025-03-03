//@ run-rustfix
//@ edition:2018
//@ check-pass
#![warn(rust_2021_prelude_collisions)]

trait TryIntoU32 {
    fn try_into(self) -> Result<u32, ()>;
}

impl TryIntoU32 for u8 {
    fn try_into(self) -> Result<u32, ()> {
        Ok(self as u32)
    }
}

// needed for autoref test
impl TryIntoU32 for &f32 {
    fn try_into(self) -> Result<u32, ()> {
        Ok(*self as u32)
    }
}

trait TryFromU8: Sized {
    fn try_from(x: u8) -> Result<Self, ()>;
}

impl TryFromU8 for u32 {
    fn try_from(x: u8) -> Result<Self, ()> {
        Ok(x as u32)
    }
}

impl TryIntoU32 for *const u16 {
    fn try_into(self) -> Result<u32, ()> {
        Ok(unsafe { *self } as u32)
    }
}

trait FromByteIterator {
    fn from_iter<T>(iter: T) -> Self
    where
        T: Iterator<Item = u8>;
}

impl FromByteIterator for Vec<u8> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        iter.collect()
    }
}

fn main() {
    // test dot-call that will break in 2021 edition
    let _: u32 = 3u8.try_into().unwrap();
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    // test associated function call that will break in 2021 edition
    let _ = u32::try_from(3u8).unwrap();
    //~^ WARNING trait-associated function `try_from` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    // test reverse turbofish too
    let _ = <Vec<u8>>::from_iter(vec![1u8, 2, 3, 4, 5, 6].into_iter());
    //~^ WARNING trait-associated function `from_iter` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    // negative testing lint (this line should *not* emit a warning)
    let _: u32 = TryFromU8::try_from(3u8).unwrap();

    // test type omission
    let _: u32 = <_>::try_from(3u8).unwrap();
    //~^ WARNING trait-associated function `try_from` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    // test autoderef
    let _: u32 = (&3u8).try_into().unwrap();
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    // test autoref
    let _: u32 = 3.0.try_into().unwrap();
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    let mut data = 3u16;
    let mut_ptr = std::ptr::addr_of_mut!(data);
    let _: u32 = mut_ptr.try_into().unwrap();
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition

    type U32Alias = u32;
    let _ = U32Alias::try_from(3u8).unwrap();
    //~^ WARNING trait-associated function `try_from` will become ambiguous in Rust 2021
    //~^^ WARNING this is accepted in the current edition
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
//
// ferrocene-annotations: fls_151r19d7xbgz
// Entities
//
// ferrocene-annotations: fls_izl8iuhoz9e0
// Scopes
//
// ferrocene-annotations: fls_6ozthochxz1i
// Binding Scopes
//
// ferrocene-annotations: fls_ftphlagzd2te
// Generic Parameter Scope
//
// ferrocene-annotations: fls_m0z7omni9hp0
// Item Scope
//
// ferrocene-annotations: fls_769b4p8v3cwu
// Label Scope
//
// ferrocene-annotations: fls_kgbi26212eof
// Self Scope
//
// ferrocene-annotations: fls_octf6sf7yso
// Textual Macro Scope
//
// ferrocene-annotations: fls_lnpyb285qdiy
// Scope Hierarchy
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_40xoego2thsp
// Resolution
//
// ferrocene-annotations: fls_i6qzga6dyaee
// Path Resolution
//
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
