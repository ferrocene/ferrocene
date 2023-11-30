// Tests that the use of uninitialized variable in assignment operator
// expression is detected.

pub fn main() {
    let x: isize;
    x += 1; //~ ERROR E0381

    let x: isize;
    x -= 1; //~ ERROR E0381

    let x: isize;
    x *= 1; //~ ERROR E0381

    let x: isize;
    x /= 1; //~ ERROR E0381

    let x: isize;
    x %= 1; //~ ERROR E0381

    let x: isize;
    x ^= 1; //~ ERROR E0381

    let x: isize;
    x &= 1; //~ ERROR E0381

    let x: isize;
    x |= 1; //~ ERROR E0381

    let x: isize;
    x <<= 1; //~ ERROR E0381

    let x: isize;
    x >>= 1; //~ ERROR E0381
}

//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
