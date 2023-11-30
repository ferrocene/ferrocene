trait Trait1<'a> {}
trait Trait2<'a, 'b> {}

fn f() where
    for<'a> dyn Trait1<'a>: Trait1<'a>, // OK
    (dyn for<'a> Trait1<'a>): Trait1<'a>,
    //~^ ERROR use of undeclared lifetime name `'a`
    for<'a> dyn for<'b> Trait2<'a, 'b>: Trait2<'a, 'b>,
    //~^ ERROR use of undeclared lifetime name `'b`
{}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
