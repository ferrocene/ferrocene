// Regression test for #83621.

extern "C" {
    static x: _; //~ ERROR: [E0121]
}

fn main() {}

// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
