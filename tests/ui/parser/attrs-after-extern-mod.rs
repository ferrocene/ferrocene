// Make sure there's an error when given `extern { ... #[attr] }`.

fn main() {}

extern "C" {
    #[cfg(stage37)] //~ ERROR expected item after attributes
}

// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
