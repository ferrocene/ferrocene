//@ build-fail

#![deny(ferrocene::known_unvalidated)] //~ NOTE level

use std::marker::PhantomData;

// Setup so we don't depend on core.

struct MyBox<T: ?Sized>(PhantomData<T>);
impl<T: ?Sized> MyBox<T> {
    #[ferrocene::prevalidated]
    fn new(x: &T) -> Self {
        Self(PhantomData)
    }
}

struct Once<T>(T);
#[ferrocene::prevalidated]
fn iter_once<T>(x: T) -> Once<T> {
    Once(x)
}

trait MyIterator {
    type Item;
    fn collect<C>(self) -> C where Self: Sized { //~ NOTE unvalidated
        loop {}
    }
}
impl<T> MyIterator for Once<T> {
    type Item = T;
}
impl<T> MyIterator for MyBox<dyn MyIterator<Item = T>> {
    type Item = T;
}

// Actual test

#[ferrocene::prevalidated]
fn cast<T, I: 'static + MyIterator<Item = T>>(x: I) -> MyBox<dyn MyIterator<Item = T>> {
    MyBox::new(&x) //~ ERROR unvalidated
    //~^ NOTE dynamic trait object
    //~^^ NOTE `MyIterator::collect`
    //~^^^ NOTE main functions
}


fn main() { //~ NOTE validated
    cast(iter_once(1)); //~ NOTE instantiated
}
