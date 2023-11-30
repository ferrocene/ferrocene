// FIXME(tschottdorf): This should compile. See #44912.

pub fn main() {
    let x = &Some((3, 3));
    let _: &i32 = match x {
        Some((x, 3)) | &Some((ref x, 5)) => x,
        //~^ ERROR is bound inconsistently
        _ => &5i32,
    };
}

// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
