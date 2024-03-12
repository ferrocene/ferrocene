//~ ERROR

//@ build-fail
//@ ignore-32bit

struct S<T> {
    x: [T; !0],
}

pub fn f() -> usize {
    std::mem::size_of::<S<u8>>()
}

fn main() {
    let x = f();
}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
