fn main() {
    &panic!()
    //~^ ERROR mismatched types
    //~| expected unit type `()`
    //~| found reference `&_`
    //~| expected `()`, found reference
}

// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
