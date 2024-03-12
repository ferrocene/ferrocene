//@ run-fail
//@ error-pattern:attempt to divide by zero
//@ ignore-emscripten no processes

#[allow(unconditional_panic)]
fn main() {
    let y = 0;
    let _z = 1 / y;
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
