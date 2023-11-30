trait A { fn foo(); }
trait B { fn foo(); }

struct AB {}

impl A for AB {
    fn foo() {}
}

impl B for AB {
    fn foo() {}
}

fn main() {
    AB::foo();  //~ ERROR E0034
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
