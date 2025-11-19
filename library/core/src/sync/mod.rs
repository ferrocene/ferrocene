//! Synchronization primitives

#![stable(feature = "rust1", since = "1.0.0")]

pub mod atomic;
#[cfg_attr(feature = "ferrocene_certified", deprecated(since = "TBD", note = "This is not certified"))]
mod exclusive;
#[unstable(feature = "exclusive_wrapper", issue = "98407")]
#[cfg_attr(feature = "ferrocene_certified", deprecated(since = "TBD", note = "This is not certified"))]
pub use exclusive::Exclusive;
