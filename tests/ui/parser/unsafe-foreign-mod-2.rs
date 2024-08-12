extern "C" unsafe {
    //~^ ERROR expected `{`, found keyword `unsafe`
    unsafe fn foo();
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// unsafe Blocks
