//@ run-pass
#![allow(unused_must_use)]
//@ needs-threads

use std::thread;

pub fn main() {
    let x = "Hello world!".to_string();
    thread::spawn(move|| {
        println!("{}", x);
    }).join();
}
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
