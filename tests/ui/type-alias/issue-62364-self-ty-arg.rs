struct Struct<P1> {
    field: P1,
}

type Alias<'a> = Struct<&'a Self>;
//~^ ERROR cannot find type `Self` in this scope [E0411]

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
