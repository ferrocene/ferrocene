//@ edition:2018

#![feature(try_blocks)]

fn main() {}

fn in_loop() {
    loop
        let x = 0; //~ ERROR expected `{`, found keyword `let`
        drop(0);
}

fn in_while() {
    while true
        let x = 0; //~ ERROR expected `{`, found keyword `let`
}

fn in_for() {
    for x in 0..1
        let x = 0; //~ ERROR expected `{`, found keyword `let`
}


// FIXME
fn in_try() {
    try //~ ERROR expected expression, found reserved keyword `try`
        let x = 0;
}

// FIXME(#80931)
fn in_async() {
    async
        let x = 0; //~ ERROR expected one of `move`, `|`, or `||`, found keyword `let`
}

// FIXME(#78168)
fn in_const() {
    let x = const 2; //~ ERROR expected expression, found keyword `const`
}

// FIXME(#78168)
fn in_const_in_match() {
    let x = 2;
    match x {
        const 2 => {}
        //~^ ERROR expected identifier, found keyword `const`
        //~| ERROR expected one of `=>`, `if`, or `|`, found `2`
    }
}

// ferrocene-annotations: fls_aadan19t5006
// async Blocks
//
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
