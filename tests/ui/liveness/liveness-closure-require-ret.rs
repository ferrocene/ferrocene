fn force<F>(f: F) -> isize where F: FnOnce() -> isize { f() }
fn main() { println!("{}", force(|| {})); } //~ ERROR mismatched types

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
