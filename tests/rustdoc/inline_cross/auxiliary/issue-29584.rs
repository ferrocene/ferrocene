//@ compile-flags: -Cmetadata=aux
//@ edition: 2015
pub struct Foo;

#[doc(hidden)]
mod bar {
    trait Bar {}

    impl Bar for ::Foo {}
}
