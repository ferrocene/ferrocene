<<<<<<< HEAD
#[cfg(not())] //~ ERROR E0536
pub fn something() {}

pub fn main() {}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
=======
pub fn main() {
    if cfg!(not()) { } //~ ERROR E0536
}
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
