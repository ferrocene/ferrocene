trait MyTrait {}
struct AssertMyTrait<T: MyTrait>(T);

trait HelperTrait {
    type MyItem;
}

impl HelperTrait for () {
    type MyItem = Option<((AssertMyTrait<bool>, u8))>; //~ ERROR the trait bound
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
