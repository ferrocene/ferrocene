//! Tests that coroutines that cannot return or unwind don't have unnecessary
//! panic branches.

//@ compile-flags: -C panic=abort
//@ no-prefer-dynamic

#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

struct HasDrop;

impl Drop for HasDrop {
    fn drop(&mut self) {}
}

fn callee() {}

// EMIT_MIR coroutine_tiny.main-{closure#0}.StateTransform.diff
fn main() {
    let _gen = #[coroutine]
    |_x: u8| {
        let _d = HasDrop;
        loop {
            yield;
            callee();
        }
    };
}
<<<<<<< ferrocene/main:tests/mir-opt/coroutine_tiny.rs

// ferrocene-annotations: um_rustc_C_panic
||||||| b5d1746e7d2:tests/mir-opt/coroutine_tiny.rs
=======

// CHECK-NOT: panic
// CHECK-NOT: cleanup
>>>>>>> rust-lang/rust/HEAD--generated-by-pull-upstream:tests/mir-opt/coroutine/coroutine_tiny.rs
