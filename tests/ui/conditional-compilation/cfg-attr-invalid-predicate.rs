#[cfg(foo(bar))] //~ ERROR malformed `cfg` attribute input [E0539]
fn check() {}

fn main() {}

// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
