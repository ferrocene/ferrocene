extern "C" unsafe {
               //~^ ERROR expected `{`, found keyword `unsafe`
               //~| ERROR extern block cannot be declared unsafe
    unsafe fn foo();
        //~^ ERROR functions in `extern` blocks cannot have qualifiers
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// unsafe Blocks
