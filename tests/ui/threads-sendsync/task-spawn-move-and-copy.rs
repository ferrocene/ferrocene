//@ run-pass
#![allow(unused_must_use)]
//@ needs-threads

use std::thread;
use std::sync::mpsc::channel;

pub fn main() {
    let (tx, rx) = channel::<usize>();

    let x: Box<isize> = Box::new(1);
    let x_in_parent = &(*x) as *const isize as usize;

    let t = thread::spawn(move || {
        let x_in_child = &(*x) as *const isize as usize;
        tx.send(x_in_child).unwrap();
    });

    let x_in_child = rx.recv().unwrap();
    assert_eq!(x_in_parent, x_in_child);

    t.join();
}

// ferrocene-annotations: fls_xd2oxlebhs14
// Closure Types
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
