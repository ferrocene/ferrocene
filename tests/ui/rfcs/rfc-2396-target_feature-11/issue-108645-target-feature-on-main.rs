//@ only-x86_64

#[target_feature(enable = "avx2")]
fn main() {}
//~^ ERROR `main` function is not allowed to have `#[target_feature]`

// ferrocene-annotations: fls_8jb3sjqamdpu
// Program Entry Point
