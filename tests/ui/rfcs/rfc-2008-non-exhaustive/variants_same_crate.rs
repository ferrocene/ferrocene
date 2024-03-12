//@ run-pass

pub enum NonExhaustiveVariants {
    #[non_exhaustive] Unit,
    #[non_exhaustive] Tuple(u32),
    #[non_exhaustive] Struct { field: u32 }
}

fn main() {
    let variant_tuple = NonExhaustiveVariants::Tuple(340);
    let _variant_struct = NonExhaustiveVariants::Struct { field: 340 };

    match variant_tuple {
        NonExhaustiveVariants::Unit => "",
        NonExhaustiveVariants::Tuple(_fe_tpl) => "",
        NonExhaustiveVariants::Struct { field: _ } => ""
    };
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
// ferrocene-annotations: fls_xgqh0ju6bmbn
// Patterns
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
