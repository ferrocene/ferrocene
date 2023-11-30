trait A {
    fn a(&self) {
        || self.b()
        //~^ ERROR no method named `b` found
    }
}
fn main() {}

// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
