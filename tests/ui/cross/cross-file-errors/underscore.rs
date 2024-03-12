//@ ignore-test (auxiliary, used by other tests)
#![crate_type = "lib"]

macro_rules! underscore {
    () => (
        _
    )
}

// ferrocene-annotations: fls_ujig607lmwbm
// Attribute crate_type
