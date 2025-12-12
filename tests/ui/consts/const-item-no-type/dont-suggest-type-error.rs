fn main() {
    const FOO = "hello" + 1;
    //~^ ERROR cannot add `{integer}` to `&str`
    //~| ERROR missing type for `const` item
    println!("{}", FOO);
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
