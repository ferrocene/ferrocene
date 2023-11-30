fn main() {
    (|| {})(|| {
        //~^ ERROR function takes 0 arguments but 1 argument was supplied
        let b = 1;
    });
}
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
