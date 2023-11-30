#[track_caller] //~ ERROR `main` function is not allowed to be
fn main() {
    panic!("{}: oh no", std::panic::Location::caller());
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
