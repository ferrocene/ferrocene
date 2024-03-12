// Syntactically, a foreign static may have a body.

//@ check-pass

fn main() {}

#[cfg(FALSE)]
extern "C" {
    static X: u8;
    static mut Y: u8;
}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
