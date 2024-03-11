//@ run-pass
//@ compile-flags: -O

// Regression test for issue #80309

pub unsafe fn foo(x: *const i8) -> i8 {
    *x.wrapping_sub(x as _).wrapping_add(x as _)
}

fn main() {
    let x = 42;
    println!("{}", unsafe { foo(&x) });
}

// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
