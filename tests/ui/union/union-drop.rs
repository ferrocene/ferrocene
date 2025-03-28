//@ run-pass

#![allow(dead_code)]
#![allow(unused_variables)]
// FIXME(static_mut_refs): this could use an atomic
#![allow(static_mut_refs)]

// Drop works for union itself.

#[derive(Copy, Clone)]
struct S;

union U {
    a: u8
}

union W {
    a: S,
}

union Y {
    a: S,
}

impl Drop for U {
    fn drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

impl Drop for W {
    fn drop(&mut self) {
        unsafe { CHECK += 1; }
    }
}

static mut CHECK: u8 = 0;

fn main() {
    unsafe {
        assert_eq!(CHECK, 0);
        {
            let u = U { a: 1 };
        }
        assert_eq!(CHECK, 1); // 1, dtor of U is called
        {
            let w = W { a: S };
        }
        assert_eq!(CHECK, 2); // 2, dtor of W is called
        {
            let y = Y { a: S };
        }
        assert_eq!(CHECK, 2); // 2, Y has no dtor
        {
            let u2 = U { a: 1 };
            std::mem::forget(u2);
        }
        assert_eq!(CHECK, 2); // 2, dtor of U *not* called for u2
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
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
