extern "C" {
    const fn foo();
    //~^ ERROR functions in `extern` blocks cannot have qualifiers
    const unsafe fn bar();
    //~^ ERROR functions in `extern` blocks cannot have qualifiers
    //~| ERROR functions in `extern` blocks cannot have qualifiers
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
