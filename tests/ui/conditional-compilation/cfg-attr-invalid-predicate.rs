#[cfg(foo(bar))] //~ ERROR malformed `cfg` attribute input [E0539]
fn check() {}

fn main() {}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
