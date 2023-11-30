#[cfg(FALSE)]
impl S {
    fn f(#[attr]) {} //~ ERROR expected parameter name, found `)`
}

#[cfg(FALSE)]
impl T for S {
    fn f(#[attr]) {} //~ ERROR expected parameter name, found `)`
}

#[cfg(FALSE)]
trait T {
    fn f(#[attr]); //~ ERROR expected argument name, found `)`
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
