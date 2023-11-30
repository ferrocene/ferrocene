trait A { fn foo(self); }
trait B { fn foo(self); }

struct AB {}

impl A for AB {
    fn foo(self) {}
}

impl B for AB {
    fn foo(self) {}
}

fn main() {
    AB {}.foo();  //~ ERROR E0034
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
