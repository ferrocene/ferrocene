//@ run-rustfix

fn _f0(&_a: u32) {} //~ ERROR mismatched types
fn _f1(&mut _a: u32) {} //~ ERROR mismatched types
fn _f2(&&_a: &u32) {} //~ ERROR mismatched types
fn _f3(&mut &_a: &mut u32) {} //~ ERROR mismatched types
fn _f4(&&mut _a: &u32) {} //~ ERROR mismatched types
fn _f5(&mut &mut _a: &mut u32) {} //~ ERROR mismatched types

fn main() {
    let _: fn(u32) = |&_a| (); //~ ERROR mismatched types
    let _: fn(u32) = |&mut _a| (); //~ ERROR mismatched types
    let _: fn(&u32) = |&&_a| (); //~ ERROR mismatched types
    let _: fn(&mut u32) = |&mut &_a| (); //~ ERROR mismatched types
    let _: fn(&u32) = |&&mut _a| (); //~ ERROR mismatched types
    let _: fn(&mut u32) = |&mut &mut _a| (); //~ ERROR mismatched types

    let _ = |&_a: u32| (); //~ ERROR mismatched types
    let _ = |&mut _a: u32| (); //~ ERROR mismatched types
    let _ = |&&_a: &u32| (); //~ ERROR mismatched types
    let _ = |&mut &_a: &mut u32| (); //~ ERROR mismatched types
    let _ = |&&mut _a: &u32| (); //~ ERROR mismatched types
    let _ = |&mut &mut _a: &mut u32| (); //~ ERROR mismatched types

    #[allow(unused_mut)]
    {
        struct S(u8);

        let &mut _a = 0; //~ ERROR mismatched types
        let S(&mut _b) = S(0); //~ ERROR mismatched types
        let (&mut _c,) = (0,); //~ ERROR mismatched types

        match 0 {
            &mut _d => {} //~ ERROR mismatched types
        }
    }
}

// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
//
// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
