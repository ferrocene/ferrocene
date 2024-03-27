#[cfg(FALSE)]
fn syntax() {
    bar::<Item = 'a>(); //~ ERROR lifetimes are not permitted in this context
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
