//@ check-fail

struct DataWrapper<'a> {
    data: &'a [u8; Self::SIZE], //~ ERROR generic `Self`
}

impl DataWrapper<'_> {
    const SIZE: usize = 14;
}

pub fn main() {}

// ferrocene-annotations: fls_wdec78luqh5b
// Abstract Data Types
