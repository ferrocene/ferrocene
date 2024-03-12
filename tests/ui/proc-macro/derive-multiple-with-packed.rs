//@ check-pass

#[derive(Clone, Copy)]
#[derive(Debug)] // OK, even if `Copy` is in the different `#[derive]`
#[derive(PartialEq)] // OK too
#[repr(packed)]
struct CacheRecordHeader {
    field: u64,
}

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
