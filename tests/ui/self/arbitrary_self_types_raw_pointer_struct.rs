//@ run-pass
#![feature(arbitrary_self_types)]

use std::rc::Rc;

struct Foo(String);

impl Foo {
    unsafe fn foo(self: *const Self) -> *const str {
        (*self).0.as_ref()
    }

    fn complicated_1(self: *const Rc<Self>) -> &'static str {
        "Foo::complicated_1"
    }

    unsafe fn complicated_2(self: Rc<*const Self>) -> *const str {
        (**self).0.as_ref()
    }
}

fn main() {
    let foo = Foo("abc123".into());
    assert_eq!("abc123", unsafe { &*(&foo as *const Foo).foo() });
    assert_eq!("Foo::complicated_1", std::ptr::null::<Rc<Foo>>().complicated_1());
    let rc = Rc::new(&foo as *const Foo);
    assert_eq!("abc123", unsafe { &*rc.complicated_2()});
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_g0uyl7qw4c7w
// Parenthesized Expressions
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
