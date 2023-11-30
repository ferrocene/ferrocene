fn main() {
    let bar;
    fn baz(_x: isize) { }
    baz(bar); //~ ERROR E0381
}

//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
