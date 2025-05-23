//@ edition: 2021

// The null char check for C string literals was originally implemented after
// expansion, which meant the first five strings in this file triggered errors,
// and the remaining ten did not. But this is different to all the other
// content checks done on string literals, such as checks for invalid escapes
// and bare CR chars. So the check was moved earlier. The check can be moved
// back to after expansion at a later date if necessary, because that would be
// a backward compatible change. (In contrast, moving the check from after
// expansion to lexing time would be a backward incompatible change, because it
// could break code that was previously accepted.)

fn main() {
    c"\0";     //~ ERROR null characters in C string literals
    c"\u{00}"; //~ ERROR null characters in C string literals
    c" ";     //~ ERROR null characters in C string literals
    c"\x00";   //~ ERROR null characters in C string literals
    cr" ";    //~ ERROR null characters in C string literals
}

macro_rules! empty {
    ($($tt:tt)*) => {};
}

// The cfg does not consume the literals before nul checking occurs.
#[cfg(FALSE)]
fn test() {
    c"\0";     //~ ERROR null characters in C string literals
    c"\u{00}"; //~ ERROR null characters in C string literals
    c" ";     //~ ERROR null characters in C string literals
    c"\x00";   //~ ERROR null characters in C string literals
    cr" ";    //~ ERROR null characters in C string literals
}

// The macro does not consume the literals before nul checking occurs.
fn test_empty() {
    empty!(c"\0");     //~ ERROR null characters in C string literals
    empty!(c"\u{00}"); //~ ERROR null characters in C string literals
    empty!(c" ");     //~ ERROR null characters in C string literals
    empty!(c"\x00");   //~ ERROR null characters in C string literals
    empty!(cr" ");    //~ ERROR null characters in C string literals
}

// ferrocene-annotations: fls_u1ghcy16emve
// C String Literals
//
// ferrocene-annotations: fls_p090c5otnelw
// Simple C String Literals
//
// ferrocene-annotations: fls_g4ldypf3rl6i
// Raw C String Literals
