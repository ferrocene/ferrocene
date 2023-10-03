#[allow(non_camel_case_types)]
struct foo(usize);

fn main() {
    let (foo, _) = (2, 3); //~ ERROR let bindings cannot shadow tuple structs
}

// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
