//@ build-fail

//@ error-pattern: are too big for the target architecture
fn main() {
    println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
