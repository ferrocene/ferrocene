#[allow(invalid_type_param_default)]

fn foo<T, U = u64>() -> (T, U) {
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
