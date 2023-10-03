trait A { }

impl A for isize {
    const BAR: () = (); //~ ERROR const `BAR` is not a member of trait `A`
    type Baz = (); //~ ERROR type `Baz` is not a member of trait `A`
    fn foo(&self) { } //~ ERROR method `foo` is not a member of trait `A`
}

fn main() { }

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
