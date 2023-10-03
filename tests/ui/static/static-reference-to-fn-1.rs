struct A<'a> {
    func: &'a fn() -> Option<isize>
}

impl<'a> A<'a> {
    fn call(&self) -> Option<isize> {
        (*self.func)()
    }
}

fn foo() -> Option<isize> {
    None
}

fn create() -> A<'static> {
    A {
        func: &foo, //~ ERROR mismatched types
    }
}

fn main() {
    let a = create();
    a.call();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_g0uyl7qw4c7w
// Parenthesized Expressions
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
