trait A { fn foo(&self); }
trait B { fn foo(&self); }

fn foo<T:A + B>(t: T) {
    t.foo(); //~ ERROR E0034
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
