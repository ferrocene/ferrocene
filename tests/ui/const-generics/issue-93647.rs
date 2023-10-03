struct X<const N: usize = {
    (||1usize)()
    //~^ ERROR cannot call non-const closure
}>;

fn main() {}

// ferrocene-annotations: fls_66m4rnbssgig
// Constant Expressions
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
