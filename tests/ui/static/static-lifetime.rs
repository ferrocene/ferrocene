pub trait Arbitrary: Sized + 'static {}

impl<'a, A: Clone> Arbitrary for ::std::borrow::Cow<'a, A> {} //~ ERROR lifetime bound

fn main() {
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
