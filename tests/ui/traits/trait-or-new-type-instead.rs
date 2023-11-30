impl<T> Option<T> {
//~^ ERROR cannot define inherent `impl` for a type outside of the crate where the type is defined
    pub fn foo(&self) { }
}

fn main() { }

// ferrocene-annotations: fls_46ork6fz5o2e
// Implementation Coherence
