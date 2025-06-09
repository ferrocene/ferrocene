//@ build-fail

fn main() {
    println!("Size: {}", std::mem::size_of::<[u8; u64::MAX as usize]>());
    //~^ ERROR too big for the target architecture
}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
