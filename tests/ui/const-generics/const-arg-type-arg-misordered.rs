type Array<T, const N: usize> = [T; N];

fn foo<const N: usize>() -> Array<N, ()> {
    //~^ ERROR constant provided when a type was expected
    unimplemented!()
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
