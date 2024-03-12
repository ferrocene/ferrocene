//@ run-pass
// Testing guarantees provided by once functions.



use std::sync::Arc;

fn foo<F:FnOnce()>(blk: F) {
    blk();
}

pub fn main() {
    let x = Arc::new(true);
    foo(move|| {
        assert!(*x);
        drop(x);
    });
}

// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_8gpcpvc99pxj
// Call Conformance
