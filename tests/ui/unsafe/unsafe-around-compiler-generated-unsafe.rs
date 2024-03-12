//@ edition:2018

#![deny(unused_unsafe)]

fn main() {
    let _ = async {
        unsafe { async {}.await; } //~ ERROR unnecessary `unsafe`
    };

    // `format_args!` expands with a compiler-generated unsafe block
    unsafe { println!("foo"); } //~ ERROR unnecessary `unsafe`
}

// ferrocene-annotations: fls_aadan19t5006
// Async Blocks
//
// ferrocene-annotations: fls_hyrbmfmf6r8g
// Await Expressions
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
