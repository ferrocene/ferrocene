//! related issue: <https://github.com/rust-lang/rust/issues/40313>
//@ compile-flags: -Zwrite-long-types-to-disk=yes
use std::cell::Cell;
use std::panic::catch_unwind;
fn main() {
    let mut x = Cell::new(22);
    catch_unwind(|| { x.set(23); });
    //~^ ERROR the type `UnsafeCell<i32>` may contain interior mutability and a
}

// ferrocene-annotations: fls_omaq7psg83n3
// Interior Mutability
//
// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
