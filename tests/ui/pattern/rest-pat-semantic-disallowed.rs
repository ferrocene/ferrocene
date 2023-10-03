// Here we test that rest patterns, i.e. `..`, are not allowed
// outside of slice (+ ident patterns within those), tuple,
// and tuple struct patterns and that duplicates are caught in these contexts.

#![feature(box_patterns)]

fn main() {}

macro_rules! mk_pat {
    () => { .. } //~ ERROR `..` patterns are not allowed here
}

fn rest_patterns() {
    let mk_pat!();

    // Top level:
    fn foo(..: u8) {} //~ ERROR `..` patterns are not allowed here
    let ..;  //~ ERROR `..` patterns are not allowed here

    // Box patterns:
    let box ..;  //~ ERROR `..` patterns are not allowed here

    // In or-patterns:
    match 1 {
        1 | .. => {} //~ ERROR `..` patterns are not allowed here
    }

    // Ref patterns:
    let &..; //~ ERROR `..` patterns are not allowed here
    let &mut ..; //~ ERROR `..` patterns are not allowed here

    // Ident patterns:
    let x @ ..; //~ ERROR `..` patterns are not allowed here
    //~^ ERROR type annotations needed
    let ref x @ ..; //~ ERROR `..` patterns are not allowed here
    let ref mut x @ ..; //~ ERROR `..` patterns are not allowed here

    // Tuple:
    let (..): (u8,); // OK.
    let (..,): (u8,); // OK.
    let (
        ..,
        .., //~ ERROR `..` can only be used once per tuple pattern
        .. //~ ERROR `..` can only be used once per tuple pattern
    ): (u8, u8, u8);
    let (
        ..,
        x,
        .. //~ ERROR `..` can only be used once per tuple pattern
    ): (u8, u8, u8);

    struct A(u8, u8, u8);

    // Tuple struct (same idea as for tuple patterns):
    let A(..); // OK.
    let A(..,); // OK.
    let A(
        ..,
        .., //~ ERROR `..` can only be used once per tuple struct pattern
        .. //~ ERROR `..` can only be used once per tuple struct pattern
    );
    let A(
        ..,
        x,
        .. //~ ERROR `..` can only be used once per tuple struct pattern
    );

    // Array/Slice:
    let [..]: &[u8]; // OK.
    let [..,]: &[u8]; // OK.
    let [
        ..,
        .., //~ ERROR `..` can only be used once per slice pattern
        .. //~ ERROR `..` can only be used once per slice pattern
    ]: &[u8];
    let [
        ..,
        ref x @ .., //~ ERROR `..` can only be used once per slice pattern
        ref mut y @ .., //~ ERROR `..` can only be used once per slice pattern
        (ref z @ ..), //~ ERROR `..` patterns are not allowed here
        .. //~ ERROR `..` can only be used once per slice pattern
    ]: &[u8];
}

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
