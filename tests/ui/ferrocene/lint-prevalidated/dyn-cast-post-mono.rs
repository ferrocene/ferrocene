//@ revisions: thir post-mono
//@[thir] check-pass
//@[post-mono] build-fail

#![deny(ferrocene::unvalidated)] //[post-mono]~ NOTE level

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
    fn collect<C>(self) -> C where Self: Sized { //[post-mono]~ NOTE unvalidated
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
    MyBox::new(&x) //[post-mono]~ ERROR unvalidated
    //[post-mono]~^ NOTE dynamic trait object
    //[post-mono]~^^ NOTE `MyIterator::collect`
    //[post-mono]~^^^ NOTE main functions
}


fn main() { //[post-mono]~ NOTE validated
    cast(iter_once(1)); //[post-mono]~ NOTE instantiated
}
