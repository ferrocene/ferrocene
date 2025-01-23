<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
#![feature(start)]

#[start]
#[track_caller] //~ ERROR `#[start]` function is not allowed to be `#[track_caller]`
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    panic!("{}: oh no", std::panic::Location::caller());
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
