// revisions: mir thir
// [thir]compile-flags: -Zthir-unsafeck

#[deny(unused_unsafe)]
fn main() {
    let mut v = Vec::<i32>::with_capacity(24);

    unsafe {
        let f = |v: &mut Vec<_>| {
            unsafe { //~ ERROR unnecessary `unsafe`
                v.set_len(24);
                |w: &mut Vec<u32>| { unsafe { //~ ERROR unnecessary `unsafe`
                    w.set_len(32);
                } };
            }
            |x: &mut Vec<u32>| { unsafe { //~ ERROR unnecessary `unsafe`
                x.set_len(40);
            } };
        };

        v.set_len(0);
        f(&mut v);
    }

    |y: &mut Vec<u32>| { unsafe {
        y.set_len(48);
    } };
}

// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
