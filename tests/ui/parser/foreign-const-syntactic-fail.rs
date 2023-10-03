// Syntactically, a `const` item inside an `extern { ... }` block is not allowed.

fn main() {}

#[cfg(FALSE)]
extern "C" {
    const A: isize; //~ ERROR extern items cannot be `const`
    const B: isize = 42; //~ ERROR extern items cannot be `const`
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
