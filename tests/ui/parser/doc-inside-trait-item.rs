trait User{
    fn test();
    /// empty doc
    //~^ ERROR found a documentation comment that doesn't document anything
}
fn main() {}

// ferrocene-annotations: fls_q8l2jza7d9xa
// Comments
