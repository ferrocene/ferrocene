use crate::cell::UnsafeCell;
use crate::mem::ManuallyDrop;
use crate::ops::Deref;
use crate::panic::{RefUnwindSafe, UnwindSafe};
use crate::sync::Once;
use crate::{fmt, ptr};

use super::once::ExclusiveState;

// We use the state of a Once as discriminant value. Upon creation, the state is
// "incomplete" and `f` contains the initialization closure. In the first call to
// `call_once`, `f` is taken and run. If it succeeds, `value` is set and the state
// is changed to "complete". If it panics, the Once is poisoned, so none of the
// two fields is initialized.
union Data<T, F> {
    value: ManuallyDrop<T>,
    f: ManuallyDrop<F>,
}

/// A value which is initialized on the first access.
///
/// This type is a thread-safe [`LazyCell`], and can be used in statics.
/// Since initialization may be called from multiple threads, any
/// dereferencing call will block the calling thread if another
/// initialization routine is currently running.
///
/// [`LazyCell`]: crate::cell::LazyCell
///
/// # Examples
///
/// Initialize static variables with `LazyLock`.
///
/// ```
/// use std::collections::HashMap;
///
/// use std::sync::LazyLock;
///
/// static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
///     println!("initializing");
///     let mut m = HashMap::new();
///     m.insert(13, "Spica".to_string());
///     m.insert(74, "Hoyten".to_string());
///     m
/// });
///
/// fn main() {
///     println!("ready");
///     std::thread::spawn(|| {
///         println!("{:?}", HASHMAP.get(&13));
///     }).join().unwrap();
///     println!("{:?}", HASHMAP.get(&74));
///
///     // Prints:
///     //   ready
///     //   initializing
///     //   Some("Spica")
///     //   Some("Hoyten")
/// }
/// ```
/// Initialize fields with `LazyLock`.
/// ```
/// use std::sync::LazyLock;
///
/// #[derive(Debug)]
/// struct UseCellLock {
///     number: LazyLock<u32>,
/// }
/// fn main() {
///     let lock: LazyLock<u32> = LazyLock::new(|| 0u32);
///
///     let data = UseCellLock { number: lock };
///     println!("{}", *data.number);
/// }
/// ```
#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
pub struct LazyLock<T, F = fn() -> T> {
    once: Once,
    data: UnsafeCell<Data<T, F>>,
}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    /// Creates a new lazy value with the given initializing function.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::LazyLock;
    ///
    /// let hello = "Hello, World!".to_string();
    ///
    /// let lazy = LazyLock::new(|| hello.to_uppercase());
    ///
    /// assert_eq!(&*lazy, "HELLO, WORLD!");
    /// ```
    #[inline]
    #[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
    #[rustc_const_stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
    pub const fn new(f: F) -> LazyLock<T, F> {
        LazyLock { once: Once::new(), data: UnsafeCell::new(Data { f: ManuallyDrop::new(f) }) }
    }

    /// Creates a new lazy value that is already initialized.
    #[inline]
    #[cfg(test)]
    pub(crate) fn preinit(value: T) -> LazyLock<T, F> {
        let once = Once::new();
        once.call_once(|| {});
        LazyLock { once, data: UnsafeCell::new(Data { value: ManuallyDrop::new(value) }) }
    }

    /// Consumes this `LazyLock` returning the stored value.
    ///
    /// Returns `Ok(value)` if `Lazy` is initialized and `Err(f)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(lazy_cell_consume)]
    ///
    /// use std::sync::LazyLock;
    ///
    /// let hello = "Hello, World!".to_string();
    ///
    /// let lazy = LazyLock::new(|| hello.to_uppercase());
    ///
    /// assert_eq!(&*lazy, "HELLO, WORLD!");
    /// assert_eq!(LazyLock::into_inner(lazy).ok(), Some("HELLO, WORLD!".to_string()));
    /// ```
    #[unstable(feature = "lazy_cell_consume", issue = "109736")]
    pub fn into_inner(mut this: Self) -> Result<T, F> {
        let state = this.once.state();
        match state {
            ExclusiveState::Poisoned => panic!("LazyLock instance has previously been poisoned"),
            state => {
                let this = ManuallyDrop::new(this);
                let data = unsafe { ptr::read(&this.data) }.into_inner();
                match state {
                    ExclusiveState::Incomplete => Err(ManuallyDrop::into_inner(unsafe { data.f })),
                    ExclusiveState::Complete => Ok(ManuallyDrop::into_inner(unsafe { data.value })),
                    ExclusiveState::Poisoned => unreachable!(),
                }
            }
        }
    }

    /// Forces the evaluation of this lazy value and returns a reference to
    /// result. This is equivalent to the `Deref` impl, but is explicit.
    ///
    /// This method will block the calling thread if another initialization
    /// routine is currently running.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::LazyLock;
    ///
    /// let lazy = LazyLock::new(|| 92);
    ///
    /// assert_eq!(LazyLock::force(&lazy), &92);
    /// assert_eq!(&*lazy, &92);
    /// ```
    #[inline]
    #[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
    pub fn force(this: &LazyLock<T, F>) -> &T {
        this.once.call_once(|| {
            // SAFETY: `call_once` only runs this closure once, ever.
            let data = unsafe { &mut *this.data.get() };
            let f = unsafe { ManuallyDrop::take(&mut data.f) };
            let value = f();
            data.value = ManuallyDrop::new(value);
        });

        // SAFETY:
        // There are four possible scenarios:
        // * the closure was called and initialized `value`.
        // * the closure was called and panicked, so this point is never reached.
        // * the closure was not called, but a previous call initialized `value`.
        // * the closure was not called because the Once is poisoned, so this point
        //   is never reached.
        // So `value` has definitely been initialized and will not be modified again.
        unsafe { &*(*this.data.get()).value }
    }
}

impl<T, F> LazyLock<T, F> {
    /// Get the inner value if it has already been initialized.
    fn get(&self) -> Option<&T> {
        if self.once.is_completed() {
            // SAFETY:
            // The closure has been run successfully, so `value` has been initialized
            // and will not be modified again.
            Some(unsafe { &*(*self.data.get()).value })
        } else {
            None
        }
    }
}

#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
impl<T, F> Drop for LazyLock<T, F> {
    fn drop(&mut self) {
        match self.once.state() {
            ExclusiveState::Incomplete => unsafe { ManuallyDrop::drop(&mut self.data.get_mut().f) },
            ExclusiveState::Complete => unsafe {
                ManuallyDrop::drop(&mut self.data.get_mut().value)
            },
            ExclusiveState::Poisoned => {}
        }
    }
}

#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    /// Dereferences the value.
    ///
    /// This method will block the calling thread if another initialization
    /// routine is currently running.
    ///
    #[inline]
    fn deref(&self) -> &T {
        LazyLock::force(self)
    }
}

#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
impl<T: Default> Default for LazyLock<T> {
    /// Creates a new lazy value using `Default` as the initializing function.
    #[inline]
    fn default() -> LazyLock<T> {
        LazyLock::new(T::default)
    }
}

#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
impl<T: fmt::Debug, F> fmt::Debug for LazyLock<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("LazyLock");
        match self.get() {
            Some(v) => d.field(v),
            None => d.field(&format_args!("<uninit>")),
        };
        d.finish()
    }
}

// We never create a `&F` from a `&LazyLock<T, F>` so it is fine
// to not impl `Sync` for `F`.
#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
unsafe impl<T: Sync + Send, F: Send> Sync for LazyLock<T, F> {}
// auto-derived `Send` impl is OK.

#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
impl<T: RefUnwindSafe + UnwindSafe, F: UnwindSafe> RefUnwindSafe for LazyLock<T, F> {}
#[stable(feature = "lazy_cell", since = "CURRENT_RUSTC_VERSION")]
impl<T: UnwindSafe, F: UnwindSafe> UnwindSafe for LazyLock<T, F> {}

#[cfg(test)]
mod tests;
