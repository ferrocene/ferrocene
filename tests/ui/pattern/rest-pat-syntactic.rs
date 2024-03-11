// Here we test that `..` is allowed in all pattern locations *syntactically*.
// The semantic test is in `rest-pat-semantic-disallowed.rs`.

//@ check-pass

fn main() {}

macro_rules! accept_pat {
    ($p:pat) => {}
}

accept_pat!(..);

#[cfg(FALSE)]
fn rest_patterns() {
    // Top level:
    fn foo(..: u8) {}
    let ..;

    // Box patterns:
    let box ..;
    //~^ WARN box pattern syntax is experimental
    //~| WARN unstable syntax

    // In or-patterns:
    match x {
        .. | .. => {}
    }

    // Ref patterns:
    let &..;
    let &mut ..;

    // Ident patterns:
    let x @ ..;
    let ref x @ ..;
    let ref mut x @ ..;

    // Tuple:
    let (..); // This is interpreted as a tuple pattern, not a parenthesis one.
    let (..,); // Allowing trailing comma.
    let (.., .., ..); // Duplicates also.
    let (.., P, ..); // Including with things in between.

    // Tuple struct (same idea as for tuple patterns):
    let A(..);
    let A(..,);
    let A(.., .., ..);
    let A(.., P, ..);

    // Array/Slice (like with tuple patterns):
    let [..];
    let [..,];
    let [.., .., ..];
    let [.., P, ..];

    // Random walk to guard against special casing:
    match x {
        .. |
        [
            (
                box .., //~ WARN box pattern syntax is experimental
                &(..),
                &mut ..,
                x @ ..
            ),
            ref x @ ..,
        ] |
        ref mut x @ ..
        => {}
    }
    //~| WARN unstable syntax
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
