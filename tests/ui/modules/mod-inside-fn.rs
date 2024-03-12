//@ run-pass

fn f() -> isize {
    mod m {
        pub fn g() -> isize { 720 }
    }

    m::g()
}

pub fn main() {
    assert_eq!(f(), 720);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
