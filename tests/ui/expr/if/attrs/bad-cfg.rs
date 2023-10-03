#![feature(stmt_expr_attributes)]

fn main() {
    let _ = #[cfg(FALSE)] if true {}; //~ ERROR removing an expression
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
