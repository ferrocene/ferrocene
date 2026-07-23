//@ run-pass

extern "C" {
    pub fn free(p: *mut std::ffi::c_void);
}

pub fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
