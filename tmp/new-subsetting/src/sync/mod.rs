//! Synchronization primitives

#![stable(feature = "rust1", since = "1.0.0")]

pub mod atomic;
#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod exclusive;
#[unstable(feature = "exclusive_wrapper", issue = "98407")]
#[cfg(not(feature = "ferrocene_certified"))]
pub use exclusive::Exclusive;
