fn foo() -> isize {
    let x: isize;

    while 1 != 2  {
        break;
        x = 0;
    }

    println!("{}", x); //~ ERROR E0381

    return 17;
}

fn main() { println!("{}", foo()); }

//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
