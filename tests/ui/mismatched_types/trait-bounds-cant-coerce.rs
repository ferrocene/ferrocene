trait Foo {
    fn dummy(&self) { }
}

fn a(_x: Box<dyn Foo + Send>) {
}

fn c(x: Box<dyn Foo + Sync + Send>) {
    a(x);
}

fn d(x: Box<dyn Foo>) {
    a(x); //~ ERROR mismatched types [E0308]
}

fn main() { }

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
