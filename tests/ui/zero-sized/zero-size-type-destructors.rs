//@ run-pass
#![allow(non_upper_case_globals)]

static mut destructions : isize = 3;

pub fn foo() {
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
          unsafe { destructions -= 1 };
        }
    }

    let _x = [Foo, Foo, Foo];
}

pub fn main() {
  foo();
  assert_eq!(unsafe { destructions }, 0);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
