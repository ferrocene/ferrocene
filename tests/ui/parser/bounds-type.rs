//@ compile-flags: -Z parse-only

struct S<
    T: 'a + Tr, // OK
    T: Tr + 'a, // OK
    T: 'a, // OK
    T:, // OK
    T: ?for<'a> Trait, // OK
    T: Tr +, // OK
    T: ?'a, //~ ERROR `?` may only modify trait bounds, not lifetime bounds

    T: ~const Tr, // OK
    T: ~const ?Tr, // OK
    T: ~const Tr + 'a, // OK
    T: ~const 'a, //~ ERROR `~const` may only modify trait bounds, not lifetime bounds
    T: const 'a, //~ ERROR `const` may only modify trait bounds, not lifetime bounds
>;

fn main() {}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
