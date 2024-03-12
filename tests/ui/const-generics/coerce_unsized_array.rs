//@ run-pass
fn foo<const N: usize>(v: &[u8; N]) -> &[u8] {
    v
}

fn main() {
    assert_eq!(foo(&[1, 2]), &[1, 2]);
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
