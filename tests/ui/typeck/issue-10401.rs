fn main() {
    let mut a = "a";
    a += { "b" };
    //~^ ERROR: binary assignment operation `+=` cannot be applied
}

// ferrocene-annotations: fls_290jmzfh7x4e
// Compound Assignment Expressions
