//@ run-pass

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    Unit,
    Tuple(u32),
    Struct { field: u32 }
}

fn main() {
    let enum_unit = NonExhaustiveEnum::Unit;

    match enum_unit {
        NonExhaustiveEnum::Unit => "first",
        NonExhaustiveEnum::Tuple(_) => "second",
        NonExhaustiveEnum::Struct { .. } => "third",
    };
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
// ferrocene-annotations: fls_xgqh0ju6bmbn
// Patterns
