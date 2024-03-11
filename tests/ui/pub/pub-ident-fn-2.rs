//@ run-rustfix

pub foo(_s: usize) { bar() }
//~^ ERROR missing `fn` for function definition

fn bar() {}

fn main() {
    foo(2);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
