#[track_caller(1)]
fn f() {}
//~^^ ERROR malformed `track_caller` attribute input

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
