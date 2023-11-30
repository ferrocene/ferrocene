#![allow(dead_code)]
#![deny(unreachable_patterns)]

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

fn unreachable_1() {
    match (true, false) {
        TRUE_TRUE => (),
        (false, false) => (),
        (false, true) => (),
        (true, false) => (),
        (true, true) => ()
        //~^ ERROR unreachable pattern
    }
}

const NONE: Option<Direction> = None;
const EAST: Direction = East;

fn unreachable_2() {
    match Some(Some(North)) {
        Some(NONE) => (),
        Some(Some(North)) => (),
        Some(Some(EAST)) => (),
        Some(Some(South)) => (),
        Some(Some(West)) => (),
        Some(Some(East)) => (),
        //~^ ERROR unreachable pattern
        None => ()
    }
}

const NEW_FALSE: NewBool = NewBool(false);
struct Foo {
    bar: Option<Direction>,
    baz: NewBool
}

fn unreachable_3() {
    match (Foo { bar: Some(EAST), baz: NewBool(true) }) {
        Foo { bar: None, baz: NewBool(true) } => (),
        Foo { bar: _, baz: NEW_FALSE } => (),
        Foo { bar: Some(West), baz: NewBool(true) } => (),
        Foo { bar: Some(South), .. } => (),
        Foo { bar: Some(EAST), .. } => (),
        Foo { bar: Some(North), baz: NewBool(true) } => (),
        Foo { bar: Some(EAST), baz: NewBool(false) } => ()
        //~^ ERROR unreachable pattern
    }
}

fn main() {
    unreachable_1();
    unreachable_2();
    unreachable_3();
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
