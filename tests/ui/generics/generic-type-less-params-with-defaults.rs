use std::marker;

struct Heap;

struct Vec<T, A = Heap>(
    marker::PhantomData<(T,A)>);

fn main() {
    let _: Vec;
    //~^ ERROR missing generics for struct `Vec`
}

// ferrocene-annotations: fls_xjcxbajhzp3d
// Type Parameters
