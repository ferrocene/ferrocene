enum Foo {
    A { a, b: usize }
    //~^ ERROR expected `:`, found `,`
}

fn main() {
    // no complaints about non-existing fields
    let f = Foo::A { a:3, b: 4};
    match f {
        // no complaints about non-existing fields
        Foo::A {a, b} => {}
    }
}

// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
