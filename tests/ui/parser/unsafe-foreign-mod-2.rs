extern "C" unsafe {
    //~^ ERROR expected `{`, found keyword `unsafe`
    //~| ERROR extern block cannot be declared unsafe
    unsafe fn foo();
    //~^ ERROR items in unadorned `extern` blocks cannot have safety qualifiers
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// unsafe Blocks
