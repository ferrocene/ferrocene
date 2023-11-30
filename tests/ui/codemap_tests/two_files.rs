include!("two_files_data.rs");

struct Baz { }

impl Bar for Baz { } //~ ERROR expected trait, found type alias

fn main() { }

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
