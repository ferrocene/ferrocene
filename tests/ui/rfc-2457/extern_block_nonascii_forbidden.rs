#![feature(extern_types)]

extern "C" {
    type 一; //~ items in `extern` blocks cannot use non-ascii identifiers
    fn 二(); //~ items in `extern` blocks cannot use non-ascii identifiers
    static 三: usize; //~ items in `extern` blocks cannot use non-ascii identifiers
}

fn main() {}

// ferrocene-annotations: fls_21vnag69kbwe
// Identifiers
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
