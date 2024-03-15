use crate::cmp::Ordering;
use crate::fmt;
use crate::hash;
use crate::intrinsics;
use crate::intrinsics::assert_unsafe_precondition;
use crate::marker::Unsize;
use crate::mem::{MaybeUninit, SizedTypeProperties};
use crate::num::NonZero;
use crate::ops::{CoerceUnsized, DispatchFromDyn};
use crate::ptr;
use crate::ptr::Unique;
use crate::slice::{self, SliceIndex};

/// `*mut T` but non-zero and [covariant].
///
/// This is often the correct thing to use when building data structures using
/// raw pointers, but is ultimately more dangerous to use because of its additional
/// properties. If you're not sure if you should use `NonNull<T>`, just use `*mut T`!
///
/// Unlike `*mut T`, the pointer must always be non-null, even if the pointer
/// is never dereferenced. This is so that enums may use this forbidden value
/// as a discriminant -- `Option<NonNull<T>>` has the same size as `*mut T`.
/// However the pointer may still dangle if it isn't dereferenced.
///
/// Unlike `*mut T`, `NonNull<T>` was chosen to be covariant over `T`. This makes it
/// possible to use `NonNull<T>` when building covariant types, but introduces the
/// risk of unsoundness if used in a type that shouldn't actually be covariant.
/// (The opposite choice was made for `*mut T` even though technically the unsoundness
/// could only be caused by calling unsafe functions.)
///
/// Covariance is correct for most safe abstractions, such as `Box`, `Rc`, `Arc`, `Vec`,
/// and `LinkedList`. This is the case because they provide a public API that follows the
/// normal shared XOR mutable rules of Rust.
///
/// If your type cannot safely be covariant, you must ensure it contains some
/// additional field to provide invariance. Often this field will be a [`PhantomData`]
/// type like `PhantomData<Cell<T>>` or `PhantomData<&'a mut T>`.
///
/// Notice that `NonNull<T>` has a `From` instance for `&T`. However, this does
/// not change the fact that mutating through a (pointer derived from a) shared
/// reference is undefined behavior unless the mutation happens inside an
/// [`UnsafeCell<T>`]. The same goes for creating a mutable reference from a shared
/// reference. When using this `From` instance without an `UnsafeCell<T>`,
/// it is your responsibility to ensure that `as_mut` is never called, and `as_ptr`
/// is never used for mutation.
///
/// # Representation
///
/// Thanks to the [null pointer optimization],
/// `NonNull<T>` and `Option<NonNull<T>>`
/// are guaranteed to have the same size and alignment:
///
/// ```
/// # use std::mem::{size_of, align_of};
/// use std::ptr::NonNull;
///
/// assert_eq!(size_of::<NonNull<i16>>(), size_of::<Option<NonNull<i16>>>());
/// assert_eq!(align_of::<NonNull<i16>>(), align_of::<Option<NonNull<i16>>>());
///
/// assert_eq!(size_of::<NonNull<str>>(), size_of::<Option<NonNull<str>>>());
/// assert_eq!(align_of::<NonNull<str>>(), align_of::<Option<NonNull<str>>>());
/// ```
///
/// [covariant]: https://doc.rust-lang.org/reference/subtyping.html
/// [`PhantomData`]: crate::marker::PhantomData
/// [`UnsafeCell<T>`]: crate::cell::UnsafeCell
/// [null pointer optimization]: crate::option#representation
#[stable(feature = "nonnull", since = "1.25.0")]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
#[rustc_diagnostic_item = "NonNull"]
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}

/// `NonNull` pointers are not `Send` because the data they reference may be aliased.
// N.B., this impl is unnecessary, but should provide better error messages.
#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> !Send for NonNull<T> {}

/// `NonNull` pointers are not `Sync` because the data they reference may be aliased.
// N.B., this impl is unnecessary, but should provide better error messages.
#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> !Sync for NonNull<T> {}

impl<T: Sized> NonNull<T> {
    /// Creates a new `NonNull` that is dangling, but well-aligned.
    ///
    /// This is useful for initializing types which lazily allocate, like
    /// `Vec::new` does.
    ///
    /// Note that the pointer value may potentially represent a valid pointer to
    /// a `T`, which means this must not be used as a "not yet initialized"
    /// sentinel value. Types that lazily allocate must track initialization by
    /// some other means.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let ptr = NonNull::<u32>::dangling();
    /// // Important: don't try to access the value of `ptr` without
    /// // initializing it first! The pointer is not null but isn't valid either!
    /// ```
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[rustc_const_stable(feature = "const_nonnull_dangling", since = "1.36.0")]
    #[must_use]
    #[inline]
    pub const fn dangling() -> Self {
        // SAFETY: mem::align_of() returns a non-zero usize which is then casted
        // to a *mut T. Therefore, `ptr` is not null and the conditions for
        // calling new_unchecked() are respected.
        unsafe {
            let ptr = crate::ptr::dangling_mut::<T>();
            NonNull::new_unchecked(ptr)
        }
    }

    /// Returns a shared references to the value. In contrast to [`as_ref`], this does not require
    /// that the value has to be initialized.
    ///
    /// For the mutable counterpart see [`as_uninit_mut`].
    ///
    /// [`as_ref`]: NonNull::as_ref
    /// [`as_uninit_mut`]: NonNull::as_uninit_mut
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    ///
    /// * It must be "dereferenceable" in the sense defined in [the module documentation].
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get mutated (except inside `UnsafeCell`).
    ///
    /// This applies even if the result of this method is unused!
    ///
    /// [the module documentation]: crate::ptr#safety
    #[inline]
    #[must_use]
    #[unstable(feature = "ptr_as_uninit", issue = "75402")]
    #[rustc_const_unstable(feature = "const_ptr_as_ref", issue = "91822")]
    pub const unsafe fn as_uninit_ref<'a>(self) -> &'a MaybeUninit<T> {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { &*self.cast().as_ptr() }
    }

    /// Returns a unique references to the value. In contrast to [`as_mut`], this does not require
    /// that the value has to be initialized.
    ///
    /// For the shared counterpart see [`as_uninit_ref`].
    ///
    /// [`as_mut`]: NonNull::as_mut
    /// [`as_uninit_ref`]: NonNull::as_uninit_ref
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    ///
    /// * It must be "dereferenceable" in the sense defined in [the module documentation].
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get accessed (read or written) through any other pointer.
    ///
    /// This applies even if the result of this method is unused!
    ///
    /// [the module documentation]: crate::ptr#safety
    #[inline]
    #[must_use]
    #[unstable(feature = "ptr_as_uninit", issue = "75402")]
    #[rustc_const_unstable(feature = "const_ptr_as_ref", issue = "91822")]
    pub const unsafe fn as_uninit_mut<'a>(self) -> &'a mut MaybeUninit<T> {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        unsafe { &mut *self.cast().as_ptr() }
    }
}

impl<T: ?Sized> NonNull<T> {
    /// Creates a new `NonNull`.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = unsafe { NonNull::new_unchecked(&mut x as *mut _) };
    /// ```
    ///
    /// *Incorrect* usage of this function:
    ///
    /// ```rust,no_run
    /// use std::ptr::NonNull;
    ///
    /// // NEVER DO THAT!!! This is undefined behavior. ⚠️
    /// let ptr = unsafe { NonNull::<u32>::new_unchecked(std::ptr::null_mut()) };
    /// ```
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[rustc_const_stable(feature = "const_nonnull_new_unchecked", since = "1.25.0")]
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *mut T) -> Self {
        // SAFETY: the caller must guarantee that `ptr` is non-null.
        unsafe {
            assert_unsafe_precondition!(
                check_language_ub,
                "NonNull::new_unchecked requires that the pointer is non-null",
                (ptr: *mut () = ptr as *mut ()) => !ptr.is_null()
            );
            NonNull { pointer: ptr as _ }
        }
    }

    /// Creates a new `NonNull` if `ptr` is non-null.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::<u32>::new(&mut x as *mut _).expect("ptr is null!");
    ///
    /// if let Some(ptr) = NonNull::<u32>::new(std::ptr::null_mut()) {
    ///     unreachable!();
    /// }
    /// ```
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[rustc_const_unstable(feature = "const_nonnull_new", issue = "93235")]
    #[inline]
    pub const fn new(ptr: *mut T) -> Option<Self> {
        if !ptr.is_null() {
            // SAFETY: The pointer is already checked and is not null
            Some(unsafe { Self::new_unchecked(ptr) })
        } else {
            None
        }
    }

    /// Performs the same functionality as [`std::ptr::from_raw_parts`], except that a
    /// `NonNull` pointer is returned, as opposed to a raw `*const` pointer.
    ///
    /// See the documentation of [`std::ptr::from_raw_parts`] for more details.
    ///
    /// [`std::ptr::from_raw_parts`]: crate::ptr::from_raw_parts
    #[unstable(feature = "ptr_metadata", issue = "81513")]
    #[rustc_const_unstable(feature = "ptr_metadata", issue = "81513")]
    #[inline]
    pub const fn from_raw_parts(
        data_pointer: NonNull<()>,
        metadata: <T as super::Pointee>::Metadata,
    ) -> NonNull<T> {
        // SAFETY: The result of `ptr::from::raw_parts_mut` is non-null because `data_pointer` is.
        unsafe {
            NonNull::new_unchecked(super::from_raw_parts_mut(data_pointer.as_ptr(), metadata))
        }
    }

    /// Decompose a (possibly wide) pointer into its data pointer and metadata components.
    ///
    /// The pointer can be later reconstructed with [`NonNull::from_raw_parts`].
    #[unstable(feature = "ptr_metadata", issue = "81513")]
    #[rustc_const_unstable(feature = "ptr_metadata", issue = "81513")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn to_raw_parts(self) -> (NonNull<()>, <T as super::Pointee>::Metadata) {
        (self.cast(), super::metadata(self.as_ptr()))
    }

    /// Gets the "address" portion of the pointer.
    ///
    /// For more details see the equivalent method on a raw pointer, [`pointer::addr`].
    ///
    /// This API and its claimed semantics are part of the Strict Provenance experiment,
    /// see the [`ptr` module documentation][crate::ptr].
    #[must_use]
    #[inline]
    #[unstable(feature = "strict_provenance", issue = "95228")]
    pub fn addr(self) -> NonZero<usize> {
        // SAFETY: The pointer is guaranteed by the type to be non-null,
        // meaning that the address will be non-zero.
        unsafe { NonZero::new_unchecked(self.pointer.addr()) }
    }

    /// Creates a new pointer with the given address.
    ///
    /// For more details see the equivalent method on a raw pointer, [`pointer::with_addr`].
    ///
    /// This API and its claimed semantics are part of the Strict Provenance experiment,
    /// see the [`ptr` module documentation][crate::ptr].
    #[must_use]
    #[inline]
    #[unstable(feature = "strict_provenance", issue = "95228")]
    pub fn with_addr(self, addr: NonZero<usize>) -> Self {
        // SAFETY: The result of `ptr::from::with_addr` is non-null because `addr` is guaranteed to be non-zero.
        unsafe { NonNull::new_unchecked(self.pointer.with_addr(addr.get()) as *mut _) }
    }

    /// Creates a new pointer by mapping `self`'s address to a new one.
    ///
    /// For more details see the equivalent method on a raw pointer, [`pointer::map_addr`].
    ///
    /// This API and its claimed semantics are part of the Strict Provenance experiment,
    /// see the [`ptr` module documentation][crate::ptr].
    #[must_use]
    #[inline]
    #[unstable(feature = "strict_provenance", issue = "95228")]
    pub fn map_addr(self, f: impl FnOnce(NonZero<usize>) -> NonZero<usize>) -> Self {
        self.with_addr(f(self.addr()))
    }

    /// Acquires the underlying `*mut` pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::new(&mut x).expect("ptr is null!");
    ///
    /// let x_value = unsafe { *ptr.as_ptr() };
    /// assert_eq!(x_value, 0);
    ///
    /// unsafe { *ptr.as_ptr() += 2; }
    /// let x_value = unsafe { *ptr.as_ptr() };
    /// assert_eq!(x_value, 2);
    /// ```
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[rustc_const_stable(feature = "const_nonnull_as_ptr", since = "1.32.0")]
    #[rustc_never_returns_null_ptr]
    #[must_use]
    #[inline(always)]
    pub const fn as_ptr(self) -> *mut T {
        self.pointer as *mut T
    }

    /// Returns a shared reference to the value. If the value may be uninitialized, [`as_uninit_ref`]
    /// must be used instead.
    ///
    /// For the mutable counterpart see [`as_mut`].
    ///
    /// [`as_uninit_ref`]: NonNull::as_uninit_ref
    /// [`as_mut`]: NonNull::as_mut
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    ///
    /// * It must be "dereferenceable" in the sense defined in [the module documentation].
    ///
    /// * The pointer must point to an initialized instance of `T`.
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get mutated (except inside `UnsafeCell`).
    ///
    /// This applies even if the result of this method is unused!
    /// (The part about being initialized is not yet fully decided, but until
    /// it is, the only safe approach is to ensure that they are indeed initialized.)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::new(&mut x as *mut _).expect("ptr is null!");
    ///
    /// let ref_x = unsafe { ptr.as_ref() };
    /// println!("{ref_x}");
    /// ```
    ///
    /// [the module documentation]: crate::ptr#safety
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[rustc_const_stable(feature = "const_nonnull_as_ref", since = "1.73.0")]
    #[must_use]
    #[inline(always)]
    pub const unsafe fn as_ref<'a>(&self) -> &'a T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a reference.
        // `cast_const` avoids a mutable raw pointer deref.
        unsafe { &*self.as_ptr().cast_const() }
    }

    /// Returns a unique reference to the value. If the value may be uninitialized, [`as_uninit_mut`]
    /// must be used instead.
    ///
    /// For the shared counterpart see [`as_ref`].
    ///
    /// [`as_uninit_mut`]: NonNull::as_uninit_mut
    /// [`as_ref`]: NonNull::as_ref
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be properly aligned.
    ///
    /// * It must be "dereferenceable" in the sense defined in [the module documentation].
    ///
    /// * The pointer must point to an initialized instance of `T`.
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get accessed (read or written) through any other pointer.
    ///
    /// This applies even if the result of this method is unused!
    /// (The part about being initialized is not yet fully decided, but until
    /// it is, the only safe approach is to ensure that they are indeed initialized.)
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let mut ptr = NonNull::new(&mut x).expect("null pointer");
    ///
    /// let x_ref = unsafe { ptr.as_mut() };
    /// assert_eq!(*x_ref, 0);
    /// *x_ref += 2;
    /// assert_eq!(*x_ref, 2);
    /// ```
    ///
    /// [the module documentation]: crate::ptr#safety
    #[stable(feature = "nonnull", since = "1.25.0")]
    #[rustc_const_unstable(feature = "const_ptr_as_ref", issue = "91822")]
    #[must_use]
    #[inline(always)]
    pub const unsafe fn as_mut<'a>(&mut self) -> &'a mut T {
        // SAFETY: the caller must guarantee that `self` meets all the
        // requirements for a mutable reference.
        unsafe { &mut *self.as_ptr() }
    }

    /// Casts to a pointer of another type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ptr::NonNull;
    ///
    /// let mut x = 0u32;
    /// let ptr = NonNull::new(&mut x as *mut _).expect("null pointer");
    ///
    /// let casted_ptr = ptr.cast::<i8>();
    /// let raw_ptr: *mut i8 = casted_ptr.as_ptr();
    /// ```
    #[stable(feature = "nonnull_cast", since = "1.27.0")]
    #[rustc_const_stable(feature = "const_nonnull_cast", since = "1.36.0")]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    #[inline]
    pub const fn cast<U>(self) -> NonNull<U> {
        // SAFETY: `self` is a `NonNull` pointer which is necessarily non-null
        unsafe { NonNull { pointer: self.as_ptr() as *mut U } }
    }

    /// Calculates the offset from a pointer.
    ///
    /// `count` is in units of T; e.g., a `count` of 3 represents a pointer
    /// offset of `3 * size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both the starting and resulting pointer must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * The computed offset, **in bytes**, cannot overflow an `isize`.
    ///
    /// * The offset being in bounds cannot rely on "wrapping around" the address
    ///   space. That is, the infinite-precision sum, **in bytes** must fit in a usize.
    ///
    /// The compiler and standard library generally tries to ensure allocations
    /// never reach a size where an offset is a concern. For instance, `Vec`
    /// and `Box` ensure they never allocate more than `isize::MAX` bytes, so
    /// `vec.as_ptr().add(vec.len())` is always safe.
    ///
    /// Most platforms fundamentally can't even construct such an allocation.
    /// For instance, no known 64-bit platform can ever serve a request
    /// for 2<sup>63</sup> bytes due to page-table limitations or splitting the address space.
    /// However, some 32-bit and 16-bit platforms may successfully serve a request for
    /// more than `isize::MAX` bytes with things like Physical Address
    /// Extension. As such, memory acquired directly from allocators or memory
    /// mapped files *may* be too large to handle with this function.
    ///
    /// [allocated object]: crate::ptr#allocated-object
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(non_null_convenience)]
    /// use std::ptr::NonNull;
    ///
    /// let mut s = [1, 2, 3];
    /// let ptr: NonNull<u32> = NonNull::new(s.as_mut_ptr()).unwrap();
    ///
    /// unsafe {
    ///     println!("{}", ptr.offset(1).read());
    ///     println!("{}", ptr.offset(2).read());
    /// }
    /// ```
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[must_use = "returns a new pointer rather than modifying its argument"]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn offset(self, count: isize) -> NonNull<T>
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset`.
        // Additionally safety contract of `offset` guarantees that the resulting pointer is
        // pointing to an allocation, there can't be an allocation at null, thus it's safe to
        // construct `NonNull`.
        unsafe { NonNull { pointer: intrinsics::offset(self.pointer, count) } }
    }

    /// Calculates the offset from a pointer in bytes.
    ///
    /// `count` is in units of **bytes**.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [offset][pointer::offset] on it. See that method for documentation
    /// and safety requirements.
    ///
    /// For non-`Sized` pointees this operation changes only the data pointer,
    /// leaving the metadata untouched.
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[must_use]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_offset(self, count: isize) -> Self {
        // SAFETY: the caller must uphold the safety contract for `offset` and `byte_offset` has
        // the same safety contract.
        // Additionally safety contract of `offset` guarantees that the resulting pointer is
        // pointing to an allocation, there can't be an allocation at null, thus it's safe to
        // construct `NonNull`.
        unsafe { NonNull { pointer: self.pointer.byte_offset(count) } }
    }

    /// Calculates the offset from a pointer (convenience for `.offset(count as isize)`).
    ///
    /// `count` is in units of T; e.g., a `count` of 3 represents a pointer
    /// offset of `3 * size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both the starting and resulting pointer must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * The computed offset, **in bytes**, cannot overflow an `isize`.
    ///
    /// * The offset being in bounds cannot rely on "wrapping around" the address
    ///   space. That is, the infinite-precision sum must fit in a `usize`.
    ///
    /// The compiler and standard library generally tries to ensure allocations
    /// never reach a size where an offset is a concern. For instance, `Vec`
    /// and `Box` ensure they never allocate more than `isize::MAX` bytes, so
    /// `vec.as_ptr().add(vec.len())` is always safe.
    ///
    /// Most platforms fundamentally can't even construct such an allocation.
    /// For instance, no known 64-bit platform can ever serve a request
    /// for 2<sup>63</sup> bytes due to page-table limitations or splitting the address space.
    /// However, some 32-bit and 16-bit platforms may successfully serve a request for
    /// more than `isize::MAX` bytes with things like Physical Address
    /// Extension. As such, memory acquired directly from allocators or memory
    /// mapped files *may* be too large to handle with this function.
    ///
    /// [allocated object]: crate::ptr#allocated-object
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(non_null_convenience)]
    /// use std::ptr::NonNull;
    ///
    /// let s: &str = "123";
    /// let ptr: NonNull<u8> = NonNull::new(s.as_ptr().cast_mut()).unwrap();
    ///
    /// unsafe {
    ///     println!("{}", ptr.add(1).read() as char);
    ///     println!("{}", ptr.add(2).read() as char);
    /// }
    /// ```
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[must_use = "returns a new pointer rather than modifying its argument"]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset`.
        // Additionally safety contract of `offset` guarantees that the resulting pointer is
        // pointing to an allocation, there can't be an allocation at null, thus it's safe to
        // construct `NonNull`.
        unsafe { NonNull { pointer: intrinsics::offset(self.pointer, count) } }
    }

    /// Calculates the offset from a pointer in bytes (convenience for `.byte_offset(count as isize)`).
    ///
    /// `count` is in units of bytes.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [`add`][NonNull::add] on it. See that method for documentation
    /// and safety requirements.
    ///
    /// For non-`Sized` pointees this operation changes only the data pointer,
    /// leaving the metadata untouched.
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[must_use]
    #[inline(always)]
    #[rustc_allow_const_fn_unstable(set_ptr_value)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_add(self, count: usize) -> Self {
        // SAFETY: the caller must uphold the safety contract for `add` and `byte_add` has the same
        // safety contract.
        // Additionally safety contract of `add` guarantees that the resulting pointer is pointing
        // to an allocation, there can't be an allocation at null, thus it's safe to construct
        // `NonNull`.
        unsafe { NonNull { pointer: self.pointer.byte_add(count) } }
    }

    /// Calculates the offset from a pointer (convenience for
    /// `.offset((count as isize).wrapping_neg())`).
    ///
    /// `count` is in units of T; e.g., a `count` of 3 represents a pointer
    /// offset of `3 * size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both the starting and resulting pointer must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * The computed offset cannot exceed `isize::MAX` **bytes**.
    ///
    /// * The offset being in bounds cannot rely on "wrapping around" the address
    ///   space. That is, the infinite-precision sum must fit in a usize.
    ///
    /// The compiler and standard library generally tries to ensure allocations
    /// never reach a size where an offset is a concern. For instance, `Vec`
    /// and `Box` ensure they never allocate more than `isize::MAX` bytes, so
    /// `vec.as_ptr().add(vec.len()).sub(vec.len())` is always safe.
    ///
    /// Most platforms fundamentally can't even construct such an allocation.
    /// For instance, no known 64-bit platform can ever serve a request
    /// for 2<sup>63</sup> bytes due to page-table limitations or splitting the address space.
    /// However, some 32-bit and 16-bit platforms may successfully serve a request for
    /// more than `isize::MAX` bytes with things like Physical Address
    /// Extension. As such, memory acquired directly from allocators or memory
    /// mapped files *may* be too large to handle with this function.
    ///
    /// [allocated object]: crate::ptr#allocated-object
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(non_null_convenience)]
    /// use std::ptr::NonNull;
    ///
    /// let s: &str = "123";
    ///
    /// unsafe {
    ///     let end: NonNull<u8> = NonNull::new(s.as_ptr().cast_mut()).unwrap().add(3);
    ///     println!("{}", end.sub(1).read() as char);
    ///     println!("{}", end.sub(2).read() as char);
    /// }
    /// ```
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[must_use = "returns a new pointer rather than modifying its argument"]
    // We could always go back to wrapping if unchecked becomes unacceptable
    #[rustc_allow_const_fn_unstable(const_int_unchecked_arith)]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn sub(self, count: usize) -> Self
    where
        T: Sized,
    {
        if T::IS_ZST {
            // Pointer arithmetic does nothing when the pointee is a ZST.
            self
        } else {
            // SAFETY: the caller must uphold the safety contract for `offset`.
            // Because the pointee is *not* a ZST, that means that `count` is
            // at most `isize::MAX`, and thus the negation cannot overflow.
            unsafe { self.offset(intrinsics::unchecked_sub(0, count as isize)) }
        }
    }

    /// Calculates the offset from a pointer in bytes (convenience for
    /// `.byte_offset((count as isize).wrapping_neg())`).
    ///
    /// `count` is in units of bytes.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [`sub`][NonNull::sub] on it. See that method for documentation
    /// and safety requirements.
    ///
    /// For non-`Sized` pointees this operation changes only the data pointer,
    /// leaving the metadata untouched.
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[must_use]
    #[inline(always)]
    #[rustc_allow_const_fn_unstable(set_ptr_value)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_sub(self, count: usize) -> Self {
        // SAFETY: the caller must uphold the safety contract for `sub` and `byte_sub` has the same
        // safety contract.
        // Additionally safety contract of `sub` guarantees that the resulting pointer is pointing
        // to an allocation, there can't be an allocation at null, thus it's safe to construct
        // `NonNull`.
        unsafe { NonNull { pointer: self.pointer.byte_sub(count) } }
    }

    /// Calculates the distance between two pointers. The returned value is in
    /// units of T: the distance in bytes divided by `mem::size_of::<T>()`.
    ///
    /// This is equivalent to `(self as isize - origin as isize) / (mem::size_of::<T>() as isize)`,
    /// except that it has a lot more opportunities for UB, in exchange for the compiler
    /// better understanding what you are doing.
    ///
    /// The primary motivation of this method is for computing the `len` of an array/slice
    /// of `T` that you are currently representing as a "start" and "end" pointer
    /// (and "end" is "one past the end" of the array).
    /// In that case, `end.offset_from(start)` gets you the length of the array.
    ///
    /// All of the following safety requirements are trivially satisfied for this usecase.
    ///
    /// [`offset`]: #method.offset
    ///
    /// # Safety
    ///
    /// If any of the following conditions are violated, the result is Undefined
    /// Behavior:
    ///
    /// * Both `self` and `origin` must be either in bounds or one
    ///   byte past the end of the same [allocated object].
    ///
    /// * Both pointers must be *derived from* a pointer to the same object.
    ///   (See below for an example.)
    ///
    /// * The distance between the pointers, in bytes, must be an exact multiple
    ///   of the size of `T`.
    ///
    /// * The distance between the pointers, **in bytes**, cannot overflow an `isize`.
    ///
    /// * The distance being in bounds cannot rely on "wrapping around" the address space.
    ///
    /// Rust types are never larger than `isize::MAX` and Rust allocations never wrap around the
    /// address space, so two pointers within some value of any Rust type `T` will always satisfy
    /// the last two conditions. The standard library also generally ensures that allocations
    /// never reach a size where an offset is a concern. For instance, `Vec` and `Box` ensure they
    /// never allocate more than `isize::MAX` bytes, so `ptr_into_vec.offset_from(vec.as_ptr())`
    /// always satisfies the last two conditions.
    ///
    /// Most platforms fundamentally can't even construct such a large allocation.
    /// For instance, no known 64-bit platform can ever serve a request
    /// for 2<sup>63</sup> bytes due to page-table limitations or splitting the address space.
    /// However, some 32-bit and 16-bit platforms may successfully serve a request for
    /// more than `isize::MAX` bytes with things like Physical Address
    /// Extension. As such, memory acquired directly from allocators or memory
    /// mapped files *may* be too large to handle with this function.
    /// (Note that [`offset`] and [`add`] also have a similar limitation and hence cannot be used on
    /// such large allocations either.)
    ///
    /// The requirement for pointers to be derived from the same allocated object is primarily
    /// needed for `const`-compatibility: the distance between pointers into *different* allocated
    /// objects is not known at compile-time. However, the requirement also exists at
    /// runtime and may be exploited by optimizations. If you wish to compute the difference between
    /// pointers that are not guaranteed to be from the same allocation, use `(self as isize -
    /// origin as isize) / mem::size_of::<T>()`.
    // FIXME: recommend `addr()` instead of `as usize` once that is stable.
    ///
    /// [`add`]: #method.add
    /// [allocated object]: crate::ptr#allocated-object
    ///
    /// # Panics
    ///
    /// This function panics if `T` is a Zero-Sized Type ("ZST").
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// #![feature(non_null_convenience)]
    /// use std::ptr::NonNull;
    ///
    /// let a = [0; 5];
    /// let ptr1: NonNull<u32> = NonNull::from(&a[1]);
    /// let ptr2: NonNull<u32> = NonNull::from(&a[3]);
    /// unsafe {
    ///     assert_eq!(ptr2.offset_from(ptr1), 2);
    ///     assert_eq!(ptr1.offset_from(ptr2), -2);
    ///     assert_eq!(ptr1.offset(2), ptr2);
    ///     assert_eq!(ptr2.offset(-2), ptr1);
    /// }
    /// ```
    ///
    /// *Incorrect* usage:
    ///
    /// ```rust,no_run
    /// #![feature(non_null_convenience, strict_provenance)]
    /// use std::ptr::NonNull;
    ///
    /// let ptr1 = NonNull::new(Box::into_raw(Box::new(0u8))).unwrap();
    /// let ptr2 = NonNull::new(Box::into_raw(Box::new(1u8))).unwrap();
    /// let diff = (ptr2.addr().get() as isize).wrapping_sub(ptr1.addr().get() as isize);
    /// // Make ptr2_other an "alias" of ptr2, but derived from ptr1.
    /// let ptr2_other = NonNull::new(ptr1.as_ptr().wrapping_byte_offset(diff)).unwrap();
    /// assert_eq!(ptr2.addr(), ptr2_other.addr());
    /// // Since ptr2_other and ptr2 are derived from pointers to different objects,
    /// // computing their offset is undefined behavior, even though
    /// // they point to the same address!
    /// unsafe {
    ///     let zero = ptr2_other.offset_from(ptr2); // Undefined Behavior
    /// }
    /// ```
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn offset_from(self, origin: NonNull<T>) -> isize
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `offset_from`.
        unsafe { self.pointer.offset_from(origin.pointer) }
    }

    /// Calculates the distance between two pointers. The returned value is in
    /// units of **bytes**.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [`offset_from`][NonNull::offset_from] on it. See that method for
    /// documentation and safety requirements.
    ///
    /// For non-`Sized` pointees this operation considers only the data pointers,
    /// ignoring the metadata.
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn byte_offset_from<U: ?Sized>(self, origin: NonNull<U>) -> isize {
        // SAFETY: the caller must uphold the safety contract for `byte_offset_from`.
        unsafe { self.pointer.byte_offset_from(origin.pointer) }
    }

    // N.B. `wrapping_offset``, `wrapping_add`, etc are not implemented because they can wrap to null

    /// Calculates the distance between two pointers, *where it's known that
    /// `self` is equal to or greater than `origin`*. The returned value is in
    /// units of T: the distance in bytes is divided by `mem::size_of::<T>()`.
    ///
    /// This computes the same value that [`offset_from`](#method.offset_from)
    /// would compute, but with the added precondition that the offset is
    /// guaranteed to be non-negative.  This method is equivalent to
    /// `usize::try_from(self.offset_from(origin)).unwrap_unchecked()`,
    /// but it provides slightly more information to the optimizer, which can
    /// sometimes allow it to optimize slightly better with some backends.
    ///
    /// This method can be though of as recovering the `count` that was passed
    /// to [`add`](#method.add) (or, with the parameters in the other order,
    /// to [`sub`](#method.sub)).  The following are all equivalent, assuming
    /// that their safety preconditions are met:
    /// ```rust
    /// # #![feature(non_null_convenience)]
    /// # unsafe fn blah(ptr: std::ptr::NonNull<u32>, origin: std::ptr::NonNull<u32>, count: usize) -> bool {
    /// ptr.sub_ptr(origin) == count
    /// # &&
    /// origin.add(count) == ptr
    /// # &&
    /// ptr.sub(count) == origin
    /// # }
    /// ```
    ///
    /// # Safety
    ///
    /// - The distance between the pointers must be non-negative (`self >= origin`)
    ///
    /// - *All* the safety conditions of [`offset_from`](#method.offset_from)
    ///   apply to this method as well; see it for the full details.
    ///
    /// Importantly, despite the return type of this method being able to represent
    /// a larger offset, it's still *not permitted* to pass pointers which differ
    /// by more than `isize::MAX` *bytes*.  As such, the result of this method will
    /// always be less than or equal to `isize::MAX as usize`.
    ///
    /// # Panics
    ///
    /// This function panics if `T` is a Zero-Sized Type ("ZST").
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(non_null_convenience)]
    /// use std::ptr::NonNull;
    ///
    /// let a = [0; 5];
    /// let ptr1: NonNull<u32> = NonNull::from(&a[1]);
    /// let ptr2: NonNull<u32> = NonNull::from(&a[3]);
    /// unsafe {
    ///     assert_eq!(ptr2.sub_ptr(ptr1), 2);
    ///     assert_eq!(ptr1.add(2), ptr2);
    ///     assert_eq!(ptr2.sub(2), ptr1);
    ///     assert_eq!(ptr2.sub_ptr(ptr2), 0);
    /// }
    ///
    /// // This would be incorrect, as the pointers are not correctly ordered:
    /// // ptr1.sub_ptr(ptr2)
    /// ```
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    // #[unstable(feature = "ptr_sub_ptr", issue = "95892")]
    // #[rustc_const_unstable(feature = "const_ptr_sub_ptr", issue = "95892")]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn sub_ptr(self, subtracted: NonNull<T>) -> usize
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `sub_ptr`.
        unsafe { self.pointer.sub_ptr(subtracted.pointer) }
    }

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// See [`ptr::read`] for safety concerns and examples.
    ///
    /// [`ptr::read`]: crate::ptr::read()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn read(self) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `read`.
        unsafe { ptr::read(self.pointer) }
    }

    /// Performs a volatile read of the value from `self` without moving it. This
    /// leaves the memory in `self` unchanged.
    ///
    /// Volatile operations are intended to act on I/O memory, and are guaranteed
    /// to not be elided or reordered by the compiler across other volatile
    /// operations.
    ///
    /// See [`ptr::read_volatile`] for safety concerns and examples.
    ///
    /// [`ptr::read_volatile`]: crate::ptr::read_volatile()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn read_volatile(self) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `read_volatile`.
        unsafe { ptr::read_volatile(self.pointer) }
    }

    /// Reads the value from `self` without moving it. This leaves the
    /// memory in `self` unchanged.
    ///
    /// Unlike `read`, the pointer may be unaligned.
    ///
    /// See [`ptr::read_unaligned`] for safety concerns and examples.
    ///
    /// [`ptr::read_unaligned`]: crate::ptr::read_unaligned()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn read_unaligned(self) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `read_unaligned`.
        unsafe { ptr::read_unaligned(self.pointer) }
    }

    /// Copies `count * size_of<T>` bytes from `self` to `dest`. The source
    /// and destination may overlap.
    ///
    /// NOTE: this has the *same* argument order as [`ptr::copy`].
    ///
    /// See [`ptr::copy`] for safety concerns and examples.
    ///
    /// [`ptr::copy`]: crate::ptr::copy()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn copy_to(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `copy`.
        unsafe { ptr::copy(self.pointer, dest.as_ptr(), count) }
    }

    /// Copies `count * size_of<T>` bytes from `self` to `dest`. The source
    /// and destination may *not* overlap.
    ///
    /// NOTE: this has the *same* argument order as [`ptr::copy_nonoverlapping`].
    ///
    /// See [`ptr::copy_nonoverlapping`] for safety concerns and examples.
    ///
    /// [`ptr::copy_nonoverlapping`]: crate::ptr::copy_nonoverlapping()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn copy_to_nonoverlapping(self, dest: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `copy_nonoverlapping`.
        unsafe { ptr::copy_nonoverlapping(self.pointer, dest.as_ptr(), count) }
    }

    /// Copies `count * size_of<T>` bytes from `src` to `self`. The source
    /// and destination may overlap.
    ///
    /// NOTE: this has the *opposite* argument order of [`ptr::copy`].
    ///
    /// See [`ptr::copy`] for safety concerns and examples.
    ///
    /// [`ptr::copy`]: crate::ptr::copy()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn copy_from(self, src: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `copy`.
        unsafe { ptr::copy(src.pointer, self.as_ptr(), count) }
    }

    /// Copies `count * size_of<T>` bytes from `src` to `self`. The source
    /// and destination may *not* overlap.
    ///
    /// NOTE: this has the *opposite* argument order of [`ptr::copy_nonoverlapping`].
    ///
    /// See [`ptr::copy_nonoverlapping`] for safety concerns and examples.
    ///
    /// [`ptr::copy_nonoverlapping`]: crate::ptr::copy_nonoverlapping()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn copy_from_nonoverlapping(self, src: NonNull<T>, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `copy_nonoverlapping`.
        unsafe { ptr::copy_nonoverlapping(src.pointer, self.as_ptr(), count) }
    }

    /// Executes the destructor (if any) of the pointed-to value.
    ///
    /// See [`ptr::drop_in_place`] for safety concerns and examples.
    ///
    /// [`ptr::drop_in_place`]: crate::ptr::drop_in_place()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    pub unsafe fn drop_in_place(self) {
        // SAFETY: the caller must uphold the safety contract for `drop_in_place`.
        unsafe { ptr::drop_in_place(self.as_ptr()) }
    }

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    ///
    /// See [`ptr::write`] for safety concerns and examples.
    ///
    /// [`ptr::write`]: crate::ptr::write()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    //#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn write(self, val: T)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `write`.
        unsafe { ptr::write(self.as_ptr(), val) }
    }

    /// Invokes memset on the specified pointer, setting `count * size_of::<T>()`
    /// bytes of memory starting at `self` to `val`.
    ///
    /// See [`ptr::write_bytes`] for safety concerns and examples.
    ///
    /// [`ptr::write_bytes`]: crate::ptr::write_bytes()
    #[doc(alias = "memset")]
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    //#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn write_bytes(self, val: u8, count: usize)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `write_bytes`.
        unsafe { ptr::write_bytes(self.as_ptr(), val, count) }
    }

    /// Performs a volatile write of a memory location with the given value without
    /// reading or dropping the old value.
    ///
    /// Volatile operations are intended to act on I/O memory, and are guaranteed
    /// to not be elided or reordered by the compiler across other volatile
    /// operations.
    ///
    /// See [`ptr::write_volatile`] for safety concerns and examples.
    ///
    /// [`ptr::write_volatile`]: crate::ptr::write_volatile()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn write_volatile(self, val: T)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `write_volatile`.
        unsafe { ptr::write_volatile(self.as_ptr(), val) }
    }

    /// Overwrites a memory location with the given value without reading or
    /// dropping the old value.
    ///
    /// Unlike `write`, the pointer may be unaligned.
    ///
    /// See [`ptr::write_unaligned`] for safety concerns and examples.
    ///
    /// [`ptr::write_unaligned`]: crate::ptr::write_unaligned()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    //#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
    #[inline(always)]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub const unsafe fn write_unaligned(self, val: T)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `write_unaligned`.
        unsafe { ptr::write_unaligned(self.as_ptr(), val) }
    }

    /// Replaces the value at `self` with `src`, returning the old
    /// value, without dropping either.
    ///
    /// See [`ptr::replace`] for safety concerns and examples.
    ///
    /// [`ptr::replace`]: crate::ptr::replace()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[inline(always)]
    pub unsafe fn replace(self, src: T) -> T
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `replace`.
        unsafe { ptr::replace(self.as_ptr(), src) }
    }

    /// Swaps the values at two mutable locations of the same type, without
    /// deinitializing either. They may overlap, unlike `mem::swap` which is
    /// otherwise equivalent.
    ///
    /// See [`ptr::swap`] for safety concerns and examples.
    ///
    /// [`ptr::swap`]: crate::ptr::swap()
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    //#[rustc_const_unstable(feature = "const_swap", issue = "83163")]
    #[inline(always)]
    pub const unsafe fn swap(self, with: NonNull<T>)
    where
        T: Sized,
    {
        // SAFETY: the caller must uphold the safety contract for `swap`.
        unsafe { ptr::swap(self.as_ptr(), with.as_ptr()) }
    }

    /// Computes the offset that needs to be applied to the pointer in order to make it aligned to
    /// `align`.
    ///
    /// If it is not possible to align the pointer, the implementation returns
    /// `usize::MAX`. It is permissible for the implementation to *always*
    /// return `usize::MAX`. Only your algorithm's performance can depend
    /// on getting a usable offset here, not its correctness.
    ///
    /// The offset is expressed in number of `T` elements, and not bytes.
    ///
    /// There are no guarantees whatsoever that offsetting the pointer will not overflow or go
    /// beyond the allocation that the pointer points into. It is up to the caller to ensure that
    /// the returned offset is correct in all terms other than alignment.
    ///
    /// # Panics
    ///
    /// The function panics if `align` is not a power-of-two.
    ///
    /// # Examples
    ///
    /// Accessing adjacent `u8` as `u16`
    ///
    /// ```
    /// #![feature(non_null_convenience)]
    /// use std::mem::align_of;
    /// use std::ptr::NonNull;
    ///
    /// # unsafe {
    /// let x = [5_u8, 6, 7, 8, 9];
    /// let ptr = NonNull::new(x.as_ptr() as *mut u8).unwrap();
    /// let offset = ptr.align_offset(align_of::<u16>());
    ///
    /// if offset < x.len() - 1 {
    ///     let u16_ptr = ptr.add(offset).cast::<u16>();
    ///     assert!(u16_ptr.read() == u16::from_ne_bytes([5, 6]) || u16_ptr.read() == u16::from_ne_bytes([6, 7]));
    /// } else {
    ///     // while the pointer can be aligned via `offset`, it would point
    ///     // outside the allocation
    /// }
    /// # }
    /// ```
    #[unstable(feature = "non_null_convenience", issue = "117691")]
    #[rustc_const_unstable(feature = "non_null_convenience", issue = "117691")]
    //#[rustc_const_unstable(feature = "const_align_offset", issue = "90962")]
    #[must_use]
    #[inline]
    pub const fn align_offset(self, align: usize) -> usize
    where
        T: Sized,
    {
        if !align.is_power_of_two() {
            panic!("align_offset: align is not a power-of-two");
        }

        {
            // SAFETY: `align` has been checked to be a power of 2 above.
            unsafe { ptr::align_offset(self.pointer, align) }
        }
    }

    /// Returns whether the pointer is properly aligned for `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// use std::ptr::NonNull;
    ///
    /// // On some platforms, the alignment of i32 is less than 4.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    ///
    /// let data = AlignedI32(42);
    /// let ptr = NonNull::<AlignedI32>::from(&data);
    ///
    /// assert!(ptr.is_aligned());
    /// assert!(!NonNull::new(ptr.as_ptr().wrapping_byte_add(1)).unwrap().is_aligned());
    /// ```
    ///
    /// # At compiletime
    /// **Note: Alignment at compiletime is experimental and subject to change. See the
    /// [tracking issue] for details.**
    ///
    /// At compiletime, the compiler may not know where a value will end up in memory.
    /// Calling this function on a pointer created from a reference at compiletime will only
    /// return `true` if the pointer is guaranteed to be aligned. This means that the pointer
    /// is never aligned if cast to a type with a stricter alignment than the reference's
    /// underlying allocation.
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// #![feature(const_pointer_is_aligned)]
    /// #![feature(non_null_convenience)]
    /// #![feature(const_option)]
    /// #![feature(const_nonnull_new)]
    /// use std::ptr::NonNull;
    ///
    /// // On some platforms, the alignment of primitives is less than their size.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    /// #[repr(align(8))]
    /// struct AlignedI64(i64);
    ///
    /// const _: () = {
    ///     let data = [AlignedI32(42), AlignedI32(42)];
    ///     let ptr = NonNull::<AlignedI32>::new(&data[0] as *const _ as *mut _).unwrap();
    ///     assert!(ptr.is_aligned());
    ///
    ///     // At runtime either `ptr1` or `ptr2` would be aligned, but at compiletime neither is aligned.
    ///     let ptr1 = ptr.cast::<AlignedI64>();
    ///     let ptr2 = unsafe { ptr.add(1).cast::<AlignedI64>() };
    ///     assert!(!ptr1.is_aligned());
    ///     assert!(!ptr2.is_aligned());
    /// };
    /// ```
    ///
    /// Due to this behavior, it is possible that a runtime pointer derived from a compiletime
    /// pointer is aligned, even if the compiletime pointer wasn't aligned.
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// #![feature(const_pointer_is_aligned)]
    ///
    /// // On some platforms, the alignment of primitives is less than their size.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    /// #[repr(align(8))]
    /// struct AlignedI64(i64);
    ///
    /// // At compiletime, neither `COMPTIME_PTR` nor `COMPTIME_PTR + 1` is aligned.
    /// const COMPTIME_PTR: *const AlignedI32 = &AlignedI32(42);
    /// const _: () = assert!(!COMPTIME_PTR.cast::<AlignedI64>().is_aligned());
    /// const _: () = assert!(!COMPTIME_PTR.wrapping_add(1).cast::<AlignedI64>().is_aligned());
    ///
    /// // At runtime, either `runtime_ptr` or `runtime_ptr + 1` is aligned.
    /// let runtime_ptr = COMPTIME_PTR;
    /// assert_ne!(
    ///     runtime_ptr.cast::<AlignedI64>().is_aligned(),
    ///     runtime_ptr.wrapping_add(1).cast::<AlignedI64>().is_aligned(),
    /// );
    /// ```
    ///
    /// If a pointer is created from a fixed address, this function behaves the same during
    /// runtime and compiletime.
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// #![feature(const_pointer_is_aligned)]
    /// #![feature(const_option)]
    /// #![feature(const_nonnull_new)]
    /// use std::ptr::NonNull;
    ///
    /// // On some platforms, the alignment of primitives is less than their size.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    /// #[repr(align(8))]
    /// struct AlignedI64(i64);
    ///
    /// const _: () = {
    ///     let ptr = NonNull::new(40 as *mut AlignedI32).unwrap();
    ///     assert!(ptr.is_aligned());
    ///
    ///     // For pointers with a known address, runtime and compiletime behavior are identical.
    ///     let ptr1 = ptr.cast::<AlignedI64>();
    ///     let ptr2 = NonNull::new(ptr.as_ptr().wrapping_add(1)).unwrap().cast::<AlignedI64>();
    ///     assert!(ptr1.is_aligned());
    ///     assert!(!ptr2.is_aligned());
    /// };
    /// ```
    ///
    /// [tracking issue]: https://github.com/rust-lang/rust/issues/104203
    #[unstable(feature = "pointer_is_aligned", issue = "96284")]
    #[rustc_const_unstable(feature = "const_pointer_is_aligned", issue = "104203")]
    #[must_use]
    #[inline]
    pub const fn is_aligned(self) -> bool
    where
        T: Sized,
    {
        self.pointer.is_aligned()
    }

    /// Returns whether the pointer is aligned to `align`.
    ///
    /// For non-`Sized` pointees this operation considers only the data pointer,
    /// ignoring the metadata.
    ///
    /// # Panics
    ///
    /// The function panics if `align` is not a power-of-two (this includes 0).
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    ///
    /// // On some platforms, the alignment of i32 is less than 4.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    ///
    /// let data = AlignedI32(42);
    /// let ptr = &data as *const AlignedI32;
    ///
    /// assert!(ptr.is_aligned_to(1));
    /// assert!(ptr.is_aligned_to(2));
    /// assert!(ptr.is_aligned_to(4));
    ///
    /// assert!(ptr.wrapping_byte_add(2).is_aligned_to(2));
    /// assert!(!ptr.wrapping_byte_add(2).is_aligned_to(4));
    ///
    /// assert_ne!(ptr.is_aligned_to(8), ptr.wrapping_add(1).is_aligned_to(8));
    /// ```
    ///
    /// # At compiletime
    /// **Note: Alignment at compiletime is experimental and subject to change. See the
    /// [tracking issue] for details.**
    ///
    /// At compiletime, the compiler may not know where a value will end up in memory.
    /// Calling this function on a pointer created from a reference at compiletime will only
    /// return `true` if the pointer is guaranteed to be aligned. This means that the pointer
    /// cannot be stricter aligned than the reference's underlying allocation.
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// #![feature(const_pointer_is_aligned)]
    ///
    /// // On some platforms, the alignment of i32 is less than 4.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    ///
    /// const _: () = {
    ///     let data = AlignedI32(42);
    ///     let ptr = &data as *const AlignedI32;
    ///
    ///     assert!(ptr.is_aligned_to(1));
    ///     assert!(ptr.is_aligned_to(2));
    ///     assert!(ptr.is_aligned_to(4));
    ///
    ///     // At compiletime, we know for sure that the pointer isn't aligned to 8.
    ///     assert!(!ptr.is_aligned_to(8));
    ///     assert!(!ptr.wrapping_add(1).is_aligned_to(8));
    /// };
    /// ```
    ///
    /// Due to this behavior, it is possible that a runtime pointer derived from a compiletime
    /// pointer is aligned, even if the compiletime pointer wasn't aligned.
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// #![feature(const_pointer_is_aligned)]
    ///
    /// // On some platforms, the alignment of i32 is less than 4.
    /// #[repr(align(4))]
    /// struct AlignedI32(i32);
    ///
    /// // At compiletime, neither `COMPTIME_PTR` nor `COMPTIME_PTR + 1` is aligned.
    /// const COMPTIME_PTR: *const AlignedI32 = &AlignedI32(42);
    /// const _: () = assert!(!COMPTIME_PTR.is_aligned_to(8));
    /// const _: () = assert!(!COMPTIME_PTR.wrapping_add(1).is_aligned_to(8));
    ///
    /// // At runtime, either `runtime_ptr` or `runtime_ptr + 1` is aligned.
    /// let runtime_ptr = COMPTIME_PTR;
    /// assert_ne!(
    ///     runtime_ptr.is_aligned_to(8),
    ///     runtime_ptr.wrapping_add(1).is_aligned_to(8),
    /// );
    /// ```
    ///
    /// If a pointer is created from a fixed address, this function behaves the same during
    /// runtime and compiletime.
    ///
    /// ```
    /// #![feature(pointer_is_aligned)]
    /// #![feature(const_pointer_is_aligned)]
    ///
    /// const _: () = {
    ///     let ptr = 40 as *const u8;
    ///     assert!(ptr.is_aligned_to(1));
    ///     assert!(ptr.is_aligned_to(2));
    ///     assert!(ptr.is_aligned_to(4));
    ///     assert!(ptr.is_aligned_to(8));
    ///     assert!(!ptr.is_aligned_to(16));
    /// };
    /// ```
    ///
    /// [tracking issue]: https://github.com/rust-lang/rust/issues/104203
    #[unstable(feature = "pointer_is_aligned", issue = "96284")]
    #[rustc_const_unstable(feature = "const_pointer_is_aligned", issue = "104203")]
    #[must_use]
    #[inline]
    pub const fn is_aligned_to(self, align: usize) -> bool {
        self.pointer.is_aligned_to(align)
    }
}

impl<T> NonNull<[T]> {
    /// Creates a non-null raw slice from a thin pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, but dereferencing the return value is unsafe.
    /// See the documentation of [`slice::from_raw_parts`] for slice safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::ptr::NonNull;
    ///
    /// // create a slice pointer when starting out with a pointer to the first element
    /// let mut x = [5, 6, 7];
    /// let nonnull_pointer = NonNull::new(x.as_mut_ptr()).unwrap();
    /// let slice = NonNull::slice_from_raw_parts(nonnull_pointer, 3);
    /// assert_eq!(unsafe { slice.as_ref()[2] }, 7);
    /// ```
    ///
    /// (Note that this example artificially demonstrates a use of this method,
    /// but `let slice = NonNull::from(&x[..]);` would be a better way to write code like this.)
    #[stable(feature = "nonnull_slice_from_raw_parts", since = "1.70.0")]
    #[rustc_const_unstable(feature = "const_slice_from_raw_parts_mut", issue = "67456")]
    #[must_use]
    #[inline]
    pub const fn slice_from_raw_parts(data: NonNull<T>, len: usize) -> Self {
        // SAFETY: `data` is a `NonNull` pointer which is necessarily non-null
        unsafe { Self::new_unchecked(super::slice_from_raw_parts_mut(data.as_ptr(), len)) }
    }

    /// Returns the length of a non-null raw slice.
    ///
    /// The returned value is the number of **elements**, not the number of bytes.
    ///
    /// This function is safe, even when the non-null raw slice cannot be dereferenced to a slice
    /// because the pointer does not have a valid address.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::ptr::NonNull;
    ///
    /// let slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(), 3);
    /// assert_eq!(slice.len(), 3);
    /// ```
    #[stable(feature = "slice_ptr_len_nonnull", since = "1.63.0")]
    #[rustc_const_stable(feature = "const_slice_ptr_len_nonnull", since = "1.63.0")]
    #[rustc_allow_const_fn_unstable(const_slice_ptr_len)]
    #[must_use]
    #[inline]
    pub const fn len(self) -> usize {
        self.as_ptr().len()
    }

    /// Returns a non-null pointer to the slice's buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(slice_ptr_get)]
    /// use std::ptr::NonNull;
    ///
    /// let slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(), 3);
    /// assert_eq!(slice.as_non_null_ptr(), NonNull::<i8>::dangling());
    /// ```
    #[inline]
    #[must_use]
    #[unstable(feature = "slice_ptr_get", issue = "74265")]
    #[rustc_const_unstable(feature = "slice_ptr_get", issue = "74265")]
    pub const fn as_non_null_ptr(self) -> NonNull<T> {
        self.cast()
    }

    /// Returns a raw pointer to the slice's buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(slice_ptr_get)]
    /// use std::ptr::NonNull;
    ///
    /// let slice: NonNull<[i8]> = NonNull::slice_from_raw_parts(NonNull::dangling(), 3);
    /// assert_eq!(slice.as_mut_ptr(), NonNull::<i8>::dangling().as_ptr());
    /// ```
    #[inline]
    #[must_use]
    #[unstable(feature = "slice_ptr_get", issue = "74265")]
    #[rustc_const_unstable(feature = "slice_ptr_get", issue = "74265")]
    #[rustc_never_returns_null_ptr]
    pub const fn as_mut_ptr(self) -> *mut T {
        self.as_non_null_ptr().as_ptr()
    }

    /// Returns a shared reference to a slice of possibly uninitialized values. In contrast to
    /// [`as_ref`], this does not require that the value has to be initialized.
    ///
    /// For the mutable counterpart see [`as_uninit_slice_mut`].
    ///
    /// [`as_ref`]: NonNull::as_ref
    /// [`as_uninit_slice_mut`]: NonNull::as_uninit_slice_mut
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be [valid] for reads for `ptr.len() * mem::size_of::<T>()` many bytes,
    ///   and it must be properly aligned. This means in particular:
    ///
    ///     * The entire memory range of this slice must be contained within a single allocated object!
    ///       Slices can never span across multiple allocated objects.
    ///
    ///     * The pointer must be aligned even for zero-length slices. One
    ///       reason for this is that enum layout optimizations may rely on references
    ///       (including slices of any length) being aligned and non-null to distinguish
    ///       them from other data. You can obtain a pointer that is usable as `data`
    ///       for zero-length slices using [`NonNull::dangling()`].
    ///
    /// * The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get mutated (except inside `UnsafeCell`).
    ///
    /// This applies even if the result of this method is unused!
    ///
    /// See also [`slice::from_raw_parts`].
    ///
    /// [valid]: crate::ptr#safety
    #[inline]
    #[must_use]
    #[unstable(feature = "ptr_as_uninit", issue = "75402")]
    #[rustc_const_unstable(feature = "const_ptr_as_ref", issue = "91822")]
    pub const unsafe fn as_uninit_slice<'a>(self) -> &'a [MaybeUninit<T>] {
        // SAFETY: the caller must uphold the safety contract for `as_uninit_slice`.
        unsafe { slice::from_raw_parts(self.cast().as_ptr(), self.len()) }
    }

    /// Returns a unique reference to a slice of possibly uninitialized values. In contrast to
    /// [`as_mut`], this does not require that the value has to be initialized.
    ///
    /// For the shared counterpart see [`as_uninit_slice`].
    ///
    /// [`as_mut`]: NonNull::as_mut
    /// [`as_uninit_slice`]: NonNull::as_uninit_slice
    ///
    /// # Safety
    ///
    /// When calling this method, you have to ensure that all of the following is true:
    ///
    /// * The pointer must be [valid] for reads and writes for `ptr.len() * mem::size_of::<T>()`
    ///   many bytes, and it must be properly aligned. This means in particular:
    ///
    ///     * The entire memory range of this slice must be contained within a single allocated object!
    ///       Slices can never span across multiple allocated objects.
    ///
    ///     * The pointer must be aligned even for zero-length slices. One
    ///       reason for this is that enum layout optimizations may rely on references
    ///       (including slices of any length) being aligned and non-null to distinguish
    ///       them from other data. You can obtain a pointer that is usable as `data`
    ///       for zero-length slices using [`NonNull::dangling()`].
    ///
    /// * The total size `ptr.len() * mem::size_of::<T>()` of the slice must be no larger than `isize::MAX`.
    ///   See the safety documentation of [`pointer::offset`].
    ///
    /// * You must enforce Rust's aliasing rules, since the returned lifetime `'a` is
    ///   arbitrarily chosen and does not necessarily reflect the actual lifetime of the data.
    ///   In particular, while this reference exists, the memory the pointer points to must
    ///   not get accessed (read or written) through any other pointer.
    ///
    /// This applies even if the result of this method is unused!
    ///
    /// See also [`slice::from_raw_parts_mut`].
    ///
    /// [valid]: crate::ptr#safety
    ///
    /// # Examples
    ///
    /// ```rust
    /// #![feature(allocator_api, ptr_as_uninit)]
    ///
    /// use std::alloc::{Allocator, Layout, Global};
    /// use std::mem::MaybeUninit;
    /// use std::ptr::NonNull;
    ///
    /// let memory: NonNull<[u8]> = Global.allocate(Layout::new::<[u8; 32]>())?;
    /// // This is safe as `memory` is valid for reads and writes for `memory.len()` many bytes.
    /// // Note that calling `memory.as_mut()` is not allowed here as the content may be uninitialized.
    /// # #[allow(unused_variables)]
    /// let slice: &mut [MaybeUninit<u8>] = unsafe { memory.as_uninit_slice_mut() };
    /// # Ok::<_, std::alloc::AllocError>(())
    /// ```
    #[inline]
    #[must_use]
    #[unstable(feature = "ptr_as_uninit", issue = "75402")]
    #[rustc_const_unstable(feature = "const_ptr_as_ref", issue = "91822")]
    pub const unsafe fn as_uninit_slice_mut<'a>(self) -> &'a mut [MaybeUninit<T>] {
        // SAFETY: the caller must uphold the safety contract for `as_uninit_slice_mut`.
        unsafe { slice::from_raw_parts_mut(self.cast().as_ptr(), self.len()) }
    }

    /// Returns a raw pointer to an element or subslice, without doing bounds
    /// checking.
    ///
    /// Calling this method with an out-of-bounds index or when `self` is not dereferenceable
    /// is *[undefined behavior]* even if the resulting pointer is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(slice_ptr_get)]
    /// use std::ptr::NonNull;
    ///
    /// let x = &mut [1, 2, 4];
    /// let x = NonNull::slice_from_raw_parts(NonNull::new(x.as_mut_ptr()).unwrap(), x.len());
    ///
    /// unsafe {
    ///     assert_eq!(x.get_unchecked_mut(1).as_ptr(), x.as_non_null_ptr().as_ptr().add(1));
    /// }
    /// ```
    #[unstable(feature = "slice_ptr_get", issue = "74265")]
    #[inline]
    pub unsafe fn get_unchecked_mut<I>(self, index: I) -> NonNull<I::Output>
    where
        I: SliceIndex<[T]>,
    {
        // SAFETY: the caller ensures that `self` is dereferenceable and `index` in-bounds.
        // As a consequence, the resulting pointer cannot be null.
        unsafe { NonNull::new_unchecked(self.as_ptr().get_unchecked_mut(index)) }
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Clone for NonNull<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Copy for NonNull<T> {}

#[unstable(feature = "coerce_unsized", issue = "18598")]
impl<T: ?Sized, U: ?Sized> CoerceUnsized<NonNull<U>> for NonNull<T> where T: Unsize<U> {}

#[unstable(feature = "dispatch_from_dyn", issue = "none")]
impl<T: ?Sized, U: ?Sized> DispatchFromDyn<NonNull<U>> for NonNull<T> where T: Unsize<U> {}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> fmt::Debug for NonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> fmt::Pointer for NonNull<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.as_ptr(), f)
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Eq for NonNull<T> {}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> PartialEq for NonNull<T> {
    #[inline]
    #[allow(ambiguous_wide_pointer_comparisons)]
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> Ord for NonNull<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ptr().cmp(&other.as_ptr())
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> PartialOrd for NonNull<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ptr().partial_cmp(&other.as_ptr())
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> hash::Hash for NonNull<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state)
    }
}

#[unstable(feature = "ptr_internals", issue = "none")]
impl<T: ?Sized> From<Unique<T>> for NonNull<T> {
    #[inline]
    fn from(unique: Unique<T>) -> Self {
        unique.as_non_null_ptr()
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> From<&mut T> for NonNull<T> {
    /// Converts a `&mut T` to a `NonNull<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(reference: &mut T) -> Self {
        // SAFETY: A mutable reference cannot be null.
        unsafe { NonNull { pointer: reference as *mut T } }
    }
}

#[stable(feature = "nonnull", since = "1.25.0")]
impl<T: ?Sized> From<&T> for NonNull<T> {
    /// Converts a `&T` to a `NonNull<T>`.
    ///
    /// This conversion is safe and infallible since references cannot be null.
    #[inline]
    fn from(reference: &T) -> Self {
        // SAFETY: A reference cannot be null.
        unsafe { NonNull { pointer: reference as *const T } }
    }
}
