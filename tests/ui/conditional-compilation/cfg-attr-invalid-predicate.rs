#[cfg(foo(bar))] //~ ERROR invalid predicate `foo`
fn check() {}

fn main() {}

// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
