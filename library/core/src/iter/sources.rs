#[cfg(not(feature = "ferrocene_subset"))]
mod empty;
#[cfg(not(feature = "ferrocene_subset"))]
mod from_coroutine;
mod from_fn;
#[cfg(not(feature = "ferrocene_subset"))]
mod generator;
#[cfg(not(feature = "ferrocene_subset"))]
mod once;
#[cfg(not(feature = "ferrocene_subset"))]
mod once_with;
#[cfg(not(feature = "ferrocene_subset"))]
mod repeat;
#[cfg(not(feature = "ferrocene_subset"))]
mod repeat_n;
#[cfg(not(feature = "ferrocene_subset"))]
mod repeat_with;
#[cfg(not(feature = "ferrocene_subset"))]
mod successors;

#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "iter_empty", since = "1.2.0")]
pub use self::empty::{Empty, empty};
#[cfg(not(feature = "ferrocene_subset"))]
#[unstable(
    feature = "iter_from_coroutine",
    issue = "43122",
    reason = "coroutines are unstable"
)]
pub use self::from_coroutine::{FromCoroutine, from_coroutine};
#[stable(feature = "iter_from_fn", since = "1.34.0")]
pub use self::from_fn::{FromFn, from_fn};
#[cfg(not(feature = "ferrocene_subset"))]
#[unstable(feature = "iter_macro", issue = "142269", reason = "generators are unstable")]
pub use self::generator::iter;
#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "iter_once", since = "1.2.0")]
pub use self::once::{Once, once};
#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "iter_once_with", since = "1.43.0")]
pub use self::once_with::{OnceWith, once_with};
#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::repeat::{Repeat, repeat};
#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "iter_repeat_n", since = "1.82.0")]
pub use self::repeat_n::{RepeatN, repeat_n};
#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "iterator_repeat_with", since = "1.28.0")]
pub use self::repeat_with::{RepeatWith, repeat_with};
#[cfg(not(feature = "ferrocene_subset"))]
#[stable(feature = "iter_successors", since = "1.34.0")]
pub use self::successors::{Successors, successors};
