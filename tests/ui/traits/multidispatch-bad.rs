// Test that we detect an illegal combination of types.

trait Convert<Target> {
    fn convert(&self) -> Target;
}

impl Convert<u32> for i32 {
    fn convert(&self) -> u32 {
        *self as u32
    }
}

fn test<T,U>(_: T, _: U)
where T : Convert<U>
{
}

fn a() {
    test(22i32, 44i32); //~ ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
