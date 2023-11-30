// This test ensures that or patterns require binding mode consistency across arms.

#![allow(non_camel_case_types)]
fn main() {
    // One level:
    let (Ok(a) | Err(ref a)): Result<&u8, u8> = Ok(&0);
    //~^ ERROR variable `a` is bound inconsistently
    let (Ok(ref mut a) | Err(a)): Result<u8, &mut u8> = Ok(0);
    //~^ ERROR variable `a` is bound inconsistently
    let (Ok(ref a) | Err(ref mut a)): Result<&u8, &mut u8> = Ok(&0);
    //~^ ERROR variable `a` is bound inconsistently
    //~| ERROR mismatched types
    let (Ok((ref a, b)) | Err((ref mut a, ref b))) = Ok((0, &0));
    //~^ ERROR variable `a` is bound inconsistently
    //~| ERROR variable `b` is bound inconsistently
    //~| ERROR mismatched types

    // Two levels:
    let (Ok(Ok(a) | Err(a)) | Err(ref a)) = Err(0);
    //~^ ERROR variable `a` is bound inconsistently

    // Three levels:
    let (Ok([Ok((Ok(ref a) | Err(a),)) | Err(a)]) | Err(a)) = Err(&1);
    //~^ ERROR variable `a` is bound inconsistently
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
