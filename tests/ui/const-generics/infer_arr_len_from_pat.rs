//@ check-pass
//
// see issue #70529

fn as_chunks<const N: usize>() -> [u8; N] {
    loop {}
}

fn main() {
    let [_, _] = as_chunks();
}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
