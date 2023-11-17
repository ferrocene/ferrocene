#[cfg(FALSE)]
fn syntax() {
    bar::<Item = 'a>(); //~ ERROR associated lifetimes are not supported
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
