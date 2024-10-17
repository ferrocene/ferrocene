//@ build-fail
//@ only-x86_64
//@ compile-flags: -Zmir-opt-level=0

fn main() {
    Bug::V([0; !0]); //~ ERROR are too big for the target
}

enum Bug {
    V([u8; !0]),
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
