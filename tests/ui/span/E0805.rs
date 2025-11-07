pub fn main() {
    if cfg!(not()) { } //~ ERROR E0805
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
