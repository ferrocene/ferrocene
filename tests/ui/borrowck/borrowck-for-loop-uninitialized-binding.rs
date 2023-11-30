fn f() -> isize {
    let mut x: isize;
    for _ in 0..0 { x = 10; }
    return x; //~ ERROR E0381
}

fn main() { f(); }

//
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
