// RFC 401 test extracted into distinct file. This is because some the
// change to suppress "derived" errors wound up suppressing this error
// message, since the fallback for `3` doesn't occur.

fn main() {
    let _ = 3 as bool;
    //~^ ERROR cannot cast as `bool`
}

// ferrocene-annotations: fls_tiqp1gxf116z
// Bool Type
