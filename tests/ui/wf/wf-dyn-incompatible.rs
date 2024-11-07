// Check that dyn-incompatible traits are not WF when used as trait object types.
// Issue #21953.

trait A {
    fn foo(&self, _x: &Self);
}

fn main() {
    let _x: &dyn A; //~ ERROR E0038
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_4ikc07mfrez5
// Object Safety
