#[inline(unknown)] //~ ERROR malformed `inline` attribute
pub fn something() {}

fn main() {
    something();
}

// ferrocene-annotations: fls_ypio6boj3pwf
// Attribute inline
