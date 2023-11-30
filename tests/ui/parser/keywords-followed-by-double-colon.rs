fn main() {
    struct::foo();
    //~^ ERROR expected identifier
}
fn bar() {
    mut::baz();
    //~^ ERROR expected expression, found keyword `mut`
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_mec5cg5aptf8
// Strict Keywords
