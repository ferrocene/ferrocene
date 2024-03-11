//@ build-pass
//@ compile-flags: --crate-type=rlib --emit=llvm-ir -Cno-prepopulate-passes

// This is a variant of issue-91050-1.rs -- see there for an explanation.

pub mod before {
    extern "C" {
        pub static GLOBAL1: [u8; 1];
    }

    pub unsafe fn do_something_with_array() -> u8 {
        GLOBAL1[0]
    }
}

pub mod inner {
    extern "C" {
        pub static GLOBAL1: u8;
    }

    pub unsafe fn call() -> u8 {
        GLOBAL1 + 42
    }
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
