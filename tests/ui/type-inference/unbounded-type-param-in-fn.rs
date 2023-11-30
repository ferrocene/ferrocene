fn foo<T>() -> T {
    panic!()
}

fn main() {
    foo(); //~ ERROR type annotations needed
}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
