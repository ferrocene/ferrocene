mod accum;
mod collect;
mod double_ended;
mod exact_size;
mod iterator;
mod marker;
mod unchecked_iterator;

#[unstable(issue = "none", feature = "inplace_iteration")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use self::marker::InPlaceIterable;
#[unstable(issue = "none", feature = "trusted_fused")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use self::marker::TrustedFused;
#[unstable(feature = "trusted_step", issue = "85731")]
pub use self::marker::TrustedStep;
pub(crate) use self::unchecked_iterator::UncheckedIterator;
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg(not(feature = "ferrocene_subset"))]
pub use self::{
    accum::{Product, Sum},
    collect::{Extend, FromIterator, IntoIterator},
    double_ended::DoubleEndedIterator,
    exact_size::ExactSizeIterator,
    iterator::Iterator,
    marker::{FusedIterator, TrustedLen},
};

// Ferrocene addition: imports for certified subset
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg(feature = "ferrocene_subset")]
#[rustfmt::skip]
pub use self::{
    accum::Sum,
    collect::{Extend, FromIterator, IntoIterator},
    double_ended::DoubleEndedIterator,
    exact_size::ExactSizeIterator,
    iterator::Iterator,
    marker::{FusedIterator, TrustedLen},
};
