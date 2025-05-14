fn foo() {
    let x: isize;
    fn bar() { log(debug, x); }
    //~^ ERROR can't capture dynamic environment in a fn item
    //~| ERROR cannot find value `debug` in this scope
    //~| ERROR cannot find function `log` in this scope
}
fn main() { foo(); }

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
