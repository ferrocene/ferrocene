//@ error-pattern: return

fn f() -> isize { } //~ ERROR mismatched types

fn main() { f(); }

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
