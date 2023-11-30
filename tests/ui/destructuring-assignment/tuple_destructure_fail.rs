const C: i32 = 1;

fn main() {
    let (mut a, mut b);
    (a, .., b, ..) = (0, 1); //~ ERROR `..` can only be used once per tuple pattern
    (a, a, b) = (1, 2); //~ ERROR mismatched types
    (C, ..) = (0,1); //~ ERROR invalid left-hand side of assignment
    (_,) = (1, 2); //~ ERROR mismatched types
}

// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Wildcard Patterns
