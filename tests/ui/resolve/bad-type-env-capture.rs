fn foo<T>() {
    fn bar(b: T) { } //~ ERROR can't use generic parameters from outer
}
fn main() { }

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_ftphlagzd2te
// Generic Parameter Scope
