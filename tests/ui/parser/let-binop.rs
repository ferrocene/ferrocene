//@ run-rustfix

fn main() {
    let a: i8 *= 1; //~ ERROR can't reassign to an uninitialized variable
    let _ = a;
    let b += 1; //~ ERROR can't reassign to an uninitialized variable
    let _ = b;
    let c *= 1; //~ ERROR can't reassign to an uninitialized variable
    let _ = c;
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
