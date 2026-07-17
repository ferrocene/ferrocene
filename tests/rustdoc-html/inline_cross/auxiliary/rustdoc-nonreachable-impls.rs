pub struct Foo;

pub trait Woof {}
pub trait Bark {}

mod private {
    // should be shown
    impl crate::Woof for crate::Foo {}

    pub trait Bar {}
    pub struct Wibble;

    // these should not be shown
    impl Bar for crate::Foo {}
    impl Bar for Wibble {}
    impl crate::Bark for Wibble {}
    impl crate::Woof for Wibble {}
}

#[doc(hidden)]
pub mod hidden {
    // should be shown
    impl crate::Bark for crate::Foo {}

    pub trait Qux {}
    pub struct Wobble;


    // these should only be shown if they're re-exported correctly
    impl Qux for crate::Foo {}
    impl Qux for Wobble {}
    impl crate::Bark for Wobble {}
    impl crate::Woof for Wobble {}
}
