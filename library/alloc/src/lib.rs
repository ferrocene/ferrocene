//! # The Rust core allocation and collections library
//!
//! This library provides smart pointers and collections for managing
//! heap-allocated values.
//!
//! This library, like core, normally doesn’t need to be used directly
//! since its contents are re-exported in the [`std` crate](../std/index.html).
//! Crates that use the `#![no_std]` attribute however will typically
//! not depend on `std`, so they’d use this crate instead.
//!
//! ## Boxed values
//!
//! The [`Box`] type is a smart pointer type. There can only be one owner of a
//! [`Box`], and the owner can decide to mutate the contents, which live on the
//! heap.
//!
//! This type can be sent among threads efficiently as the size of a `Box` value
//! is the same as that of a pointer. Tree-like data structures are often built
//! with boxes because each node often has only one owner, the parent.
//!
//! ## Reference counted pointers
//!
//! The [`Rc`] type is a non-threadsafe reference-counted pointer type intended
//! for sharing memory within a thread. An [`Rc`] pointer wraps a type, `T`, and
//! only allows access to `&T`, a shared reference.
//!
//! This type is useful when inherited mutability (such as using [`Box`]) is too
//! constraining for an application, and is often paired with the [`Cell`] or
//! [`RefCell`] types in order to allow mutation.
//!
//! ## Atomically reference counted pointers
//!
//! The [`Arc`] type is the threadsafe equivalent of the [`Rc`] type. It
//! provides all the same functionality of [`Rc`], except it requires that the
//! contained type `T` is shareable. Additionally, [`Arc<T>`][`Arc`] is itself
//! sendable while [`Rc<T>`][`Rc`] is not.
//!
//! This type allows for shared access to the contained data, and is often
//! paired with synchronization primitives such as mutexes to allow mutation of
//! shared resources.
//!
//! ## Collections
//!
//! Implementations of the most common general purpose data structures are
//! defined in this library. They are re-exported through the
//! [standard collections library](../std/collections/index.html).
//!
//! ## Heap interfaces
//!
//! The [`alloc`](alloc/index.html) module defines the low-level interface to the
//! default global allocator. It is not compatible with the libc allocator API.
//!
//! [`Arc`]: sync
//! [`Box`]: boxed
//! [`Cell`]: core::cell
//! [`Rc`]: rc
//! [`RefCell`]: core::cell

// To run alloc tests without x.py without ending up with two copies of alloc, Miri needs to be
// able to "empty" this crate. See <https://github.com/rust-lang/miri-test-libstd/issues/4>.
// rustc itself never sets the feature, so this line has no effect there.
#![cfg(any(not(feature = "miri-test-libstd"), test, doctest))]
//
#![allow(unused_attributes)]
#![stable(feature = "alloc", since = "1.36.0")]
#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
    test(no_crate_inject, attr(allow(unused_variables), deny(warnings)))
)]
#![doc(cfg_hide(
    not(test),
    not(any(test, bootstrap)),
    any(not(feature = "miri-test-libstd"), test, doctest),
    no_global_oom_handling,
    not(no_global_oom_handling),
    not(no_rc),
    not(no_sync),
    target_has_atomic = "ptr"
))]
#![doc(rust_logo)]
#![feature(rustdoc_internals)]
#![no_std]
#![needs_allocator]
// Lints:
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(fuzzy_provenance_casts)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(explicit_outlives_requirements)]
#![warn(multiple_supertrait_upcastable)]
#![allow(internal_features)]
#![allow(rustdoc::redundant_explicit_links)]
#![deny(ffi_unwind_calls)]
//
// Library features:
// tidy-alphabetical-start
#![cfg_attr(not(no_global_oom_handling), feature(const_alloc_error))]
#![cfg_attr(not(no_global_oom_handling), feature(const_btree_len))]
#![cfg_attr(test, feature(is_sorted))]
#![cfg_attr(test, feature(new_uninit))]
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(array_chunks)]
#![feature(array_into_iter_constructors)]
#![feature(array_windows)]
#![feature(ascii_char)]
#![feature(assert_matches)]
#![feature(async_fn_traits)]
#![feature(async_iterator)]
#![feature(coerce_unsized)]
#![feature(const_align_of_val)]
#![feature(const_box)]
#![feature(const_cow_is_borrowed)]
#![feature(const_eval_select)]
#![feature(const_maybe_uninit_as_mut_ptr)]
#![feature(const_maybe_uninit_write)]
#![feature(const_pin)]
#![feature(const_refs_to_cell)]
#![feature(const_size_of_val)]
#![feature(const_waker)]
#![feature(core_intrinsics)]
#![feature(deprecated_suggestion)]
#![feature(dispatch_from_dyn)]
#![feature(error_generic_member_access)]
#![feature(error_in_core)]
#![feature(exact_size_is_empty)]
#![feature(extend_one)]
#![feature(fmt_internals)]
#![feature(fn_traits)]
#![feature(generic_nonzero)]
#![feature(hasher_prefixfree_extras)]
#![feature(hint_assert_unchecked)]
#![feature(inline_const)]
#![feature(inplace_iteration)]
#![feature(iter_advance_by)]
#![feature(iter_next_chunk)]
#![feature(iter_repeat_n)]
#![feature(layout_for_ptr)]
#![feature(local_waker)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(non_null_convenience)]
#![feature(panic_internals)]
#![feature(pattern)]
#![feature(ptr_internals)]
#![feature(ptr_metadata)]
#![feature(ptr_sub_ptr)]
#![feature(receiver_trait)]
#![feature(set_ptr_value)]
#![feature(sized_type_properties)]
#![feature(slice_from_ptr_range)]
#![feature(slice_index_methods)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(slice_range)]
#![feature(std_internals)]
#![feature(str_internals)]
#![feature(strict_provenance)]
#![feature(trusted_fused)]
#![feature(trusted_len)]
#![feature(trusted_random_access)]
#![feature(try_trait_v2)]
#![feature(try_with_capacity)]
#![feature(tuple_trait)]
#![feature(unchecked_math)]
#![feature(unicode_internals)]
#![feature(unsize)]
#![feature(utf8_chunks)]
// tidy-alphabetical-end
//
// Language features:
// tidy-alphabetical-start
#![cfg_attr(not(test), feature(coroutine_trait))]
#![cfg_attr(test, feature(panic_update_hook))]
#![cfg_attr(test, feature(test))]
#![feature(allocator_internals)]
#![feature(allow_internal_unstable)]
#![feature(associated_type_bounds)]
#![feature(c_unwind)]
#![feature(cfg_sanitize)]
#![feature(const_mut_refs)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_write)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(decl_macro)]
#![feature(dropck_eyepatch)]
#![feature(exclusive_range_pattern)]
#![feature(fundamental)]
#![feature(hashmap_internals)]
#![feature(lang_items)]
#![feature(min_specialization)]
#![feature(multiple_supertrait_upcastable)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(pointer_is_aligned)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(rustc_attrs)]
#![feature(slice_internals)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
#![feature(unboxed_closures)]
#![feature(unsized_fn_params)]
#![feature(with_negative_coherence)]
// tidy-alphabetical-end
//
// Rustdoc features:
#![feature(doc_cfg)]
#![feature(doc_cfg_hide)]
// Technically, this is a bug in rustdoc: rustdoc sees the documentation on `#[lang = slice_alloc]`
// blocks is for `&[T]`, which also has documentation using this feature in `core`, and gets mad
// that the feature-gate isn't enabled. Ideally, it wouldn't check for the feature gate for docs
// from other crates, but since this can only appear for lang items, it doesn't seem worth fixing.
#![feature(intra_doc_pointers)]

// Allow testing this library
#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
extern crate test;
#[cfg(test)]
mod testing;

// Module with internal macros used by other modules (needs to be included before other modules).
#[macro_use]
mod macros;

mod raw_vec;

// Heaps provided for low-level allocation strategies

pub mod alloc;

// Primitive types using the heaps above

// Need to conditionally define the mod from `boxed.rs` to avoid
// duplicating the lang-items when building in test cfg; but also need
// to allow code to have `use boxed::Box;` declarations.
#[cfg(not(test))]
pub mod boxed;
#[cfg(test)]
mod boxed {
    pub use std::boxed::Box;
}
pub mod borrow;
pub mod collections;
#[cfg(all(not(no_rc), not(no_sync), not(no_global_oom_handling)))]
pub mod ffi;
pub mod fmt;
#[cfg(not(no_rc))]
pub mod rc;
pub mod slice;
pub mod str;
pub mod string;
#[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]
pub mod sync;
#[cfg(all(not(no_global_oom_handling), not(no_rc), not(no_sync)))]
pub mod task;
#[cfg(test)]
mod tests;
pub mod vec;

#[doc(hidden)]
#[unstable(feature = "liballoc_internals", issue = "none", reason = "implementation detail")]
pub mod __export {
    pub use core::format_args;
}

#[cfg(test)]
#[allow(dead_code)] // Not used in all configurations
pub(crate) mod test_helpers {
    /// Copied from `std::test_helpers::test_rng`, since these tests rely on the
    /// seed not being the same for every RNG invocation too.
    pub(crate) fn test_rng() -> rand_xorshift::XorShiftRng {
        use std::hash::{BuildHasher, Hash, Hasher};
        let mut hasher = std::hash::RandomState::new().build_hasher();
        std::panic::Location::caller().hash(&mut hasher);
        let hc64 = hasher.finish();
        let seed_vec =
            hc64.to_le_bytes().into_iter().chain(0u8..8).collect::<crate::vec::Vec<u8>>();
        let seed: [u8; 16] = seed_vec.as_slice().try_into().unwrap();
        rand::SeedableRng::from_seed(seed)
    }
}
