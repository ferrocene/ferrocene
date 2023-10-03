// Test the `.span_label(..)` to the type when there's a
// type error in a pattern due to a the formal parameter.

fn main() {}

struct Tuple(u8);

fn foo(Tuple(_): String) {} //~ ERROR mismatched types

// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
