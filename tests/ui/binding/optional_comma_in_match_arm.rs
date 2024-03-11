//@ run-pass
#![allow(unused_unsafe)]
#![allow(while_true)]

fn main() {
    let x = 1;

    match x {
        1 => loop { break; },
        2 => while true { break; },
        3 => if true { () },
        4 => if true { () } else { () },
        5 => match () { () => () },
        6 => { () },
        7 => unsafe { () },
        _ => (),
    }

    match x {
        1 => loop { break; }
        2 => while true { break; }
        3 => if true { () }
        4 => if true { () } else { () }
        5 => match () { () => () }
        6 => { () }
        7 => unsafe { () }
        _ => ()
    }

    let r: &i32 = &x;

    match r {
        // Absence of comma should not cause confusion between a pattern
        // and a bitwise and.
        &1 => if true { () } else { () }
        &2 => (),
        _ =>()
    }
}

// ferrocene-annotations: fls_azzf1llv3wf
// Literal pattern matching
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference pattern matching
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference patterns
