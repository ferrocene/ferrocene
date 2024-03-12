//@ check-pass

fn main() {}

#[cfg(FALSE)]
impl X {
    type Y;
    type Z: Ord;
    type W: Ord where Self: Eq;
    type W where Self: Eq;
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
