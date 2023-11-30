// Issue #11669

// ignore-tidy-cr

fn main() {
    // \r\n
    let ok = "This is \
 a test";
    // \r only
    let bad = "This is \ a test";
    //~^ ERROR unknown character escape: `\r`
    //~| HELP this is an isolated carriage return

}

// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
