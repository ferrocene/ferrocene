fn foo<T: ?Sized>(_f: impl AsRef<T>) {}

fn main() {
    foo::<str, String>("".to_string()); //~ ERROR E0107
}

// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
