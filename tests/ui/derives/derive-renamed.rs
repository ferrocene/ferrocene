//@ check-pass
//@ edition:2018

use derive as my_derive;

#[my_derive(Debug)]
struct S;

fn main() {
    println!("{:?}", S); // OK
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
