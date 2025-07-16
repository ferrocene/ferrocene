pub fn main() {
    if cfg!(not()) { } //~ ERROR E0536
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
