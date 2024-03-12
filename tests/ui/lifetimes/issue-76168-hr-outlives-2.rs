//@ edition:2018
//@ check-pass

trait Trait<Input> {
    type Output;
}

async fn walk<F>(filter: F)
where
    for<'a> F: Trait<&'a u32> + 'a,
    for<'a> <F as Trait<&'a u32>>::Output: 'a,
{
}

async fn walk2<F: 'static>(filter: F)
where
    for<'a> F: Trait<&'a u32> + 'a,
    for<'a> <F as Trait<&'a u32>>::Output: 'a,
{
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
