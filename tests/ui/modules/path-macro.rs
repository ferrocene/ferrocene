macro_rules! foo {
    () => {"bar.rs"};
}

#[path = foo!()] //~ ERROR malformed `path` attribute
mod abc;

fn main() {}

// ferrocene-annotations: fls_1zbaajz5prpn
// Attribute path
