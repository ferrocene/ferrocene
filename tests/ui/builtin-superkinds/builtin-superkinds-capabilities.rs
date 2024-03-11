//@ run-pass
// Tests "capabilities" granted by traits that inherit from super-
// builtin-kinds, e.g., if a trait requires Send to implement, then
// at usage site of that trait, we know we have the Send capability.


use std::sync::mpsc::{channel, Sender, Receiver};

trait Foo : Send { }

impl <T: Send> Foo for T { }

fn foo<T: Foo + 'static>(val: T, chan: Sender<T>) {
    chan.send(val).unwrap();
}

pub fn main() {
    let (tx, rx): (Sender<isize>, Receiver<isize>) = channel();
    foo(31337, tx);
    assert_eq!(rx.recv().unwrap(), 31337);
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
