// Regression test for issue #67690
// Rustc endless loop out-of-memory and consequent SIGKILL in generic new type

//@ check-pass
pub type T<P: Send + Send + Send> = P;
//~^ WARN bounds on generic parameters are not enforced in type aliases

fn main() {}

// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
