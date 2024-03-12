//@ check-pass

#[derive(Clone, Copy)]
#[derive(Debug)] // OK, even if `Copy` is in the different `#[derive]`
#[repr(packed)]
struct CacheRecordHeader {
    field: u64,
}

fn main() {}

// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
