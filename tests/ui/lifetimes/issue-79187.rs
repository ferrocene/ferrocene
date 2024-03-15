fn thing(x: impl FnOnce(&u32)) {}

fn main() {
    let f = |_| ();
    thing(f);
    //~^ ERROR implementation of `FnOnce` is not general enough
    //~| ERROR implementation of `FnOnce` is not general enough
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
