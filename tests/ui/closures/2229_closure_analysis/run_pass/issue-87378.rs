//@ edition:2021
//@ check-pass

union Union {
    value: u64,
}

fn main() {
    let u = Union { value: 42 };

    let c = || {
       unsafe { u.value }
    };

    c();
}

// ferrocene-annotations: fls_fmdn7n7s413d
// Union Type
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
