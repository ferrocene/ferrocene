// Matching against float literals should result in a linter error

#![feature(exclusive_range_pattern)]
#![allow(unused)]
#![forbid(illegal_floating_point_literal_pattern)]

fn main() {
    let x = 42.0;
    match x {
        5.0 => {}, //~ ERROR floating-point types cannot be used in patterns
                   //~| WARNING hard error
        5.0f32 => {}, //~ ERROR floating-point types cannot be used in patterns
                      //~| WARNING hard error
        -5.0 => {}, //~ ERROR floating-point types cannot be used in patterns
                    //~| WARNING hard error
        1.0 .. 33.0 => {}, //~ ERROR floating-point types cannot be used in patterns
                           //~| WARNING hard error
                           //~| ERROR floating-point types cannot be used in patterns
                           //~| WARNING hard error
        39.0 ..= 70.0 => {}, //~ ERROR floating-point types cannot be used in patterns
                             //~| ERROR floating-point types cannot be used in patterns
                             //~| WARNING hard error
                             //~| WARNING hard error

        ..71.0 => {}
        //~^ ERROR floating-point types cannot be used in patterns
        //~| WARNING this was previously accepted by the compiler
        ..=72.0 => {}
        //~^ ERROR floating-point types cannot be used in patterns
        //~| WARNING this was previously accepted by the compiler
        71.0.. => {}
        //~^ ERROR floating-point types cannot be used in patterns
        //~| WARNING this was previously accepted by the compiler
        _ => {},
    };
    let y = 5.0;
    // Same for tuples
    match (x, 5) {
        (3.14, 1) => {}, //~ ERROR floating-point types cannot be used
                         //~| WARNING hard error
        _ => {},
    }
    // Or structs
    struct Foo { x: f32 };
    match (Foo { x }) {
        Foo { x: 2.0 } => {}, //~ ERROR floating-point types cannot be used
                              //~| WARNING hard error
        _ => {},
    }
}

// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
