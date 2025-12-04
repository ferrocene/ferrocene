//! Synchronization primitives

#![stable(feature = "rust1", since = "1.0.0")]

pub mod atomic;
#[cfg(not(feature = "ferrocene_subset"))]
mod exclusive;
#[unstable(feature = "exclusive_wrapper", issue = "98407")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use exclusive::Exclusive;
