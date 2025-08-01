//@ edition: 2015
// Regression test for #85794

struct Baz {
    inner : dyn fn ()
    //~^ ERROR expected `,`, or `}`, found keyword `fn`
    //~| ERROR expected identifier, found keyword `fn`
    //~| ERROR cannot find type `dyn` in this scope
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
