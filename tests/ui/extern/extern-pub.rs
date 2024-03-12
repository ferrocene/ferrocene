//@ run-pass
//@ pretty-expanded FIXME #23616

extern "C" {
    pub fn free(p: *const u8);
}

pub fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
