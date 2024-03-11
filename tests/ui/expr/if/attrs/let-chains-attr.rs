//@ check-pass

#![feature(let_chains)]

#[cfg(FALSE)]
fn foo() {
    #[attr]
    if let Some(_) = Some(true) && let Ok(_) = Ok(1) {
    } else if let Some(false) = Some(true) {
    }
}

fn main() {}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_u1afezy1ye99
// Conditional Compilation
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_p0t1ch115tra
// If Let Expressions
