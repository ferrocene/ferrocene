#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod accum;
mod collect;
#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod double_ended;
#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod exact_size;
mod iterator;
#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod marker;
#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod unchecked_iterator;

#[unstable(issue = "none", feature = "inplace_iteration")]
#[cfg(not(feature = "ferrocene_certified"))]
pub use self::marker::InPlaceIterable;
#[unstable(issue = "none", feature = "trusted_fused")]
#[cfg(not(feature = "ferrocene_certified"))]
pub use self::marker::TrustedFused;
#[unstable(feature = "trusted_step", issue = "85731")]
#[cfg(not(feature = "ferrocene_certified"))]
pub use self::marker::TrustedStep;
#[cfg(not(feature = "ferrocene_certified"))]
pub(crate) use self::unchecked_iterator::UncheckedIterator;
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg(not(feature = "ferrocene_certified"))]
pub use self::{
    accum::{Product, Sum},
    collect::{Extend, FromIterator, IntoIterator},
    double_ended::DoubleEndedIterator,
    exact_size::ExactSizeIterator,
    iterator::Iterator,
    marker::{FusedIterator, TrustedLen},
};
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg(feature = "ferrocene_certified")]
pub use self::{collect::IntoIterator, iterator::Iterator};
