#![feature(stmt_expr_attributes)]

fn main() {
    let _ = #[cfg(false)] if true {}; //~ ERROR `cfg` is not supported in this position
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
