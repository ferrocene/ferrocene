mod accum;
mod collect;
mod double_ended;
mod exact_size;
mod iterator;
mod marker;
mod unchecked_iterator;

#[unstable(issue = "none", feature = "inplace_iteration")]
#[cfg_attr(feature = "ferrocene_certified", deprecated(since = "TBD", note = "This is not certified"))]
pub use self::marker::InPlaceIterable;
#[unstable(issue = "none", feature = "trusted_fused")]
#[cfg_attr(feature = "ferrocene_certified", deprecated(since = "TBD", note = "This is not certified"))]
pub use self::marker::TrustedFused;
#[unstable(feature = "trusted_step", issue = "85731")]
#[cfg_attr(feature = "ferrocene_certified", deprecated(since = "TBD", note = "This is not certified"))]
pub use self::marker::TrustedStep;
pub(crate) use self::unchecked_iterator::UncheckedIterator;
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(feature = "ferrocene_certified", deprecated(since = "TBD", note = "This is not certified"))]
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
#[cfg(false)]
#[rustfmt::skip]
pub use self::{
    accum::Sum,
    collect::{Extend, FromIterator, IntoIterator},
    double_ended::DoubleEndedIterator,
    exact_size::ExactSizeIterator,
    iterator::Iterator,
    marker::TrustedLen,
};
