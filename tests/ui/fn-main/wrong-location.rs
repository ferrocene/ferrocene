mod m {
    // An inferred main entry point
    // must appear at the top of the crate
    fn main() { }
} //~ ERROR `main` function not found

// ferrocene-annotations: fls_8jb3sjqamdpu
// Program Entry Point
