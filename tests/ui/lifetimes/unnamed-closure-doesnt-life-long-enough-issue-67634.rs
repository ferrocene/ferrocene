fn main() {
    [0].iter().flat_map(|a| [0].iter().map(|_| &a)); //~ ERROR closure may outlive
}

// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
