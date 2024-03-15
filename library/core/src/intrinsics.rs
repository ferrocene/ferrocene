//! Compiler intrinsics.
//!
//! The corresponding definitions are in <https://github.com/rust-lang/rust/blob/master/compiler/rustc_codegen_llvm/src/intrinsic.rs>.
//! The corresponding const implementations are in <https://github.com/rust-lang/rust/blob/master/compiler/rustc_const_eval/src/interpret/intrinsics.rs>.
//!
//! # Const intrinsics
//!
//! Note: any changes to the constness of intrinsics should be discussed with the language team.
//! This includes changes in the stability of the constness.
//!
//! In order to make an intrinsic usable at compile-time, one needs to copy the implementation
//! from <https://github.com/rust-lang/miri/blob/master/src/shims/intrinsics> to
//! <https://github.com/rust-lang/rust/blob/master/compiler/rustc_const_eval/src/interpret/intrinsics.rs> and add a
//! `#[rustc_const_unstable(feature = "const_such_and_such", issue = "01234")]` to the intrinsic declaration.
//!
//! If an intrinsic is supposed to be used from a `const fn` with a `rustc_const_stable` attribute,
//! the intrinsic's attribute must be `rustc_const_stable`, too. Such a change should not be done
//! without T-lang consultation, because it bakes a feature into the language that cannot be
//! replicated in user code without compiler support.
//!
//! # Volatiles
//!
//! The volatile intrinsics provide operations intended to act on I/O
//! memory, which are guaranteed to not be reordered by the compiler
//! across other volatile intrinsics. See the LLVM documentation on
//! [[volatile]].
//!
//! [volatile]: https://llvm.org/docs/LangRef.html#volatile-memory-accesses
//!
//! # Atomics
//!
//! The atomic intrinsics provide common atomic operations on machine
//! words, with multiple possible memory orderings. They obey the same
//! semantics as C++11. See the LLVM documentation on [[atomics]].
//!
//! [atomics]: https://llvm.org/docs/Atomics.html
//!
//! A quick refresher on memory ordering:
//!
//! * Acquire - a barrier for acquiring a lock. Subsequent reads and writes
//!   take place after the barrier.
//! * Release - a barrier for releasing a lock. Preceding reads and writes
//!   take place before the barrier.
//! * Sequentially consistent - sequentially consistent operations are
//!   guaranteed to happen in order. This is the standard mode for working
//!   with atomic types and is equivalent to Java's `volatile`.
//!
//! # Unwinding
//!
//! Rust intrinsics may, in general, unwind. If an intrinsic can never unwind, add the
//! `#[rustc_nounwind]` attribute so that the compiler can make use of this fact.
//!
//! However, even for intrinsics that may unwind, rustc assumes that a Rust intrinsics will never
//! initiate a foreign (non-Rust) unwind, and thus for panic=abort we can always assume that these
//! intrinsics cannot unwind.

#![unstable(
    feature = "core_intrinsics",
    reason = "intrinsics are unlikely to ever be stabilized, instead \
                      they should be used through stabilized interfaces \
                      in the rest of the standard library",
    issue = "none"
)]
#![allow(missing_docs)]

use crate::marker::DiscriminantKind;
use crate::marker::Tuple;
use crate::mem::align_of;

pub mod mir;
pub mod simd;

// These imports are used for simplifying intra-doc links
#[allow(unused_imports)]
#[cfg(all(target_has_atomic = "8", target_has_atomic = "32", target_has_atomic = "ptr"))]
use crate::sync::atomic::{self, AtomicBool, AtomicI32, AtomicIsize, AtomicU32, Ordering};

#[stable(feature = "drop_in_place", since = "1.8.0")]
#[rustc_allowed_through_unstable_modules]
#[deprecated(note = "no longer an intrinsic - use `ptr::drop_in_place` directly", since = "1.52.0")]
#[inline]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    // SAFETY: see `ptr::drop_in_place`
    unsafe { crate::ptr::drop_in_place(to_drop) }
}

#[cfg(bootstrap)]
pub use self::r#try as catch_unwind;

extern "rust-intrinsic" {
    // N.B., these intrinsics take raw pointers because they mutate aliased
    // memory, which is not valid for either `&` or `&mut`.

    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Relaxed`] as both the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_relaxed_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Relaxed`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_relaxed_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Relaxed`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_relaxed_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Acquire`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_acquire_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Acquire`] as both the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_acquire_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Acquire`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_acquire_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Release`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_release_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Release`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_release_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::Release`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_release_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::AcqRel`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_acqrel_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::AcqRel`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_acqrel_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::AcqRel`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_acqrel_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::SeqCst`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_seqcst_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::SeqCst`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_seqcst_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange` method by passing
    /// [`Ordering::SeqCst`] as both the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange`].
    #[rustc_nounwind]
    pub fn atomic_cxchg_seqcst_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);

    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Relaxed`] as both the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_relaxed_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Relaxed`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_relaxed_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Relaxed`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_relaxed_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Acquire`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_acquire_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Acquire`] as both the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_acquire_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Acquire`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_acquire_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Release`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_release_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Release`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_release_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::Release`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_release_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::AcqRel`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_acqrel_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::AcqRel`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_acqrel_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::AcqRel`] and [`Ordering::SeqCst`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_acqrel_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::SeqCst`] and [`Ordering::Relaxed`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_seqcst_relaxed<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::SeqCst`] and [`Ordering::Acquire`] as the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_seqcst_acquire<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);
    /// Stores a value if the current value is the same as the `old` value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `compare_exchange_weak` method by passing
    /// [`Ordering::SeqCst`] as both the success and failure parameters.
    /// For example, [`AtomicBool::compare_exchange_weak`].
    #[rustc_nounwind]
    pub fn atomic_cxchgweak_seqcst_seqcst<T: Copy>(dst: *mut T, old: T, src: T) -> (T, bool);

    /// Loads the current value of the pointer.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `load` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::load`].
    #[rustc_nounwind]
    pub fn atomic_load_seqcst<T: Copy>(src: *const T) -> T;
    /// Loads the current value of the pointer.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `load` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicBool::load`].
    #[rustc_nounwind]
    pub fn atomic_load_acquire<T: Copy>(src: *const T) -> T;
    /// Loads the current value of the pointer.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `load` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::load`].
    #[rustc_nounwind]
    pub fn atomic_load_relaxed<T: Copy>(src: *const T) -> T;
    /// Do NOT use this intrinsic; "unordered" operations do not exist in our memory model!
    /// In terms of the Rust Abstract Machine, this operation is equivalent to `src.read()`,
    /// i.e., it performs a non-atomic read.
    #[rustc_nounwind]
    pub fn atomic_load_unordered<T: Copy>(src: *const T) -> T;

    /// Stores the value at the specified memory location.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `store` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::store`].
    #[rustc_nounwind]
    pub fn atomic_store_seqcst<T: Copy>(dst: *mut T, val: T);
    /// Stores the value at the specified memory location.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `store` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicBool::store`].
    #[rustc_nounwind]
    pub fn atomic_store_release<T: Copy>(dst: *mut T, val: T);
    /// Stores the value at the specified memory location.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `store` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::store`].
    #[rustc_nounwind]
    pub fn atomic_store_relaxed<T: Copy>(dst: *mut T, val: T);
    /// Do NOT use this intrinsic; "unordered" operations do not exist in our memory model!
    /// In terms of the Rust Abstract Machine, this operation is equivalent to `dst.write(val)`,
    /// i.e., it performs a non-atomic write.
    #[rustc_nounwind]
    pub fn atomic_store_unordered<T: Copy>(dst: *mut T, val: T);

    /// Stores the value at the specified memory location, returning the old value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `swap` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::swap`].
    #[rustc_nounwind]
    pub fn atomic_xchg_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Stores the value at the specified memory location, returning the old value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `swap` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicBool::swap`].
    #[rustc_nounwind]
    pub fn atomic_xchg_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Stores the value at the specified memory location, returning the old value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `swap` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicBool::swap`].
    #[rustc_nounwind]
    pub fn atomic_xchg_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Stores the value at the specified memory location, returning the old value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `swap` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicBool::swap`].
    #[rustc_nounwind]
    pub fn atomic_xchg_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Stores the value at the specified memory location, returning the old value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `swap` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::swap`].
    #[rustc_nounwind]
    pub fn atomic_xchg_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Adds to the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_add` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicIsize::fetch_add`].
    #[rustc_nounwind]
    pub fn atomic_xadd_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Adds to the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_add` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicIsize::fetch_add`].
    #[rustc_nounwind]
    pub fn atomic_xadd_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Adds to the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_add` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicIsize::fetch_add`].
    #[rustc_nounwind]
    pub fn atomic_xadd_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Adds to the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_add` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicIsize::fetch_add`].
    #[rustc_nounwind]
    pub fn atomic_xadd_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Adds to the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_add` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicIsize::fetch_add`].
    #[rustc_nounwind]
    pub fn atomic_xadd_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Subtract from the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_sub` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicIsize::fetch_sub`].
    #[rustc_nounwind]
    pub fn atomic_xsub_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Subtract from the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_sub` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicIsize::fetch_sub`].
    #[rustc_nounwind]
    pub fn atomic_xsub_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Subtract from the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_sub` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicIsize::fetch_sub`].
    #[rustc_nounwind]
    pub fn atomic_xsub_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Subtract from the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_sub` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicIsize::fetch_sub`].
    #[rustc_nounwind]
    pub fn atomic_xsub_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Subtract from the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_sub` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicIsize::fetch_sub`].
    #[rustc_nounwind]
    pub fn atomic_xsub_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Bitwise and with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_and` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::fetch_and`].
    #[rustc_nounwind]
    pub fn atomic_and_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise and with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_and` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicBool::fetch_and`].
    #[rustc_nounwind]
    pub fn atomic_and_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise and with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_and` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicBool::fetch_and`].
    #[rustc_nounwind]
    pub fn atomic_and_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise and with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_and` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicBool::fetch_and`].
    #[rustc_nounwind]
    pub fn atomic_and_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise and with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_and` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::fetch_and`].
    #[rustc_nounwind]
    pub fn atomic_and_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Bitwise nand with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`AtomicBool`] type via the `fetch_nand` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::fetch_nand`].
    #[rustc_nounwind]
    pub fn atomic_nand_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise nand with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`AtomicBool`] type via the `fetch_nand` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicBool::fetch_nand`].
    #[rustc_nounwind]
    pub fn atomic_nand_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise nand with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`AtomicBool`] type via the `fetch_nand` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicBool::fetch_nand`].
    #[rustc_nounwind]
    pub fn atomic_nand_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise nand with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`AtomicBool`] type via the `fetch_nand` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicBool::fetch_nand`].
    #[rustc_nounwind]
    pub fn atomic_nand_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise nand with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`AtomicBool`] type via the `fetch_nand` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::fetch_nand`].
    #[rustc_nounwind]
    pub fn atomic_nand_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Bitwise or with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_or` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::fetch_or`].
    #[rustc_nounwind]
    pub fn atomic_or_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise or with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_or` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicBool::fetch_or`].
    #[rustc_nounwind]
    pub fn atomic_or_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise or with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_or` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicBool::fetch_or`].
    #[rustc_nounwind]
    pub fn atomic_or_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise or with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_or` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicBool::fetch_or`].
    #[rustc_nounwind]
    pub fn atomic_or_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise or with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_or` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::fetch_or`].
    #[rustc_nounwind]
    pub fn atomic_or_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Bitwise xor with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_xor` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicBool::fetch_xor`].
    #[rustc_nounwind]
    pub fn atomic_xor_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise xor with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_xor` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicBool::fetch_xor`].
    #[rustc_nounwind]
    pub fn atomic_xor_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise xor with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_xor` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicBool::fetch_xor`].
    #[rustc_nounwind]
    pub fn atomic_xor_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise xor with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_xor` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicBool::fetch_xor`].
    #[rustc_nounwind]
    pub fn atomic_xor_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Bitwise xor with the current value, returning the previous value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] types via the `fetch_xor` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicBool::fetch_xor`].
    #[rustc_nounwind]
    pub fn atomic_xor_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Maximum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_max` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicI32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_max_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_max` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicI32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_max_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_max` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicI32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_max_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_max` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicI32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_max_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_max` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicI32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_max_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Minimum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_min` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicI32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_min_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_min` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicI32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_min_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_min` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicI32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_min_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_min` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicI32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_min_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using a signed comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] signed integer types via the `fetch_min` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicI32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_min_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Minimum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_min` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicU32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_umin_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_min` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicU32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_umin_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_min` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicU32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_umin_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_min` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicU32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_umin_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Minimum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_min` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicU32::fetch_min`].
    #[rustc_nounwind]
    pub fn atomic_umin_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// Maximum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_max` method by passing
    /// [`Ordering::SeqCst`] as the `order`. For example, [`AtomicU32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_umax_seqcst<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_max` method by passing
    /// [`Ordering::Acquire`] as the `order`. For example, [`AtomicU32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_umax_acquire<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_max` method by passing
    /// [`Ordering::Release`] as the `order`. For example, [`AtomicU32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_umax_release<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_max` method by passing
    /// [`Ordering::AcqRel`] as the `order`. For example, [`AtomicU32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_umax_acqrel<T: Copy>(dst: *mut T, src: T) -> T;
    /// Maximum with the current value using an unsigned comparison.
    ///
    /// The stabilized version of this intrinsic is available on the
    /// [`atomic`] unsigned integer types via the `fetch_max` method by passing
    /// [`Ordering::Relaxed`] as the `order`. For example, [`AtomicU32::fetch_max`].
    #[rustc_nounwind]
    pub fn atomic_umax_relaxed<T: Copy>(dst: *mut T, src: T) -> T;

    /// An atomic fence.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::fence`] by passing [`Ordering::SeqCst`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_fence_seqcst();
    /// An atomic fence.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::fence`] by passing [`Ordering::Acquire`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_fence_acquire();
    /// An atomic fence.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::fence`] by passing [`Ordering::Release`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_fence_release();
    /// An atomic fence.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::fence`] by passing [`Ordering::AcqRel`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_fence_acqrel();

    /// A compiler-only memory barrier.
    ///
    /// Memory accesses will never be reordered across this barrier by the
    /// compiler, but no instructions will be emitted for it. This is
    /// appropriate for operations on the same thread that may be preempted,
    /// such as when interacting with signal handlers.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::compiler_fence`] by passing [`Ordering::SeqCst`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_singlethreadfence_seqcst();
    /// A compiler-only memory barrier.
    ///
    /// Memory accesses will never be reordered across this barrier by the
    /// compiler, but no instructions will be emitted for it. This is
    /// appropriate for operations on the same thread that may be preempted,
    /// such as when interacting with signal handlers.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::compiler_fence`] by passing [`Ordering::Acquire`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_singlethreadfence_acquire();
    /// A compiler-only memory barrier.
    ///
    /// Memory accesses will never be reordered across this barrier by the
    /// compiler, but no instructions will be emitted for it. This is
    /// appropriate for operations on the same thread that may be preempted,
    /// such as when interacting with signal handlers.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::compiler_fence`] by passing [`Ordering::Release`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_singlethreadfence_release();
    /// A compiler-only memory barrier.
    ///
    /// Memory accesses will never be reordered across this barrier by the
    /// compiler, but no instructions will be emitted for it. This is
    /// appropriate for operations on the same thread that may be preempted,
    /// such as when interacting with signal handlers.
    ///
    /// The stabilized version of this intrinsic is available in
    /// [`atomic::compiler_fence`] by passing [`Ordering::AcqRel`]
    /// as the `order`.
    #[rustc_nounwind]
    pub fn atomic_singlethreadfence_acqrel();

    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `locality` argument must be a constant integer and is a temporal locality specifier
    /// ranging from (0) - no locality, to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn prefetch_read_data<T>(data: *const T, locality: i32);
    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `locality` argument must be a constant integer and is a temporal locality specifier
    /// ranging from (0) - no locality, to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn prefetch_write_data<T>(data: *const T, locality: i32);
    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `locality` argument must be a constant integer and is a temporal locality specifier
    /// ranging from (0) - no locality, to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn prefetch_read_instruction<T>(data: *const T, locality: i32);
    /// The `prefetch` intrinsic is a hint to the code generator to insert a prefetch instruction
    /// if supported; otherwise, it is a no-op.
    /// Prefetches have no effect on the behavior of the program but can change its performance
    /// characteristics.
    ///
    /// The `locality` argument must be a constant integer and is a temporal locality specifier
    /// ranging from (0) - no locality, to (3) - extremely local keep in cache.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn prefetch_write_instruction<T>(data: *const T, locality: i32);

    /// Magic intrinsic that derives its meaning from attributes
    /// attached to the function.
    ///
    /// For example, dataflow uses this to inject static assertions so
    /// that `rustc_peek(potentially_uninitialized)` would actually
    /// double-check that dataflow did indeed compute that it is
    /// uninitialized at that point in the control flow.
    ///
    /// This intrinsic should not be used outside of the compiler.
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn rustc_peek<T>(_: T) -> T;

    /// Aborts the execution of the process.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// [`std::process::abort`](../../std/process/fn.abort.html) is to be preferred if possible,
    /// as its behavior is more user-friendly and more stable.
    ///
    /// The current implementation of `intrinsics::abort` is to invoke an invalid instruction,
    /// on most platforms.
    /// On Unix, the
    /// process will probably terminate with a signal like `SIGABRT`, `SIGILL`, `SIGTRAP`, `SIGSEGV` or
    /// `SIGBUS`.  The precise behaviour is not guaranteed and not stable.
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn abort() -> !;

    /// Informs the optimizer that this point in the code is not reachable,
    /// enabling further optimizations.
    ///
    /// N.B., this is very different from the `unreachable!()` macro: Unlike the
    /// macro, which panics when it is executed, it is *undefined behavior* to
    /// reach code marked with this function.
    ///
    /// The stabilized version of this intrinsic is [`core::hint::unreachable_unchecked`].
    #[rustc_const_stable(feature = "const_unreachable_unchecked", since = "1.57.0")]
    #[rustc_nounwind]
    pub fn unreachable() -> !;

}

/// Informs the optimizer that a condition is always true.
/// If the condition is false, the behavior is undefined.
///
/// No code is generated for this intrinsic, but the optimizer will try
/// to preserve it (and its condition) between passes, which may interfere
/// with optimization of surrounding code and reduce performance. It should
/// not be used if the invariant can be discovered by the optimizer on its
/// own, or if it does not enable any significant optimizations.
///
/// This intrinsic does not have a stable counterpart.
#[rustc_const_stable(feature = "const_assume", since = "1.77.0")]
#[rustc_nounwind]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[cfg_attr(bootstrap, inline)]
pub const unsafe fn assume(b: bool) {
    if !b {
        // SAFETY: the caller must guarantee the argument is never `false`
        unsafe { unreachable() }
    }
}

/// Hints to the compiler that branch condition is likely to be true.
/// Returns the value passed to it.
///
/// Any use other than with `if` statements will probably not have an effect.
///
/// Note that, unlike most intrinsics, this is safe to call;
/// it does not require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold
/// any safety invariants.
///
/// This intrinsic does not have a stable counterpart.
#[rustc_const_unstable(feature = "const_likely", issue = "none")]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[rustc_nounwind]
#[cfg_attr(bootstrap, inline)]
pub const fn likely(b: bool) -> bool {
    b
}

/// Hints to the compiler that branch condition is likely to be false.
/// Returns the value passed to it.
///
/// Any use other than with `if` statements will probably not have an effect.
///
/// Note that, unlike most intrinsics, this is safe to call;
/// it does not require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold
/// any safety invariants.
///
/// This intrinsic does not have a stable counterpart.
#[rustc_const_unstable(feature = "const_likely", issue = "none")]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[rustc_nounwind]
#[cfg_attr(bootstrap, inline)]
pub const fn unlikely(b: bool) -> bool {
    b
}

extern "rust-intrinsic" {
    /// Executes a breakpoint trap, for inspection by a debugger.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn breakpoint();

    /// The size of a type in bytes.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// More specifically, this is the offset in bytes between successive
    /// items of the same type, including alignment padding.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::size_of`].
    #[rustc_const_stable(feature = "const_size_of", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn size_of<T>() -> usize;

    /// The minimum alignment of a type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::align_of`].
    #[rustc_const_stable(feature = "const_min_align_of", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn min_align_of<T>() -> usize;
    /// The preferred alignment of a type.
    ///
    /// This intrinsic does not have a stable counterpart.
    /// It's "tracking issue" is [#91971](https://github.com/rust-lang/rust/issues/91971).
    #[rustc_const_unstable(feature = "const_pref_align_of", issue = "91971")]
    #[rustc_nounwind]
    pub fn pref_align_of<T>() -> usize;

    /// The size of the referenced value in bytes.
    ///
    /// The stabilized version of this intrinsic is [`crate::mem::size_of_val`].
    #[rustc_const_unstable(feature = "const_size_of_val", issue = "46571")]
    #[rustc_nounwind]
    pub fn size_of_val<T: ?Sized>(_: *const T) -> usize;
    /// The required alignment of the referenced value.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::align_of_val`].
    #[rustc_const_unstable(feature = "const_align_of_val", issue = "46571")]
    #[rustc_nounwind]
    pub fn min_align_of_val<T: ?Sized>(_: *const T) -> usize;

    /// Gets a static string slice containing the name of a type.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`core::any::type_name`].
    #[rustc_const_unstable(feature = "const_type_name", issue = "63084")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn type_name<T: ?Sized>() -> &'static str;

    /// Gets an identifier which is globally unique to the specified type. This
    /// function will return the same value for a type regardless of whichever
    /// crate it is invoked in.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`core::any::TypeId::of`].
    #[rustc_const_unstable(feature = "const_type_id", issue = "77125")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn type_id<T: ?Sized + 'static>() -> u128;

    /// A guard for unsafe functions that cannot ever be executed if `T` is uninhabited:
    /// This will statically either panic, or do nothing.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_stable(feature = "const_assert_type", since = "1.59.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn assert_inhabited<T>();

    /// A guard for unsafe functions that cannot ever be executed if `T` does not permit
    /// zero-initialization: This will statically either panic, or do nothing.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_stable(feature = "const_assert_type2", since = "1.75.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn assert_zero_valid<T>();

    /// A guard for `std::mem::uninitialized`. This will statically either panic, or do nothing.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_stable(feature = "const_assert_type2", since = "1.75.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn assert_mem_uninitialized_valid<T>();

    /// Gets a reference to a static `Location` indicating where it was called.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// Consider using [`core::panic::Location::caller`] instead.
    #[rustc_const_unstable(feature = "const_caller_location", issue = "76156")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn caller_location() -> &'static crate::panic::Location<'static>;

    /// Moves a value out of scope without running drop glue.
    ///
    /// This exists solely for [`crate::mem::forget_unsized`]; normal `forget` uses
    /// `ManuallyDrop` instead.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_const_unstable(feature = "const_intrinsic_forget", issue = "none")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn forget<T: ?Sized>(_: T);

    /// Reinterprets the bits of a value of one type as another type.
    ///
    /// Both types must have the same size. Compilation will fail if this is not guaranteed.
    ///
    /// `transmute` is semantically equivalent to a bitwise move of one type
    /// into another. It copies the bits from the source value into the
    /// destination value, then forgets the original. Note that source and destination
    /// are passed by-value, which means if `Src` or `Dst` contain padding, that padding
    /// is *not* guaranteed to be preserved by `transmute`.
    ///
    /// Both the argument and the result must be [valid](../../nomicon/what-unsafe-does.html) at
    /// their given type. Violating this condition leads to [undefined behavior][ub]. The compiler
    /// will generate code *assuming that you, the programmer, ensure that there will never be
    /// undefined behavior*. It is therefore your responsibility to guarantee that every value
    /// passed to `transmute` is valid at both types `Src` and `Dst`. Failing to uphold this condition
    /// may lead to unexpected and unstable compilation results. This makes `transmute` **incredibly
    /// unsafe**. `transmute` should be the absolute last resort.
    ///
    /// Transmuting pointers *to* integers in a `const` context is [undefined behavior][ub],
    /// unless the pointer was originally created *from* an integer.
    /// (That includes this function specifically, integer-to-pointer casts, and helpers like [`invalid`][crate::ptr::dangling],
    /// but also semantically-equivalent conversions such as punning through `repr(C)` union fields.)
    /// Any attempt to use the resulting value for integer operations will abort const-evaluation.
    /// (And even outside `const`, such transmutation is touching on many unspecified aspects of the
    /// Rust memory model and should be avoided. See below for alternatives.)
    ///
    /// Because `transmute` is a by-value operation, alignment of the *transmuted values
    /// themselves* is not a concern. As with any other function, the compiler already ensures
    /// both `Src` and `Dst` are properly aligned. However, when transmuting values that *point
    /// elsewhere* (such as pointers, references, boxes…), the caller has to ensure proper
    /// alignment of the pointed-to values.
    ///
    /// The [nomicon](../../nomicon/transmutes.html) has additional documentation.
    ///
    /// [ub]: ../../reference/behavior-considered-undefined.html
    ///
    /// # Examples
    ///
    /// There are a few things that `transmute` is really useful for.
    ///
    /// Turning a pointer into a function pointer. This is *not* portable to
    /// machines where function pointers and data pointers have different sizes.
    ///
    /// ```
    /// fn foo() -> i32 {
    ///     0
    /// }
    /// // Crucially, we `as`-cast to a raw pointer before `transmute`ing to a function pointer.
    /// // This avoids an integer-to-pointer `transmute`, which can be problematic.
    /// // Transmuting between raw pointers and function pointers (i.e., two pointer types) is fine.
    /// let pointer = foo as *const ();
    /// let function = unsafe {
    ///     std::mem::transmute::<*const (), fn() -> i32>(pointer)
    /// };
    /// assert_eq!(function(), 0);
    /// ```
    ///
    /// Extending a lifetime, or shortening an invariant lifetime. This is
    /// advanced, very unsafe Rust!
    ///
    /// ```
    /// struct R<'a>(&'a i32);
    /// unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    ///     std::mem::transmute::<R<'b>, R<'static>>(r)
    /// }
    ///
    /// unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>)
    ///                                              -> &'b mut R<'c> {
    ///     std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
    /// }
    /// ```
    ///
    /// # Alternatives
    ///
    /// Don't despair: many uses of `transmute` can be achieved through other means.
    /// Below are common applications of `transmute` which can be replaced with safer
    /// constructs.
    ///
    /// Turning raw bytes (`[u8; SZ]`) into `u32`, `f64`, etc.:
    ///
    /// ```
    /// let raw_bytes = [0x78, 0x56, 0x34, 0x12];
    ///
    /// let num = unsafe {
    ///     std::mem::transmute::<[u8; 4], u32>(raw_bytes)
    /// };
    ///
    /// // use `u32::from_ne_bytes` instead
    /// let num = u32::from_ne_bytes(raw_bytes);
    /// // or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
    /// let num = u32::from_le_bytes(raw_bytes);
    /// assert_eq!(num, 0x12345678);
    /// let num = u32::from_be_bytes(raw_bytes);
    /// assert_eq!(num, 0x78563412);
    /// ```
    ///
    /// Turning a pointer into a `usize`:
    ///
    /// ```no_run
    /// let ptr = &0;
    /// let ptr_num_transmute = unsafe {
    ///     std::mem::transmute::<&i32, usize>(ptr)
    /// };
    ///
    /// // Use an `as` cast instead
    /// let ptr_num_cast = ptr as *const i32 as usize;
    /// ```
    ///
    /// Note that using `transmute` to turn a pointer to a `usize` is (as noted above) [undefined
    /// behavior][ub] in `const` contexts. Also outside of consts, this operation might not behave
    /// as expected -- this is touching on many unspecified aspects of the Rust memory model.
    /// Depending on what the code is doing, the following alternatives are preferable to
    /// pointer-to-integer transmutation:
    /// - If the code just wants to store data of arbitrary type in some buffer and needs to pick a
    ///   type for that buffer, it can use [`MaybeUninit`][crate::mem::MaybeUninit].
    /// - If the code actually wants to work on the address the pointer points to, it can use `as`
    ///   casts or [`ptr.addr()`][pointer::addr].
    ///
    /// Turning a `*mut T` into an `&mut T`:
    ///
    /// ```
    /// let ptr: *mut i32 = &mut 0;
    /// let ref_transmuted = unsafe {
    ///     std::mem::transmute::<*mut i32, &mut i32>(ptr)
    /// };
    ///
    /// // Use a reborrow instead
    /// let ref_casted = unsafe { &mut *ptr };
    /// ```
    ///
    /// Turning an `&mut T` into an `&mut U`:
    ///
    /// ```
    /// let ptr = &mut 0;
    /// let val_transmuted = unsafe {
    ///     std::mem::transmute::<&mut i32, &mut u32>(ptr)
    /// };
    ///
    /// // Now, put together `as` and reborrowing - note the chaining of `as`
    /// // `as` is not transitive
    /// let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };
    /// ```
    ///
    /// Turning an `&str` into a `&[u8]`:
    ///
    /// ```
    /// // this is not a good way to do this.
    /// let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
    /// assert_eq!(slice, &[82, 117, 115, 116]);
    ///
    /// // You could use `str::as_bytes`
    /// let slice = "Rust".as_bytes();
    /// assert_eq!(slice, &[82, 117, 115, 116]);
    ///
    /// // Or, just use a byte string, if you have control over the string
    /// // literal
    /// assert_eq!(b"Rust", &[82, 117, 115, 116]);
    /// ```
    ///
    /// Turning a `Vec<&T>` into a `Vec<Option<&T>>`.
    ///
    /// To transmute the inner type of the contents of a container, you must make sure to not
    /// violate any of the container's invariants. For `Vec`, this means that both the size
    /// *and alignment* of the inner types have to match. Other containers might rely on the
    /// size of the type, alignment, or even the `TypeId`, in which case transmuting wouldn't
    /// be possible at all without violating the container invariants.
    ///
    /// ```
    /// let store = [0, 1, 2, 3];
    /// let v_orig = store.iter().collect::<Vec<&i32>>();
    ///
    /// // clone the vector as we will reuse them later
    /// let v_clone = v_orig.clone();
    ///
    /// // Using transmute: this relies on the unspecified data layout of `Vec`, which is a
    /// // bad idea and could cause Undefined Behavior.
    /// // However, it is no-copy.
    /// let v_transmuted = unsafe {
    ///     std::mem::transmute::<Vec<&i32>, Vec<Option<&i32>>>(v_clone)
    /// };
    ///
    /// let v_clone = v_orig.clone();
    ///
    /// // This is the suggested, safe way.
    /// // It does copy the entire vector, though, into a new array.
    /// let v_collected = v_clone.into_iter()
    ///                          .map(Some)
    ///                          .collect::<Vec<Option<&i32>>>();
    ///
    /// let v_clone = v_orig.clone();
    ///
    /// // This is the proper no-copy, unsafe way of "transmuting" a `Vec`, without relying on the
    /// // data layout. Instead of literally calling `transmute`, we perform a pointer cast, but
    /// // in terms of converting the original inner type (`&i32`) to the new one (`Option<&i32>`),
    /// // this has all the same caveats. Besides the information provided above, also consult the
    /// // [`from_raw_parts`] documentation.
    /// let v_from_raw = unsafe {
    // FIXME Update this when vec_into_raw_parts is stabilized
    ///     // Ensure the original vector is not dropped.
    ///     let mut v_clone = std::mem::ManuallyDrop::new(v_clone);
    ///     Vec::from_raw_parts(v_clone.as_mut_ptr() as *mut Option<&i32>,
    ///                         v_clone.len(),
    ///                         v_clone.capacity())
    /// };
    /// ```
    ///
    /// [`from_raw_parts`]: ../../std/vec/struct.Vec.html#method.from_raw_parts
    ///
    /// Implementing `split_at_mut`:
    ///
    /// ```
    /// use std::{slice, mem};
    ///
    /// // There are multiple ways to do this, and there are multiple problems
    /// // with the following (transmute) way.
    /// fn split_at_mut_transmute<T>(slice: &mut [T], mid: usize)
    ///                              -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let slice2 = mem::transmute::<&mut [T], &mut [T]>(slice);
    ///         // first: transmute is not type safe; all it checks is that T and
    ///         // U are of the same size. Second, right here, you have two
    ///         // mutable references pointing to the same memory.
    ///         (&mut slice[0..mid], &mut slice2[mid..len])
    ///     }
    /// }
    ///
    /// // This gets rid of the type safety problems; `&mut *` will *only* give
    /// // you an `&mut T` from an `&mut T` or `*mut T`.
    /// fn split_at_mut_casts<T>(slice: &mut [T], mid: usize)
    ///                          -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let slice2 = &mut *(slice as *mut [T]);
    ///         // however, you still have two mutable references pointing to
    ///         // the same memory.
    ///         (&mut slice[0..mid], &mut slice2[mid..len])
    ///     }
    /// }
    ///
    /// // This is how the standard library does it. This is the best method, if
    /// // you need to do something like this
    /// fn split_at_stdlib<T>(slice: &mut [T], mid: usize)
    ///                       -> (&mut [T], &mut [T]) {
    ///     let len = slice.len();
    ///     assert!(mid <= len);
    ///     unsafe {
    ///         let ptr = slice.as_mut_ptr();
    ///         // This now has three mutable references pointing at the same
    ///         // memory. `slice`, the rvalue ret.0, and the rvalue ret.1.
    ///         // `slice` is never used after `let ptr = ...`, and so one can
    ///         // treat it as "dead", and therefore, you only have two real
    ///         // mutable slices.
    ///         (slice::from_raw_parts_mut(ptr, mid),
    ///          slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    ///     }
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_allowed_through_unstable_modules]
    #[rustc_const_stable(feature = "const_transmute", since = "1.56.0")]
    #[rustc_diagnostic_item = "transmute"]
    #[rustc_nounwind]
    pub fn transmute<Src, Dst>(src: Src) -> Dst;

    /// Like [`transmute`], but even less checked at compile-time: rather than
    /// giving an error for `size_of::<Src>() != size_of::<Dst>()`, it's
    /// **Undefined Behaviour** at runtime.
    ///
    /// Prefer normal `transmute` where possible, for the extra checking, since
    /// both do exactly the same thing at runtime, if they both compile.
    ///
    /// This is not expected to ever be exposed directly to users, rather it
    /// may eventually be exposed through some more-constrained API.
    #[rustc_const_stable(feature = "const_transmute", since = "1.56.0")]
    #[rustc_nounwind]
    pub fn transmute_unchecked<Src, Dst>(src: Src) -> Dst;

    /// Returns `true` if the actual type given as `T` requires drop
    /// glue; returns `false` if the actual type provided for `T`
    /// implements `Copy`.
    ///
    /// If the actual type neither requires drop glue nor implements
    /// `Copy`, then the return value of this function is unspecified.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`mem::needs_drop`](crate::mem::needs_drop).
    #[rustc_const_stable(feature = "const_needs_drop", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn needs_drop<T: ?Sized>() -> bool;

    /// Calculates the offset from a pointer.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion would throw away aliasing information.
    ///
    /// This can only be used with `Ptr` as a raw pointer type (`*mut` or `*const`)
    /// to a `Sized` pointee and with `Delta` as `usize` or `isize`.  Any other
    /// instantiations may arbitrarily misbehave, and that's *not* a compiler bug.
    ///
    /// # Safety
    ///
    /// Both the starting and resulting pointer must be either in bounds or one
    /// byte past the end of an allocated object. If either pointer is out of
    /// bounds or arithmetic overflow occurs then any further use of the
    /// returned value will result in undefined behavior.
    ///
    /// The stabilized version of this intrinsic is [`pointer::offset`].
    #[must_use = "returns a new pointer rather than modifying its argument"]
    #[rustc_const_stable(feature = "const_ptr_offset", since = "1.61.0")]
    #[rustc_nounwind]
    pub fn offset<Ptr, Delta>(dst: Ptr, offset: Delta) -> Ptr;

    /// Calculates the offset from a pointer, potentially wrapping.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion inhibits certain optimizations.
    ///
    /// # Safety
    ///
    /// Unlike the `offset` intrinsic, this intrinsic does not restrict the
    /// resulting pointer to point into or one byte past the end of an allocated
    /// object, and it wraps with two's complement arithmetic. The resulting
    /// value is not necessarily valid to be used to actually access memory.
    ///
    /// The stabilized version of this intrinsic is [`pointer::wrapping_offset`].
    #[must_use = "returns a new pointer rather than modifying its argument"]
    #[rustc_const_stable(feature = "const_ptr_offset", since = "1.61.0")]
    #[rustc_nounwind]
    pub fn arith_offset<T>(dst: *const T, offset: isize) -> *const T;

    /// Masks out bits of the pointer according to a mask.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// Consider using [`pointer::mask`] instead.
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn ptr_mask<T>(ptr: *const T, mask: usize) -> *const T;

    /// Equivalent to the appropriate `llvm.memcpy.p0i8.0i8.*` intrinsic, with
    /// a size of `count` * `size_of::<T>()` and an alignment of
    /// `min_align_of::<T>()`
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out
    /// unless size is equal to zero.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn volatile_copy_nonoverlapping_memory<T>(dst: *mut T, src: *const T, count: usize);
    /// Equivalent to the appropriate `llvm.memmove.p0i8.0i8.*` intrinsic, with
    /// a size of `count * size_of::<T>()` and an alignment of
    /// `min_align_of::<T>()`
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out
    /// unless size is equal to zero.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn volatile_copy_memory<T>(dst: *mut T, src: *const T, count: usize);
    /// Equivalent to the appropriate `llvm.memset.p0i8.*` intrinsic, with a
    /// size of `count * size_of::<T>()` and an alignment of
    /// `min_align_of::<T>()`.
    ///
    /// The volatile parameter is set to `true`, so it will not be optimized out
    /// unless size is equal to zero.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn volatile_set_memory<T>(dst: *mut T, val: u8, count: usize);

    /// Performs a volatile load from the `src` pointer.
    ///
    /// The stabilized version of this intrinsic is [`core::ptr::read_volatile`].
    #[rustc_nounwind]
    pub fn volatile_load<T>(src: *const T) -> T;
    /// Performs a volatile store to the `dst` pointer.
    ///
    /// The stabilized version of this intrinsic is [`core::ptr::write_volatile`].
    #[rustc_nounwind]
    pub fn volatile_store<T>(dst: *mut T, val: T);

    /// Performs a volatile load from the `src` pointer
    /// The pointer is not required to be aligned.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_diagnostic_item = "intrinsics_unaligned_volatile_load"]
    pub fn unaligned_volatile_load<T>(src: *const T) -> T;
    /// Performs a volatile store to the `dst` pointer.
    /// The pointer is not required to be aligned.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_diagnostic_item = "intrinsics_unaligned_volatile_store"]
    pub fn unaligned_volatile_store<T>(dst: *mut T, val: T);

    /// Returns the square root of an `f32`
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::sqrt`](../../std/primitive.f32.html#method.sqrt)
    #[rustc_nounwind]
    pub fn sqrtf32(x: f32) -> f32;
    /// Returns the square root of an `f64`
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::sqrt`](../../std/primitive.f64.html#method.sqrt)
    #[rustc_nounwind]
    pub fn sqrtf64(x: f64) -> f64;

    /// Raises an `f32` to an integer power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::powi`](../../std/primitive.f32.html#method.powi)
    #[rustc_nounwind]
    pub fn powif32(a: f32, x: i32) -> f32;
    /// Raises an `f64` to an integer power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::powi`](../../std/primitive.f64.html#method.powi)
    #[rustc_nounwind]
    pub fn powif64(a: f64, x: i32) -> f64;

    /// Returns the sine of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::sin`](../../std/primitive.f32.html#method.sin)
    #[rustc_nounwind]
    pub fn sinf32(x: f32) -> f32;
    /// Returns the sine of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::sin`](../../std/primitive.f64.html#method.sin)
    #[rustc_nounwind]
    pub fn sinf64(x: f64) -> f64;

    /// Returns the cosine of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::cos`](../../std/primitive.f32.html#method.cos)
    #[rustc_nounwind]
    pub fn cosf32(x: f32) -> f32;
    /// Returns the cosine of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::cos`](../../std/primitive.f64.html#method.cos)
    #[rustc_nounwind]
    pub fn cosf64(x: f64) -> f64;

    /// Raises an `f32` to an `f32` power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::powf`](../../std/primitive.f32.html#method.powf)
    #[rustc_nounwind]
    pub fn powf32(a: f32, x: f32) -> f32;
    /// Raises an `f64` to an `f64` power.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::powf`](../../std/primitive.f64.html#method.powf)
    #[rustc_nounwind]
    pub fn powf64(a: f64, x: f64) -> f64;

    /// Returns the exponential of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::exp`](../../std/primitive.f32.html#method.exp)
    #[rustc_nounwind]
    pub fn expf32(x: f32) -> f32;
    /// Returns the exponential of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::exp`](../../std/primitive.f64.html#method.exp)
    #[rustc_nounwind]
    pub fn expf64(x: f64) -> f64;

    /// Returns 2 raised to the power of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::exp2`](../../std/primitive.f32.html#method.exp2)
    #[rustc_nounwind]
    pub fn exp2f32(x: f32) -> f32;
    /// Returns 2 raised to the power of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::exp2`](../../std/primitive.f64.html#method.exp2)
    #[rustc_nounwind]
    pub fn exp2f64(x: f64) -> f64;

    /// Returns the natural logarithm of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::ln`](../../std/primitive.f32.html#method.ln)
    #[rustc_nounwind]
    pub fn logf32(x: f32) -> f32;
    /// Returns the natural logarithm of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::ln`](../../std/primitive.f64.html#method.ln)
    #[rustc_nounwind]
    pub fn logf64(x: f64) -> f64;

    /// Returns the base 10 logarithm of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::log10`](../../std/primitive.f32.html#method.log10)
    #[rustc_nounwind]
    pub fn log10f32(x: f32) -> f32;
    /// Returns the base 10 logarithm of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::log10`](../../std/primitive.f64.html#method.log10)
    #[rustc_nounwind]
    pub fn log10f64(x: f64) -> f64;

    /// Returns the base 2 logarithm of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::log2`](../../std/primitive.f32.html#method.log2)
    #[rustc_nounwind]
    pub fn log2f32(x: f32) -> f32;
    /// Returns the base 2 logarithm of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::log2`](../../std/primitive.f64.html#method.log2)
    #[rustc_nounwind]
    pub fn log2f64(x: f64) -> f64;

    /// Returns `a * b + c` for `f32` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::mul_add`](../../std/primitive.f32.html#method.mul_add)
    #[rustc_nounwind]
    pub fn fmaf32(a: f32, b: f32, c: f32) -> f32;
    /// Returns `a * b + c` for `f64` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::mul_add`](../../std/primitive.f64.html#method.mul_add)
    #[rustc_nounwind]
    pub fn fmaf64(a: f64, b: f64, c: f64) -> f64;

    /// Returns the absolute value of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::abs`](../../std/primitive.f32.html#method.abs)
    #[rustc_nounwind]
    pub fn fabsf32(x: f32) -> f32;
    /// Returns the absolute value of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::abs`](../../std/primitive.f64.html#method.abs)
    #[rustc_nounwind]
    pub fn fabsf64(x: f64) -> f64;

    /// Returns the minimum of two `f32` values.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::min`]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn minnumf32(x: f32, y: f32) -> f32;
    /// Returns the minimum of two `f64` values.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::min`]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn minnumf64(x: f64, y: f64) -> f64;
    /// Returns the maximum of two `f32` values.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::max`]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn maxnumf32(x: f32, y: f32) -> f32;
    /// Returns the maximum of two `f64` values.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::max`]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn maxnumf64(x: f64, y: f64) -> f64;

    /// Copies the sign from `y` to `x` for `f32` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::copysign`](../../std/primitive.f32.html#method.copysign)
    #[rustc_nounwind]
    pub fn copysignf32(x: f32, y: f32) -> f32;
    /// Copies the sign from `y` to `x` for `f64` values.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::copysign`](../../std/primitive.f64.html#method.copysign)
    #[rustc_nounwind]
    pub fn copysignf64(x: f64, y: f64) -> f64;

    /// Returns the largest integer less than or equal to an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::floor`](../../std/primitive.f32.html#method.floor)
    #[rustc_nounwind]
    pub fn floorf32(x: f32) -> f32;
    /// Returns the largest integer less than or equal to an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::floor`](../../std/primitive.f64.html#method.floor)
    #[rustc_nounwind]
    pub fn floorf64(x: f64) -> f64;

    /// Returns the smallest integer greater than or equal to an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::ceil`](../../std/primitive.f32.html#method.ceil)
    #[rustc_nounwind]
    pub fn ceilf32(x: f32) -> f32;
    /// Returns the smallest integer greater than or equal to an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::ceil`](../../std/primitive.f64.html#method.ceil)
    #[rustc_nounwind]
    pub fn ceilf64(x: f64) -> f64;

    /// Returns the integer part of an `f32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::trunc`](../../std/primitive.f32.html#method.trunc)
    #[rustc_nounwind]
    pub fn truncf32(x: f32) -> f32;
    /// Returns the integer part of an `f64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::trunc`](../../std/primitive.f64.html#method.trunc)
    #[rustc_nounwind]
    pub fn truncf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`. Changing the rounding mode is not possible in Rust,
    /// so this rounds half-way cases to the number with an even least significant digit.
    ///
    /// May raise an inexact floating-point exception if the argument is not an integer.
    /// However, Rust assumes floating-point exceptions cannot be observed, so these exceptions
    /// cannot actually be utilized from Rust code.
    /// In other words, this intrinsic is equivalent in behavior to `nearbyintf32` and `roundevenf32`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::round_ties_even`](../../std/primitive.f32.html#method.round_ties_even)
    #[rustc_nounwind]
    pub fn rintf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. Changing the rounding mode is not possible in Rust,
    /// so this rounds half-way cases to the number with an even least significant digit.
    ///
    /// May raise an inexact floating-point exception if the argument is not an integer.
    /// However, Rust assumes floating-point exceptions cannot be observed, so these exceptions
    /// cannot actually be utilized from Rust code.
    /// In other words, this intrinsic is equivalent in behavior to `nearbyintf64` and `roundevenf64`.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::round_ties_even`](../../std/primitive.f64.html#method.round_ties_even)
    #[rustc_nounwind]
    pub fn rintf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`. Changing the rounding mode is not possible in Rust,
    /// so this rounds half-way cases to the number with an even least significant digit.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn nearbyintf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. Changing the rounding mode is not possible in Rust,
    /// so this rounds half-way cases to the number with an even least significant digit.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn nearbyintf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`. Rounds half-way cases away from zero.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f32::round`](../../std/primitive.f32.html#method.round)
    #[rustc_nounwind]
    pub fn roundf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. Rounds half-way cases away from zero.
    ///
    /// The stabilized version of this intrinsic is
    /// [`f64::round`](../../std/primitive.f64.html#method.round)
    #[rustc_nounwind]
    pub fn roundf64(x: f64) -> f64;

    /// Returns the nearest integer to an `f32`. Rounds half-way cases to the number
    /// with an even least significant digit.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn roundevenf32(x: f32) -> f32;
    /// Returns the nearest integer to an `f64`. Rounds half-way cases to the number
    /// with an even least significant digit.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn roundevenf64(x: f64) -> f64;

    /// Float addition that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn fadd_fast<T: Copy>(a: T, b: T) -> T;

    /// Float subtraction that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn fsub_fast<T: Copy>(a: T, b: T) -> T;

    /// Float multiplication that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn fmul_fast<T: Copy>(a: T, b: T) -> T;

    /// Float division that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn fdiv_fast<T: Copy>(a: T, b: T) -> T;

    /// Float remainder that allows optimizations based on algebraic rules.
    /// May assume inputs are finite.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    pub fn frem_fast<T: Copy>(a: T, b: T) -> T;

    /// Float addition that allows optimizations based on algebraic rules.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_safe_intrinsic]
    #[cfg(not(bootstrap))]
    pub fn fadd_algebraic<T: Copy>(a: T, b: T) -> T;

    /// Float subtraction that allows optimizations based on algebraic rules.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_safe_intrinsic]
    #[cfg(not(bootstrap))]
    pub fn fsub_algebraic<T: Copy>(a: T, b: T) -> T;

    /// Float multiplication that allows optimizations based on algebraic rules.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_safe_intrinsic]
    #[cfg(not(bootstrap))]
    pub fn fmul_algebraic<T: Copy>(a: T, b: T) -> T;

    /// Float division that allows optimizations based on algebraic rules.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_safe_intrinsic]
    #[cfg(not(bootstrap))]
    pub fn fdiv_algebraic<T: Copy>(a: T, b: T) -> T;

    /// Float remainder that allows optimizations based on algebraic rules.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_nounwind]
    #[rustc_safe_intrinsic]
    #[cfg(not(bootstrap))]
    pub fn frem_algebraic<T: Copy>(a: T, b: T) -> T;

    /// Convert with LLVM’s fptoui/fptosi, which may return undef for values out of range
    /// (<https://github.com/rust-lang/rust/issues/10184>)
    ///
    /// Stabilized as [`f32::to_int_unchecked`] and [`f64::to_int_unchecked`].
    #[rustc_nounwind]
    pub fn float_to_int_unchecked<Float: Copy, Int: Copy>(value: Float) -> Int;

    /// Returns the number of bits set in an integer type `T`
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `count_ones` method. For example,
    /// [`u32::count_ones`]
    #[rustc_const_stable(feature = "const_ctpop", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn ctpop<T: Copy>(x: T) -> T;

    /// Returns the number of leading unset bits (zeroes) in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `leading_zeros` method. For example,
    /// [`u32::leading_zeros`]
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::ctlz;
    ///
    /// let x = 0b0001_1100_u8;
    /// let num_leading = ctlz(x);
    /// assert_eq!(num_leading, 3);
    /// ```
    ///
    /// An `x` with value `0` will return the bit width of `T`.
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::ctlz;
    ///
    /// let x = 0u16;
    /// let num_leading = ctlz(x);
    /// assert_eq!(num_leading, 16);
    /// ```
    #[rustc_const_stable(feature = "const_ctlz", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn ctlz<T: Copy>(x: T) -> T;

    /// Like `ctlz`, but extra-unsafe as it returns `undef` when
    /// given an `x` with value `0`.
    ///
    /// This intrinsic does not have a stable counterpart.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::ctlz_nonzero;
    ///
    /// let x = 0b0001_1100_u8;
    /// let num_leading = unsafe { ctlz_nonzero(x) };
    /// assert_eq!(num_leading, 3);
    /// ```
    #[rustc_const_stable(feature = "constctlz", since = "1.50.0")]
    #[rustc_nounwind]
    pub fn ctlz_nonzero<T: Copy>(x: T) -> T;

    /// Returns the number of trailing unset bits (zeroes) in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `trailing_zeros` method. For example,
    /// [`u32::trailing_zeros`]
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::cttz;
    ///
    /// let x = 0b0011_1000_u8;
    /// let num_trailing = cttz(x);
    /// assert_eq!(num_trailing, 3);
    /// ```
    ///
    /// An `x` with value `0` will return the bit width of `T`:
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::cttz;
    ///
    /// let x = 0u16;
    /// let num_trailing = cttz(x);
    /// assert_eq!(num_trailing, 16);
    /// ```
    #[rustc_const_stable(feature = "const_cttz", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn cttz<T: Copy>(x: T) -> T;

    /// Like `cttz`, but extra-unsafe as it returns `undef` when
    /// given an `x` with value `0`.
    ///
    /// This intrinsic does not have a stable counterpart.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    ///
    /// use std::intrinsics::cttz_nonzero;
    ///
    /// let x = 0b0011_1000_u8;
    /// let num_trailing = unsafe { cttz_nonzero(x) };
    /// assert_eq!(num_trailing, 3);
    /// ```
    #[rustc_const_stable(feature = "const_cttz_nonzero", since = "1.53.0")]
    #[rustc_nounwind]
    pub fn cttz_nonzero<T: Copy>(x: T) -> T;

    /// Reverses the bytes in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `swap_bytes` method. For example,
    /// [`u32::swap_bytes`]
    #[rustc_const_stable(feature = "const_bswap", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn bswap<T: Copy>(x: T) -> T;

    /// Reverses the bits in an integer type `T`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `reverse_bits` method. For example,
    /// [`u32::reverse_bits`]
    #[rustc_const_stable(feature = "const_bitreverse", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn bitreverse<T: Copy>(x: T) -> T;

    /// Performs checked integer addition.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `overflowing_add` method. For example,
    /// [`u32::overflowing_add`]
    #[rustc_const_stable(feature = "const_int_overflow", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn add_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

    /// Performs checked integer subtraction
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `overflowing_sub` method. For example,
    /// [`u32::overflowing_sub`]
    #[rustc_const_stable(feature = "const_int_overflow", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn sub_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

    /// Performs checked integer multiplication
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `overflowing_mul` method. For example,
    /// [`u32::overflowing_mul`]
    #[rustc_const_stable(feature = "const_int_overflow", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn mul_with_overflow<T: Copy>(x: T, y: T) -> (T, bool);

    /// Performs an exact division, resulting in undefined behavior where
    /// `x % y != 0` or `y == 0` or `x == T::MIN && y == -1`
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_unstable(feature = "const_exact_div", issue = "none")]
    #[rustc_nounwind]
    pub fn exact_div<T: Copy>(x: T, y: T) -> T;

    /// Performs an unchecked division, resulting in undefined behavior
    /// where `y == 0` or `x == T::MIN && y == -1`
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_div` method. For example,
    /// [`u32::checked_div`]
    #[rustc_const_stable(feature = "const_int_unchecked_div", since = "1.52.0")]
    #[rustc_nounwind]
    pub fn unchecked_div<T: Copy>(x: T, y: T) -> T;
    /// Returns the remainder of an unchecked division, resulting in
    /// undefined behavior when `y == 0` or `x == T::MIN && y == -1`
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_rem` method. For example,
    /// [`u32::checked_rem`]
    #[rustc_const_stable(feature = "const_int_unchecked_rem", since = "1.52.0")]
    #[rustc_nounwind]
    pub fn unchecked_rem<T: Copy>(x: T, y: T) -> T;

    /// Performs an unchecked left shift, resulting in undefined behavior when
    /// `y < 0` or `y >= N`, where N is the width of T in bits.
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_shl` method. For example,
    /// [`u32::checked_shl`]
    #[rustc_const_stable(feature = "const_int_unchecked", since = "1.40.0")]
    #[rustc_nounwind]
    pub fn unchecked_shl<T: Copy>(x: T, y: T) -> T;
    /// Performs an unchecked right shift, resulting in undefined behavior when
    /// `y < 0` or `y >= N`, where N is the width of T in bits.
    ///
    /// Safe wrappers for this intrinsic are available on the integer
    /// primitives via the `checked_shr` method. For example,
    /// [`u32::checked_shr`]
    #[rustc_const_stable(feature = "const_int_unchecked", since = "1.40.0")]
    #[rustc_nounwind]
    pub fn unchecked_shr<T: Copy>(x: T, y: T) -> T;

    /// Returns the result of an unchecked addition, resulting in
    /// undefined behavior when `x + y > T::MAX` or `x + y < T::MIN`.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_unstable(feature = "const_int_unchecked_arith", issue = "none")]
    #[rustc_nounwind]
    pub fn unchecked_add<T: Copy>(x: T, y: T) -> T;

    /// Returns the result of an unchecked subtraction, resulting in
    /// undefined behavior when `x - y > T::MAX` or `x - y < T::MIN`.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_unstable(feature = "const_int_unchecked_arith", issue = "none")]
    #[rustc_nounwind]
    pub fn unchecked_sub<T: Copy>(x: T, y: T) -> T;

    /// Returns the result of an unchecked multiplication, resulting in
    /// undefined behavior when `x * y > T::MAX` or `x * y < T::MIN`.
    ///
    /// This intrinsic does not have a stable counterpart.
    #[rustc_const_unstable(feature = "const_int_unchecked_arith", issue = "none")]
    #[rustc_nounwind]
    pub fn unchecked_mul<T: Copy>(x: T, y: T) -> T;

    /// Performs rotate left.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `rotate_left` method. For example,
    /// [`u32::rotate_left`]
    #[rustc_const_stable(feature = "const_int_rotate", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn rotate_left<T: Copy>(x: T, y: T) -> T;

    /// Performs rotate right.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `rotate_right` method. For example,
    /// [`u32::rotate_right`]
    #[rustc_const_stable(feature = "const_int_rotate", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn rotate_right<T: Copy>(x: T, y: T) -> T;

    /// Returns (a + b) mod 2<sup>N</sup>, where N is the width of T in bits.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `wrapping_add` method. For example,
    /// [`u32::wrapping_add`]
    #[rustc_const_stable(feature = "const_int_wrapping", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn wrapping_add<T: Copy>(a: T, b: T) -> T;
    /// Returns (a - b) mod 2<sup>N</sup>, where N is the width of T in bits.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `wrapping_sub` method. For example,
    /// [`u32::wrapping_sub`]
    #[rustc_const_stable(feature = "const_int_wrapping", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn wrapping_sub<T: Copy>(a: T, b: T) -> T;
    /// Returns (a * b) mod 2<sup>N</sup>, where N is the width of T in bits.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `wrapping_mul` method. For example,
    /// [`u32::wrapping_mul`]
    #[rustc_const_stable(feature = "const_int_wrapping", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn wrapping_mul<T: Copy>(a: T, b: T) -> T;

    /// Computes `a + b`, saturating at numeric bounds.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `saturating_add` method. For example,
    /// [`u32::saturating_add`]
    #[rustc_const_stable(feature = "const_int_saturating", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn saturating_add<T: Copy>(a: T, b: T) -> T;
    /// Computes `a - b`, saturating at numeric bounds.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized versions of this intrinsic are available on the integer
    /// primitives via the `saturating_sub` method. For example,
    /// [`u32::saturating_sub`]
    #[rustc_const_stable(feature = "const_int_saturating", since = "1.40.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn saturating_sub<T: Copy>(a: T, b: T) -> T;

    /// This is an implementation detail of [`crate::ptr::read`] and should
    /// not be used anywhere else.  See its comments for why this exists.
    ///
    /// This intrinsic can *only* be called where the pointer is a local without
    /// projections (`read_via_copy(ptr)`, not `read_via_copy(*ptr)`) so that it
    /// trivially obeys runtime-MIR rules about derefs in operands.
    #[rustc_const_stable(feature = "const_ptr_read", since = "1.71.0")]
    #[rustc_nounwind]
    pub fn read_via_copy<T>(ptr: *const T) -> T;

    /// This is an implementation detail of [`crate::ptr::write`] and should
    /// not be used anywhere else.  See its comments for why this exists.
    ///
    /// This intrinsic can *only* be called where the pointer is a local without
    /// projections (`write_via_move(ptr, x)`, not `write_via_move(*ptr, x)`) so
    /// that it trivially obeys runtime-MIR rules about derefs in operands.
    #[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
    #[rustc_nounwind]
    pub fn write_via_move<T>(ptr: *mut T, value: T);

    /// Returns the value of the discriminant for the variant in 'v';
    /// if `T` has no discriminant, returns `0`.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The stabilized version of this intrinsic is [`core::mem::discriminant`].
    #[rustc_const_stable(feature = "const_discriminant", since = "1.75.0")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn discriminant_value<T>(v: &T) -> <T as DiscriminantKind>::Discriminant;

    /// Returns the number of variants of the type `T` cast to a `usize`;
    /// if `T` has no variants, returns `0`. Uninhabited variants will be counted.
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    ///
    /// The to-be-stabilized version of this intrinsic is [`crate::mem::variant_count`].
    #[rustc_const_unstable(feature = "variant_count", issue = "73662")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn variant_count<T>() -> usize;

    /// Rust's "try catch" construct for unwinding. Invokes the function pointer `try_fn` with the
    /// data pointer `data`, and calls `catch_fn` if unwinding occurs while `try_fn` runs.
    ///
    /// `catch_fn` must not unwind.
    ///
    /// The third argument is a function called if an unwind occurs (both Rust unwinds and foreign
    /// unwinds). This function takes the data pointer and a pointer to the target-specific
    /// exception object that was caught. For more information, see the compiler's source as well as
    /// std's `catch_unwind` implementation.
    ///
    /// The stable version of this intrinsic is `std::panic::catch_unwind`.
    #[rustc_nounwind]
    #[cfg(not(bootstrap))]
    pub fn catch_unwind(try_fn: fn(*mut u8), data: *mut u8, catch_fn: fn(*mut u8, *mut u8)) -> i32;

    /// For bootstrap only, see `catch_unwind`.
    #[rustc_nounwind]
    #[cfg(bootstrap)]
    pub fn r#try(try_fn: fn(*mut u8), data: *mut u8, catch_fn: fn(*mut u8, *mut u8)) -> i32;

    /// Emits a `!nontemporal` store according to LLVM (see their docs).
    /// Probably will never become stable.
    ///
    /// Do NOT use this intrinsic; "nontemporal" operations do not exist in our memory model!
    /// It exists to support current stdarch, but the plan is to change stdarch and remove this intrinsic.
    /// See <https://github.com/rust-lang/rust/issues/114582> for some more discussion.
    #[rustc_nounwind]
    pub fn nontemporal_store<T>(ptr: *mut T, val: T);

    /// See documentation of `<*const T>::offset_from` for details.
    #[rustc_const_stable(feature = "const_ptr_offset_from", since = "1.65.0")]
    #[rustc_nounwind]
    pub fn ptr_offset_from<T>(ptr: *const T, base: *const T) -> isize;

    /// See documentation of `<*const T>::sub_ptr` for details.
    #[rustc_const_unstable(feature = "const_ptr_sub_ptr", issue = "95892")]
    #[rustc_nounwind]
    pub fn ptr_offset_from_unsigned<T>(ptr: *const T, base: *const T) -> usize;

    /// See documentation of `<*const T>::guaranteed_eq` for details.
    /// Returns `2` if the result is unknown.
    /// Returns `1` if the pointers are guaranteed equal
    /// Returns `0` if the pointers are guaranteed inequal
    ///
    /// Note that, unlike most intrinsics, this is safe to call;
    /// it does not require an `unsafe` block.
    /// Therefore, implementations must not require the user to uphold
    /// any safety invariants.
    #[rustc_const_unstable(feature = "const_raw_ptr_comparison", issue = "53020")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn ptr_guaranteed_cmp<T>(ptr: *const T, other: *const T) -> u8;

    /// Determines whether the raw bytes of the two values are equal.
    ///
    /// This is particularly handy for arrays, since it allows things like just
    /// comparing `i96`s instead of forcing `alloca`s for `[6 x i16]`.
    ///
    /// Above some backend-decided threshold this will emit calls to `memcmp`,
    /// like slice equality does, instead of causing massive code size.
    ///
    /// Since this works by comparing the underlying bytes, the actual `T` is
    /// not particularly important.  It will be used for its size and alignment,
    /// but any validity restrictions will be ignored, not enforced.
    ///
    /// # Safety
    ///
    /// It's UB to call this if any of the *bytes* in `*a` or `*b` are uninitialized or carry a
    /// pointer value.
    /// Note that this is a stricter criterion than just the *values* being
    /// fully-initialized: if `T` has padding, it's UB to call this intrinsic.
    ///
    /// (The implementation is allowed to branch on the results of comparisons,
    /// which is UB if any of their inputs are `undef`.)
    #[rustc_const_unstable(feature = "const_intrinsic_raw_eq", issue = "none")]
    #[rustc_nounwind]
    pub fn raw_eq<T>(a: &T, b: &T) -> bool;

    /// Lexicographically compare `[left, left + bytes)` and `[right, right + bytes)`
    /// as unsigned bytes, returning negative if `left` is less, zero if all the
    /// bytes match, or positive if `right` is greater.
    ///
    /// This underlies things like `<[u8]>::cmp`, and will usually lower to `memcmp`.
    ///
    /// # Safety
    ///
    /// `left` and `right` must each be [valid] for reads of `bytes` bytes.
    ///
    /// Note that this applies to the whole range, not just until the first byte
    /// that differs.  That allows optimizations that can read in large chunks.
    ///
    /// [valid]: crate::ptr#safety
    #[rustc_const_unstable(feature = "const_intrinsic_compare_bytes", issue = "none")]
    #[rustc_nounwind]
    pub fn compare_bytes(left: *const u8, right: *const u8, bytes: usize) -> i32;

    /// See documentation of [`std::hint::black_box`] for details.
    ///
    /// [`std::hint::black_box`]: crate::hint::black_box
    #[rustc_const_unstable(feature = "const_black_box", issue = "none")]
    #[rustc_safe_intrinsic]
    #[rustc_nounwind]
    pub fn black_box<T>(dummy: T) -> T;

    #[rustc_nounwind]
    #[cfg(bootstrap)]
    pub fn vtable_size(ptr: *const ()) -> usize;

    /// `ptr` must point to a vtable.
    /// The intrinsic will return the alignment stored in that vtable.
    #[rustc_nounwind]
    pub fn vtable_align(ptr: *const ()) -> usize;

    /// Selects which function to call depending on the context.
    ///
    /// If this function is evaluated at compile-time, then a call to this
    /// intrinsic will be replaced with a call to `called_in_const`. It gets
    /// replaced with a call to `called_at_rt` otherwise.
    ///
    /// This function is safe to call, but note the stability concerns below.
    ///
    /// # Type Requirements
    ///
    /// The two functions must be both function items. They cannot be function
    /// pointers or closures. The first function must be a `const fn`.
    ///
    /// `arg` will be the tupled arguments that will be passed to either one of
    /// the two functions, therefore, both functions must accept the same type of
    /// arguments. Both functions must return RET.
    ///
    /// # Stability concerns
    ///
    /// Rust has not yet decided that `const fn` are allowed to tell whether
    /// they run at compile-time or at runtime. Therefore, when using this
    /// intrinsic anywhere that can be reached from stable, it is crucial that
    /// the end-to-end behavior of the stable `const fn` is the same for both
    /// modes of execution. (Here, Undefined Behavior is considered "the same"
    /// as any other behavior, so if the function exhibits UB at runtime then
    /// it may do whatever it wants at compile-time.)
    ///
    /// Here is an example of how this could cause a problem:
    /// ```no_run
    /// #![feature(const_eval_select)]
    /// #![feature(core_intrinsics)]
    /// # #![allow(internal_features)]
    /// # #![cfg_attr(bootstrap, allow(unused))]
    /// use std::intrinsics::const_eval_select;
    ///
    /// // Standard library
    /// # #[cfg(not(bootstrap))]
    /// pub const fn inconsistent() -> i32 {
    ///     fn runtime() -> i32 { 1 }
    ///     const fn compiletime() -> i32 { 2 }
    ///
    //      // ⚠ This code violates the required equivalence of `compiletime`
    ///     // and `runtime`.
    ///     const_eval_select((), compiletime, runtime)
    /// }
    /// # #[cfg(bootstrap)]
    /// # pub const fn inconsistent() -> i32 { 0 }
    ///
    /// // User Crate
    /// const X: i32 = inconsistent();
    /// let x = inconsistent();
    /// assert_eq!(x, X);
    /// ```
    ///
    /// Currently such an assertion would always succeed; until Rust decides
    /// otherwise, that principle should not be violated.
    #[rustc_const_unstable(feature = "const_eval_select", issue = "none")]
    #[cfg_attr(not(bootstrap), rustc_safe_intrinsic)]
    pub fn const_eval_select<ARG: Tuple, F, G, RET>(
        arg: ARG,
        called_in_const: F,
        called_at_rt: G,
    ) -> RET
    where
        G: FnOnce<ARG, Output = RET>,
        F: FnOnce<ARG, Output = RET>;
}

/// Returns whether the argument's value is statically known at
/// compile-time.
///
/// This is useful when there is a way of writing the code that will
/// be *faster* when some variables have known values, but *slower*
/// in the general case: an `if is_val_statically_known(var)` can be used
/// to select between these two variants. The `if` will be optimized away
/// and only the desired branch remains.
///
/// Formally speaking, this function non-deterministically returns `true`
/// or `false`, and the caller has to ensure sound behavior for both cases.
/// In other words, the following code has *Undefined Behavior*:
///
/// ```no_run
/// #![feature(is_val_statically_known)]
/// #![feature(core_intrinsics)]
/// # #![allow(internal_features)]
/// use std::hint::unreachable_unchecked;
/// use std::intrinsics::is_val_statically_known;
///
/// if !is_val_statically_known(0) { unsafe { unreachable_unchecked(); } }
/// ```
///
/// This also means that the following code's behavior is unspecified; it
/// may panic, or it may not:
///
/// ```no_run
/// #![feature(is_val_statically_known)]
/// #![feature(core_intrinsics)]
/// # #![allow(internal_features)]
/// use std::intrinsics::is_val_statically_known;
///
/// assert_eq!(is_val_statically_known(0), is_val_statically_known(0));
/// ```
///
/// Unsafe code may not rely on `is_val_statically_known` returning any
/// particular value, ever. However, the compiler will generally make it
/// return `true` only if the value of the argument is actually known.
///
/// When calling this in a `const fn`, both paths must be semantically
/// equivalent, that is, the result of the `true` branch and the `false`
/// branch must return the same value and have the same side-effects *no
/// matter what*.
#[rustc_const_unstable(feature = "is_val_statically_known", issue = "none")]
#[rustc_nounwind]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[cfg_attr(bootstrap, inline)]
pub const fn is_val_statically_known<T: Copy>(_arg: T) -> bool {
    false
}

/// Returns whether we should check for library UB. This evaluate to the value of `cfg!(debug_assertions)`
/// during monomorphization.
///
/// This intrinsic is evaluated after monomorphization, and therefore branching on this value can
/// be used to implement debug assertions that are included in the precompiled standard library,
/// but can be optimized out by builds that monomorphize the standard library code with debug
/// assertions disabled. This intrinsic is primarily used by [`assert_unsafe_precondition`].
///
/// We have separate intrinsics for library UB and language UB because checkers like the const-eval
/// interpreter and Miri already implement checks for language UB. Since such checkers do not know
/// about library preconditions, checks guarded by this intrinsic let them find more UB.
#[rustc_const_unstable(feature = "ub_checks", issue = "none")]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[inline(always)]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
pub(crate) const fn check_library_ub() -> bool {
    cfg!(debug_assertions)
}

/// Returns whether we should check for language UB. This evaluate to the value of `cfg!(debug_assertions)`
/// during monomorphization.
///
/// Since checks implemented at the source level must come strictly before the operation that
/// executes UB, if we enabled language UB checks in const-eval/Miri we would miss out on the
/// interpreter's improved diagnostics for the cases that our source-level checks catch.
///
/// See `check_library_ub` for more information.
#[rustc_const_unstable(feature = "ub_checks", issue = "none")]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[inline(always)]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
pub(crate) const fn check_language_ub() -> bool {
    cfg!(debug_assertions)
}

/// Allocates a block of memory at compile time.
/// At runtime, just returns a null pointer.
///
/// # Safety
///
/// - The `align` argument must be a power of two.
///    - At compile time, a compile error occurs if this constraint is violated.
///    - At runtime, it is not checked.
#[rustc_const_unstable(feature = "const_heap", issue = "79597")]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_nounwind]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[cfg_attr(bootstrap, inline)]
pub const unsafe fn const_allocate(_size: usize, _align: usize) -> *mut u8 {
    // const eval overrides this function, but runtime code should always just return null pointers.
    crate::ptr::null_mut()
}

/// Deallocates a memory which allocated by `intrinsics::const_allocate` at compile time.
/// At runtime, does nothing.
///
/// # Safety
///
/// - The `align` argument must be a power of two.
///    - At compile time, a compile error occurs if this constraint is violated.
///    - At runtime, it is not checked.
/// - If the `ptr` is created in an another const, this intrinsic doesn't deallocate it.
/// - If the `ptr` is pointing to a local variable, this intrinsic doesn't deallocate it.
#[rustc_const_unstable(feature = "const_heap", issue = "79597")]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_nounwind]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[cfg_attr(bootstrap, inline)]
pub const unsafe fn const_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {}

/// `ptr` must point to a vtable.
/// The intrinsic will return the size stored in that vtable.
#[rustc_nounwind]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[cfg_attr(not(bootstrap), rustc_intrinsic_must_be_overridden)]
#[cfg(not(bootstrap))]
pub unsafe fn vtable_size(_ptr: *const ()) -> usize {
    unreachable!()
}

/// Retag a box pointer as part of casting it to a raw pointer. This is the `Box` equivalent of
/// `(x: &mut T) as *mut T`. The input pointer must be the pointer of a `Box` (passed as raw pointer
/// to avoid all questions around move semantics and custom allocators), and `A` must be the `Box`'s
/// allocator.
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_nounwind]
#[cfg_attr(not(bootstrap), rustc_intrinsic)]
#[cfg_attr(bootstrap, inline)]
pub unsafe fn retag_box_to_raw<T: ?Sized, A>(ptr: *mut T) -> *mut T {
    // Miri needs to adjust the provenance, but for regular codegen this is not needed
    ptr
}

// Some functions are defined here because they accidentally got made
// available in this module on stable. See <https://github.com/rust-lang/rust/issues/15702>.
// (`transmute` also falls into this category, but it cannot be wrapped due to the
// check that `T` and `U` have the same size.)

/// Check that the preconditions of an unsafe function are followed. The check is enabled at
/// runtime if debug assertions are enabled when the caller is monomorphized. In const-eval/Miri
/// checks implemented with this macro for language UB are always ignored.
///
/// This macro should be called as
/// `assert_unsafe_precondition!(check_{library,lang}_ub, "message", (ident: type = expr, ident: type = expr) => check_expr)`
/// where each `expr` will be evaluated and passed in as function argument `ident: type`. Then all
/// those arguments are passed to a function with the body `check_expr`.
/// Pick `check_language_ub` when this is guarding a violation of language UB, i.e., immediate UB
/// according to the Rust Abstract Machine. Pick `check_library_ub` when this is guarding a violation
/// of a documented library precondition that does not *immediately* lead to language UB.
///
/// If `check_library_ub` is used but the check is actually guarding language UB, the check will
/// slow down const-eval/Miri and we'll get the panic message instead of the interpreter's nice
/// diagnostic, but our ability to detect UB is unchanged.
/// But if `check_language_ub` is used when the check is actually for library UB, the check is
/// omitted in const-eval/Miri and thus if we eventually execute language UB which relies on the
/// library UB, the backtrace Miri reports may be far removed from original cause.
///
/// These checks are behind a condition which is evaluated at codegen time, not expansion time like
/// [`debug_assert`]. This means that a standard library built with optimizations and debug
/// assertions disabled will have these checks optimized out of its monomorphizations, but if a
/// caller of the standard library has debug assertions enabled and monomorphizes an expansion of
/// this macro, that monomorphization will contain the check.
///
/// Since these checks cannot be optimized out in MIR, some care must be taken in both call and
/// implementation to mitigate their compile-time overhead. Calls to this macro always expand to
/// this structure:
/// ```ignore (pseudocode)
/// if ::core::intrinsics::check_language_ub() {
///     precondition_check(args)
/// }
/// ```
/// where `precondition_check` is monomorphic with the attributes `#[rustc_nounwind]`, `#[inline]` and
/// `#[rustc_no_mir_inline]`. This combination of attributes ensures that the actual check logic is
/// compiled only once and generates a minimal amount of IR because the check cannot be inlined in
/// MIR, but *can* be inlined and fully optimized by a codegen backend.
///
/// Callers should avoid introducing any other `let` bindings or any code outside this macro in
/// order to call it. Since the precompiled standard library is built with full debuginfo and these
/// variables cannot be optimized out in MIR, an innocent-looking `let` can produce enough
/// debuginfo to have a measurable compile-time impact on debug builds.
#[allow_internal_unstable(ub_checks)] // permit this to be called in stably-const fn
macro_rules! assert_unsafe_precondition {
    ($kind:ident, $message:expr, ($($name:ident:$ty:ty = $arg:expr),*$(,)?) => $e:expr $(,)?) => {
        {
            // #[cfg(bootstrap)] (this comment)
            // When the standard library is compiled with debug assertions, we want the check to inline for better performance.
            // This is important when working on the compiler, which is compiled with debug assertions locally.
            // When not compiled with debug assertions (so the precompiled std) we outline the check to minimize the compile
            // time impact when debug assertions are disabled.
            // The proper solution to this is the `#[rustc_no_mir_inline]` below, but we still want decent performance for cfg(bootstrap).
            #[cfg_attr(all(debug_assertions, bootstrap), inline(always))]
            #[cfg_attr(all(not(debug_assertions), bootstrap), inline(never))]

            // This check is inlineable, but not by the MIR inliner.
            // The reason for this is that the MIR inliner is in an exceptionally bad position
            // to think about whether or not to inline this. In MIR, this call is gated behind `debug_assertions`,
            // which will codegen to `false` in release builds. Inlining the check would be wasted work in that case and
            // would be bad for compile times.
            //
            // LLVM on the other hand sees the constant branch, so if it's `false`, it can immediately delete it without
            // inlining the check. If it's `true`, it can inline it and get significantly better performance.
            #[cfg_attr(not(bootstrap), rustc_no_mir_inline)]
            #[cfg_attr(not(bootstrap), inline)]
            #[rustc_nounwind]
            #[rustc_const_unstable(feature = "ub_checks", issue = "none")]
            const fn precondition_check($($name:$ty),*) {
                if !$e {
                    ::core::panicking::panic_nounwind(
                        concat!("unsafe precondition(s) violated: ", $message)
                    );
                }
            }

            if ::core::intrinsics::$kind() {
                precondition_check($($arg,)*);
            }
        }
    };
}
pub(crate) use assert_unsafe_precondition;

/// Checks whether `ptr` is properly aligned with respect to
/// `align_of::<T>()`.
///
/// In `const` this is approximate and can fail spuriously. It is primarily intended
/// for `assert_unsafe_precondition!` with `check_language_ub`, in which case the
/// check is anyway not executed in `const`.
#[inline]
pub(crate) const fn is_aligned_and_not_null(ptr: *const (), align: usize) -> bool {
    !ptr.is_null() && ptr.is_aligned_to(align)
}

#[inline]
pub(crate) const fn is_valid_allocation_size(size: usize, len: usize) -> bool {
    let max_len = if size == 0 { usize::MAX } else { isize::MAX as usize / size };
    len <= max_len
}

/// Checks whether the regions of memory starting at `src` and `dst` of size
/// `count * size` do *not* overlap.
///
/// Note that in const-eval this function just returns `true` and therefore must
/// only be used with `assert_unsafe_precondition!`, similar to `is_aligned_and_not_null`.
#[inline]
pub(crate) const fn is_nonoverlapping(
    src: *const (),
    dst: *const (),
    size: usize,
    count: usize,
) -> bool {
    #[inline]
    fn runtime(src: *const (), dst: *const (), size: usize, count: usize) -> bool {
        let src_usize = src.addr();
        let dst_usize = dst.addr();
        let Some(size) = size.checked_mul(count) else {
            crate::panicking::panic_nounwind(
                "is_nonoverlapping: `size_of::<T>() * count` overflows a usize",
            )
        };
        let diff = src_usize.abs_diff(dst_usize);
        // If the absolute distance between the ptrs is at least as big as the size of the buffer,
        // they do not overlap.
        diff >= size
    }

    #[inline]
    const fn comptime(_: *const (), _: *const (), _: usize, _: usize) -> bool {
        true
    }

    #[cfg_attr(not(bootstrap), allow(unused_unsafe))] // on bootstrap bump, remove unsafe block
    // SAFETY: This function's precondition is equivalent to that of `const_eval_select`.
    // Programs which do not execute UB will only see this function return `true`, which makes the
    // const and runtime implementation indistinguishable.
    unsafe {
        const_eval_select((src, dst, size, count), comptime, runtime)
    }
}

/// Copies `count * size_of::<T>()` bytes from `src` to `dst`. The source
/// and destination must *not* overlap.
///
/// For regions of memory which might overlap, use [`copy`] instead.
///
/// `copy_nonoverlapping` is semantically equivalent to C's [`memcpy`], but
/// with the argument order swapped.
///
/// The copy is "untyped" in the sense that data may be uninitialized or otherwise violate the
/// requirements of `T`. The initialization state is preserved exactly.
///
/// [`memcpy`]: https://en.cppreference.com/w/c/string/byte/memcpy
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be [valid] for reads of `count * size_of::<T>()` bytes.
///
/// * `dst` must be [valid] for writes of `count * size_of::<T>()` bytes.
///
/// * Both `src` and `dst` must be properly aligned.
///
/// * The region of memory beginning at `src` with a size of `count *
///   size_of::<T>()` bytes must *not* overlap with the region of memory
///   beginning at `dst` with the same size.
///
/// Like [`read`], `copy_nonoverlapping` creates a bitwise copy of `T`, regardless of
/// whether `T` is [`Copy`]. If `T` is not [`Copy`], using *both* the values
/// in the region beginning at `*src` and the region beginning at `*dst` can
/// [violate memory safety][read-ownership].
///
/// Note that even if the effectively copied size (`count * size_of::<T>()`) is
/// `0`, the pointers must be non-null and properly aligned.
///
/// [`read`]: crate::ptr::read
/// [read-ownership]: crate::ptr::read#ownership-of-the-returned-value
/// [valid]: crate::ptr#safety
///
/// # Examples
///
/// Manually implement [`Vec::append`]:
///
/// ```
/// use std::ptr;
///
/// /// Moves all the elements of `src` into `dst`, leaving `src` empty.
/// fn append<T>(dst: &mut Vec<T>, src: &mut Vec<T>) {
///     let src_len = src.len();
///     let dst_len = dst.len();
///
///     // Ensure that `dst` has enough capacity to hold all of `src`.
///     dst.reserve(src_len);
///
///     unsafe {
///         // The call to add is always safe because `Vec` will never
///         // allocate more than `isize::MAX` bytes.
///         let dst_ptr = dst.as_mut_ptr().add(dst_len);
///         let src_ptr = src.as_ptr();
///
///         // Truncate `src` without dropping its contents. We do this first,
///         // to avoid problems in case something further down panics.
///         src.set_len(0);
///
///         // The two regions cannot overlap because mutable references do
///         // not alias, and two different vectors cannot own the same
///         // memory.
///         ptr::copy_nonoverlapping(src_ptr, dst_ptr, src_len);
///
///         // Notify `dst` that it now holds the contents of `src`.
///         dst.set_len(dst_len + src_len);
///     }
/// }
///
/// let mut a = vec!['r'];
/// let mut b = vec!['u', 's', 't'];
///
/// append(&mut a, &mut b);
///
/// assert_eq!(a, &['r', 'u', 's', 't']);
/// assert!(b.is_empty());
/// ```
///
/// [`Vec::append`]: ../../std/vec/struct.Vec.html#method.append
#[doc(alias = "memcpy")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_allowed_through_unstable_modules]
#[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]
#[inline(always)]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
#[rustc_diagnostic_item = "ptr_copy_nonoverlapping"]
pub const unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
    extern "rust-intrinsic" {
        #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]
        #[rustc_nounwind]
        pub fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize);
    }

    assert_unsafe_precondition!(
        check_language_ub,
        "ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null \
        and the specified memory ranges do not overlap",
        (
            src: *const () = src as *const (),
            dst: *mut () = dst as *mut (),
            size: usize = size_of::<T>(),
            align: usize = align_of::<T>(),
            count: usize = count,
        ) =>
        is_aligned_and_not_null(src, align)
            && is_aligned_and_not_null(dst, align)
            && is_nonoverlapping(src, dst, size, count)
    );

    // SAFETY: the safety contract for `copy_nonoverlapping` must be
    // upheld by the caller.
    unsafe { copy_nonoverlapping(src, dst, count) }
}

/// Copies `count * size_of::<T>()` bytes from `src` to `dst`. The source
/// and destination may overlap.
///
/// If the source and destination will *never* overlap,
/// [`copy_nonoverlapping`] can be used instead.
///
/// `copy` is semantically equivalent to C's [`memmove`], but with the argument
/// order swapped. Copying takes place as if the bytes were copied from `src`
/// to a temporary array and then copied from the array to `dst`.
///
/// The copy is "untyped" in the sense that data may be uninitialized or otherwise violate the
/// requirements of `T`. The initialization state is preserved exactly.
///
/// [`memmove`]: https://en.cppreference.com/w/c/string/byte/memmove
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be [valid] for reads of `count * size_of::<T>()` bytes, and must remain valid even
///   when `dst` is written for `count * size_of::<T>()` bytes. (This means if the memory ranges
///   overlap, the two pointers must not be subject to aliasing restrictions relative to each
///   other.)
///
/// * `dst` must be [valid] for writes of `count * size_of::<T>()` bytes, and must remain valid even
///   when `src` is read for `count * size_of::<T>()` bytes.
///
/// * Both `src` and `dst` must be properly aligned.
///
/// Like [`read`], `copy` creates a bitwise copy of `T`, regardless of
/// whether `T` is [`Copy`]. If `T` is not [`Copy`], using both the values
/// in the region beginning at `*src` and the region beginning at `*dst` can
/// [violate memory safety][read-ownership].
///
/// Note that even if the effectively copied size (`count * size_of::<T>()`) is
/// `0`, the pointers must be non-null and properly aligned.
///
/// [`read`]: crate::ptr::read
/// [read-ownership]: crate::ptr::read#ownership-of-the-returned-value
/// [valid]: crate::ptr#safety
///
/// # Examples
///
/// Efficiently create a Rust vector from an unsafe buffer:
///
/// ```
/// use std::ptr;
///
/// /// # Safety
/// ///
/// /// * `ptr` must be correctly aligned for its type and non-zero.
/// /// * `ptr` must be valid for reads of `elts` contiguous elements of type `T`.
/// /// * Those elements must not be used after calling this function unless `T: Copy`.
/// # #[allow(dead_code)]
/// unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
///     let mut dst = Vec::with_capacity(elts);
///
///     // SAFETY: Our precondition ensures the source is aligned and valid,
///     // and `Vec::with_capacity` ensures that we have usable space to write them.
///     ptr::copy(ptr, dst.as_mut_ptr(), elts);
///
///     // SAFETY: We created it with this much capacity earlier,
///     // and the previous `copy` has initialized these elements.
///     dst.set_len(elts);
///     dst
/// }
/// ```
#[doc(alias = "memmove")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_allowed_through_unstable_modules]
#[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]
#[inline(always)]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
#[rustc_diagnostic_item = "ptr_copy"]
pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) {
    extern "rust-intrinsic" {
        #[rustc_const_unstable(feature = "const_intrinsic_copy", issue = "80697")]
        #[rustc_nounwind]
        fn copy<T>(src: *const T, dst: *mut T, count: usize);
    }

    // SAFETY: the safety contract for `copy` must be upheld by the caller.
    unsafe {
        assert_unsafe_precondition!(
            check_language_ub,
            "ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null \
            and the specified memory ranges do not overlap",
            (
                src: *const () = src as *const (),
                dst: *mut () = dst as *mut (),
                align: usize = align_of::<T>(),
            ) =>
            is_aligned_and_not_null(src, align)
                && is_aligned_and_not_null(dst, align)
        );
        copy(src, dst, count)
    }
}

/// Sets `count * size_of::<T>()` bytes of memory starting at `dst` to
/// `val`.
///
/// `write_bytes` is similar to C's [`memset`], but sets `count *
/// size_of::<T>()` bytes to `val`.
///
/// [`memset`]: https://en.cppreference.com/w/c/string/byte/memset
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `dst` must be [valid] for writes of `count * size_of::<T>()` bytes.
///
/// * `dst` must be properly aligned.
///
/// Note that even if the effectively copied size (`count * size_of::<T>()`) is
/// `0`, the pointer must be non-null and properly aligned.
///
/// Additionally, note that changing `*dst` in this way can easily lead to undefined behavior (UB)
/// later if the written bytes are not a valid representation of some `T`. For instance, the
/// following is an **incorrect** use of this function:
///
/// ```rust,no_run
/// unsafe {
///     let mut value: u8 = 0;
///     let ptr: *mut bool = &mut value as *mut u8 as *mut bool;
///     let _bool = ptr.read(); // This is fine, `ptr` points to a valid `bool`.
///     ptr.write_bytes(42u8, 1); // This function itself does not cause UB...
///     let _bool = ptr.read(); // ...but it makes this operation UB! ⚠️
/// }
/// ```
///
/// [valid]: crate::ptr#safety
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::ptr;
///
/// let mut vec = vec![0u32; 4];
/// unsafe {
///     let vec_ptr = vec.as_mut_ptr();
///     ptr::write_bytes(vec_ptr, 0xfe, 2);
/// }
/// assert_eq!(vec, [0xfefefefe, 0xfefefefe, 0, 0]);
/// ```
#[doc(alias = "memset")]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_allowed_through_unstable_modules]
#[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
#[inline(always)]
#[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
#[rustc_diagnostic_item = "ptr_write_bytes"]
pub const unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize) {
    extern "rust-intrinsic" {
        #[rustc_const_unstable(feature = "const_ptr_write", issue = "86302")]
        #[rustc_nounwind]
        fn write_bytes<T>(dst: *mut T, val: u8, count: usize);
    }

    // SAFETY: the safety contract for `write_bytes` must be upheld by the caller.
    unsafe {
        assert_unsafe_precondition!(
            check_language_ub,
            "ptr::write_bytes requires that the destination pointer is aligned and non-null",
            (
                addr: *const () = dst as *const (),
                align: usize = align_of::<T>(),
            ) => is_aligned_and_not_null(addr, align)
        );
        write_bytes(dst, val, count)
    }
}

/// Inform Miri that a given pointer definitely has a certain alignment.
#[cfg(miri)]
pub(crate) const fn miri_promise_symbolic_alignment(ptr: *const (), align: usize) {
    extern "Rust" {
        /// Miri-provided extern function to promise that a given pointer is properly aligned for
        /// "symbolic" alignment checks. Will fail if the pointer is not actually aligned or `align` is
        /// not a power of two. Has no effect when alignment checks are concrete (which is the default).
        fn miri_promise_symbolic_alignment(ptr: *const (), align: usize);
    }

    fn runtime(ptr: *const (), align: usize) {
        // SAFETY: this call is always safe.
        unsafe {
            miri_promise_symbolic_alignment(ptr, align);
        }
    }

    const fn compiletime(_ptr: *const (), _align: usize) {}

    #[cfg_attr(not(bootstrap), allow(unused_unsafe))] // on bootstrap bump, remove unsafe block
    // SAFETY: the extra behavior at runtime is for UB checks only.
    unsafe {
        const_eval_select((ptr, align), compiletime, runtime);
    }
}
