struct Foo;

trait MyTrait {
    fn trait_bar() {}
}

impl MyTrait for Foo {}

fn main() {
    match 0u32 {
        <Foo as MyTrait>::trait_bar => {}
        //~^ ERROR expected unit struct, unit variant or constant, found associated function
    }
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
