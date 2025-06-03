#[cfg(feature = "uncertified")]
mod accum;

mod collect;
#[cfg(feature = "uncertified")]
mod double_ended;
#[cfg(feature = "uncertified")]
mod exact_size;
mod iterator;
#[cfg(feature = "uncertified")]
mod marker;
#[cfg(feature = "uncertified")]
mod unchecked_iterator;

#[unstable(issue = "none", feature = "inplace_iteration")]
#[cfg(feature = "uncertified")]
pub use self::marker::InPlaceIterable;
#[unstable(issue = "none", feature = "trusted_fused")]
#[cfg(feature = "uncertified")]
pub use self::marker::TrustedFused;
#[unstable(feature = "trusted_step", issue = "85731")]
#[cfg(feature = "uncertified")]
pub use self::marker::TrustedStep;
#[cfg(feature = "uncertified")]
pub(crate) use self::unchecked_iterator::UncheckedIterator;
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg(feature = "uncertified")]
pub use self::{
    accum::{Product, Sum},
    collect::{Extend, FromIterator},
    double_ended::DoubleEndedIterator,
    exact_size::ExactSizeIterator,
    marker::{FusedIterator, TrustedLen},
};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::{collect::IntoIterator, iterator::Iterator};
