use self::Direction::{North, East, South, West};

#[derive(PartialEq, Eq)]
struct NewBool(bool);

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}

const TRUE_TRUE: (bool, bool) = (true, true);

fn nonexhaustive_1() {
    match (true, false) {
    //~^ ERROR non-exhaustive patterns: `(true, false)` not covered
        TRUE_TRUE => (),
        (false, false) => (),
        (false, true) => ()
    }
}

const NONE: Option<Direction> = None;
const EAST: Direction = East;

fn nonexhaustive_2() {
    match Some(Some(North)) {
    //~^ ERROR non-exhaustive patterns: `Some(Some(Direction::West))` not covered
        Some(NONE) => (),
        Some(Some(North)) => (),
        Some(Some(EAST)) => (),
        Some(Some(South)) => (),
        None => ()
    }
}

const NEW_FALSE: NewBool = NewBool(false);
struct Foo {
    bar: Option<Direction>,
    baz: NewBool
}

const STATIC_FOO: Foo = Foo { bar: None, baz: NEW_FALSE };

fn nonexhaustive_3() {
    match (Foo { bar: Some(North), baz: NewBool(true) }) {
    //~^ ERROR non-exhaustive patterns: `Foo { bar: Some(Direction::North), baz: NewBool(true) }`
        Foo { bar: None, baz: NewBool(true) } => (),
        Foo { bar: _, baz: NEW_FALSE } => (),
        Foo { bar: Some(West), baz: NewBool(true) } => (),
        Foo { bar: Some(South), .. } => (),
        Foo { bar: Some(EAST), .. } => ()
    }
}

fn main() {
    nonexhaustive_1();
    nonexhaustive_2();
    nonexhaustive_3();
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_4ckl3n2ko3i4
// Tuple Types
//
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
