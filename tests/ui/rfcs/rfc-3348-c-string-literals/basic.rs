//@ run-pass
//@ edition: 2021

fn main() {
    assert_eq!(b"test\0", c"test".to_bytes_with_nul());
}

// ferrocene-annotations: fls_u1ghcy16emve
// C String Literals
//
// ferrocene-annotations: fls_p090c5otnelw
// Simple C String Literals
