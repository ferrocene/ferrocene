//@ run-pass
//@ edition: 2021

fn main() {
    assert_eq!(
        c"\xEF\x80🦀\u{1F980}".to_bytes_with_nul(),
        &[0xEF, 0x80, 0xF0, 0x9F, 0xA6, 0x80, 0xF0, 0x9F, 0xA6, 0x80, 0x00],
    );
}

// ferrocene-annotations: fls_u1ghcy16emve
// C String Literals
//
// ferrocene-annotations: fls_p090c5otnelw
// Simple C String Literals
