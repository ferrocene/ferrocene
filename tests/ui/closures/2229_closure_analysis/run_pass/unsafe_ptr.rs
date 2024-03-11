//@ edition:2021
//@ run-pass

// Test that we can use raw ptrs when using `capture_disjoint_fields`.

#![allow(dead_code)]

#[derive(Debug)]
struct S {
    s: String,
    t: String,
}

struct T(*const S);

fn unsafe_imm() {
    let s = "".into();
    let t = "".into();
    let my_speed: Box<S> = Box::new(S { s, t });

    let p : *const S = Box::into_raw(my_speed);
    let t = T(p);

    let c = || unsafe {
        println!("{:?}", (*t.0).s);
    };

    c();
}

fn unsafe_mut() {
    let s = "".into();
    let t = "".into();
    let mut my_speed: Box<S> = Box::new(S { s, t });
    let p : *mut S = &mut *my_speed;

    let c = || {
        let x = unsafe { &mut (*p).s };
        *x = "s".into();
    };
    c();
}

fn main() {
    unsafe_mut();
    unsafe_imm();
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Type
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
