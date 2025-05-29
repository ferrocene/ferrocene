// Test a default that references `Self` which is then used in an
// object type. Issue #18956. In this case, the value is supplied by
// the user, but pretty-printing the type during the error message
// caused an ICE.

trait MyAdd<Rhs=Self> { fn add(&self, other: &Rhs) -> Self; }

impl MyAdd for i32 {
    fn add(&self, other: &i32) -> i32 { *self + *other }
}

fn main() {
    let x: i32 = 5;
    let y = x as dyn MyAdd<i32>;
    //~^ ERROR E0038
}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
