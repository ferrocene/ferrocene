//@ check-pass

fn main() {
    let s = "ZͨA͑ͦ͒͋ͤ͑̚L̄͑͋Ĝͨͥ̿͒̽̈́Oͥ͛ͭ!̏"; while true { break; } //~ WARNING while_true
    println!("{}", s);
}

// ferrocene-annotations: fls_2i089jvv8j5g
// Character Set
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
