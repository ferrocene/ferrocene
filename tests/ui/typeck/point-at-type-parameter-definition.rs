trait Trait {
    fn do_stuff(&self);
}

struct Hello;

impl Hello {
    fn method(&self) {}
}

impl<Hello> Trait for Vec<Hello> {
    fn do_stuff(&self) {
        self[0].method(); //~ ERROR no method named `method` found for type parameter `Hello` in the current scope
    }
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_sxcr4aa098i6
// Indexing Expressions
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
