// Syntactically, a foreign static may not have a body.

fn main() {}

extern "C" {
    static X: u8 = 0; //~ ERROR incorrect `static` inside `extern` block
    static mut Y: u8 = 0; //~ ERROR incorrect `static` inside `extern` block
}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
