//@ run-pass
#![allow(unused_assignments)]

// Drop works for union itself.

// FIXME(static_mut_refs): this could use an atomic
#![allow(static_mut_refs)]

use std::mem::ManuallyDrop;

struct S;

union U {
    a: ManuallyDrop<S>
}

impl Drop for S {
    fn drop(&mut self) {
        unsafe { CHECK += 10; }
    }
}

impl Drop for U {
    fn drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

static mut CHECK: u8 = 0;

fn main() {
    unsafe {
        let mut u = U { a: ManuallyDrop::new(S) };
        assert_eq!(CHECK, 0);
        u = U { a: ManuallyDrop::new(S) };
        assert_eq!(CHECK, 1); // union itself is assigned, union is dropped, field is not dropped
        *u.a = S;
        assert_eq!(CHECK, 11); // union field is assigned, field is dropped
    }
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
