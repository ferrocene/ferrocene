fn foo(x: isize) { println!("{}", x); }

fn main() {
    let x: isize;
    foo(x); //~ ERROR E0381
}

//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
