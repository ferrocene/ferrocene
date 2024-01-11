union Foo {
    bar: i8,
    zst: (),
    pizza: Pizza,
}

#[derive(Clone, Copy)]
struct Pizza {
    topping: Option<PizzaTopping>
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum PizzaTopping {
    Cheese,
    Pineapple,
}

fn do_nothing(_x: &mut Foo) {}

pub fn main() {
    let mut foo = Foo { bar: 5 };
    do_nothing(&mut foo);

    // This is UB, so this test isn't run
    match foo {
        Foo { bar: _a } => {}, //~ ERROR access to union field is unsafe
    }
    match foo {
        Foo {
            pizza: Pizza { //~ ERROR access to union field is unsafe
                topping: Some(PizzaTopping::Cheese) | Some(PizzaTopping::Pineapple) | None
            }
        } => {},
    }

    match foo {
        Foo { zst: () } => {} //~ ERROR access to union field is unsafe
    }
    match foo {
        Foo { pizza: Pizza { .. } } => {} //~ ERROR access to union field is unsafe
    }

    // binding to wildcard is okay
    match foo {
        Foo { bar: _ } => {},
    }
    let Foo { bar: _ } = foo;
}

// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
