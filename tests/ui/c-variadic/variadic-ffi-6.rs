#![crate_type="lib"]
#![feature(c_variadic)]

pub unsafe extern "C" fn use_vararg_lifetime(
    x: usize,
    y: ...
) -> &usize { //~ ERROR missing lifetime specifier
    &0
}

pub unsafe extern "C" fn use_normal_arg_lifetime(x: &usize, y: ...) -> &usize { // OK
    x
}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
