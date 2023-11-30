#[inline(unknown)] //~ ERROR E0535
pub fn something() {}

fn main() {
    something();
}

// ferrocene-annotations: fls_ypio6boj3pwf
// Attribute inline
