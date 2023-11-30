type N = u32;
struct Foo<const M: usize>;
fn test<const N: usize>() -> Foo<N> { //~ ERROR type provided when
    Foo
}

fn main() {}

// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
