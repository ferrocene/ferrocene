// issue #500

fn main() {
    let x = true;
    let y = 1;
    let z = x + y;
    //~^ ERROR cannot add `{integer}` to `bool`
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_tiqp1gxf116z
// Bool Type
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
