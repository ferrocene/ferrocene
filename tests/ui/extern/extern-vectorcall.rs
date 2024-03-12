//@ run-pass
//@ revisions: x64 x32
//@ [x64]only-x86_64
//@ [x32]only-x86

#![feature(abi_vectorcall)]

trait A {
    extern "vectorcall" fn test1(i: i32);
}

struct S;

impl A for S {
    extern "vectorcall" fn test1(i: i32) {
        assert_eq!(i, 1);
    }
}

extern "vectorcall" fn test2(i: i32) {
    assert_eq!(i, 2);
}

fn main() {
    <S as A>::test1(1);
    test2(2);
}

// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
