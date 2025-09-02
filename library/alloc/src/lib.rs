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

#![allow(incomplete_features)]
#![allow(unused_attributes)]
#![stable(feature = "alloc", since = "1.36.0")]
#![doc(
    html_playground_url = "https://play.rust-lang.org/",
    issue_tracker_base_url = "https://github.com/rust-lang/rust/issues/",
    test(no_crate_inject, attr(allow(unused_variables), deny(warnings)))
)]
#![doc(cfg_hide(
    not(test),
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
#![warn(rustdoc::unescaped_backticks)]
#![deny(ffi_unwind_calls)]
#![warn(unreachable_pub)]
//
// Ferrocene addition: We removed the tidy directives for alphabetical ordering to reduce the number
// of conflicts we have when merging main.
//
// Library features:
<<<<<<< HEAD
// not-alphabetical-start
#![cfg_attr(not(feature = "ferrocene_certified"), feature(alloc_layout_extra))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(allocator_api))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(array_into_iter_constructors))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(array_windows))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(ascii_char))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(assert_matches))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(async_fn_traits))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(async_iterator))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(bstr))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(bstr_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(cast_maybe_uninit))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(char_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(char_max_len))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(clone_to_uninit))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(coerce_unsized))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(const_default))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(const_eval_select))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(const_heap))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(const_trait_impl))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(core_intrinsics))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(deprecated_suggestion))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(deref_pure_trait))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(dispatch_from_dyn))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(ergonomic_clones))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(error_generic_member_access))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(exact_size_is_empty))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(extend_one))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(extend_one_unchecked))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(fmt_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(fn_traits))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(formatting_options))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(generic_atomic))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(hasher_prefixfree_extras))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(inplace_iteration))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(iter_advance_by))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(iter_next_chunk))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(layout_for_ptr))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(legacy_receiver_trait))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(local_waker))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(maybe_uninit_slice))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(maybe_uninit_uninit_array_transpose))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(panic_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(pattern))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(pin_coerce_unsized_trait))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(ptr_alignment_type))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(ptr_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(ptr_metadata))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(set_ptr_value))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(sized_type_properties))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(slice_from_ptr_range))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(slice_index_methods))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(slice_iter_mut_as_mut_slice))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(slice_ptr_get))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(slice_range))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(std_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(str_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(temporary_niche_types))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(trusted_fused))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(trusted_len))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(trusted_random_access))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(try_trait_v2))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(try_with_capacity))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(tuple_trait))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(ub_checks))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(unicode_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(unsize))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(unwrap_infallible))]
// not-alphabetical-end
=======
// tidy-alphabetical-start
#![feature(alloc_layout_extra)]
#![feature(allocator_api)]
#![feature(array_into_iter_constructors)]
#![feature(array_windows)]
#![feature(ascii_char)]
#![feature(assert_matches)]
#![feature(async_fn_traits)]
#![feature(async_iterator)]
#![feature(bstr)]
#![feature(bstr_internals)]
#![feature(cast_maybe_uninit)]
#![feature(char_internals)]
#![feature(char_max_len)]
#![feature(clone_to_uninit)]
#![feature(coerce_unsized)]
#![feature(const_default)]
#![feature(const_eval_select)]
#![feature(const_heap)]
#![feature(const_trait_impl)]
#![feature(core_intrinsics)]
#![feature(deprecated_suggestion)]
#![feature(deref_pure_trait)]
#![feature(dispatch_from_dyn)]
#![feature(ergonomic_clones)]
#![feature(error_generic_member_access)]
#![feature(exact_size_is_empty)]
#![feature(extend_one)]
#![feature(extend_one_unchecked)]
#![feature(fmt_internals)]
#![feature(fn_traits)]
#![feature(formatting_options)]
#![feature(generic_atomic)]
#![feature(hasher_prefixfree_extras)]
#![feature(inplace_iteration)]
#![feature(iter_advance_by)]
#![feature(iter_next_chunk)]
#![feature(layout_for_ptr)]
#![feature(legacy_receiver_trait)]
#![feature(local_waker)]
#![feature(maybe_uninit_slice)]
#![feature(maybe_uninit_uninit_array_transpose)]
#![feature(panic_internals)]
#![feature(pattern)]
#![feature(pin_coerce_unsized_trait)]
#![feature(ptr_alignment_type)]
#![feature(ptr_internals)]
#![feature(ptr_metadata)]
#![feature(set_ptr_value)]
#![feature(sized_type_properties)]
#![feature(slice_from_ptr_range)]
#![feature(slice_index_methods)]
#![feature(slice_iter_mut_as_mut_slice)]
#![feature(slice_ptr_get)]
#![feature(slice_range)]
#![feature(std_internals)]
#![feature(str_internals)]
#![feature(temporary_niche_types)]
#![feature(trusted_fused)]
#![feature(trusted_len)]
#![feature(trusted_random_access)]
#![feature(try_trait_v2)]
#![feature(try_with_capacity)]
#![feature(tuple_trait)]
#![feature(ub_checks)]
#![feature(unicode_internals)]
#![feature(unsize)]
#![feature(unwrap_infallible)]
#![feature(wtf8_internals)]
// tidy-alphabetical-end
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
//
// Language features:
// not-alphbetical-start
#![feature(allocator_internals)]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(allow_internal_unstable))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(cfg_sanitize))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(const_precise_live_drops))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(coroutine_trait))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(decl_macro))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(dropck_eyepatch))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(fundamental))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(hashmap_internals))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(intrinsics))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(lang_items))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(min_specialization))]
#![feature(multiple_supertrait_upcastable)]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(negative_impls))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(never_type))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(optimize_attribute))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(rustc_allow_const_fn_unstable))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(rustc_attrs))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(slice_internals))]
#![feature(staged_api)]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(stmt_expr_attributes))]
#![feature(strict_provenance_lints)]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(unboxed_closures))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(unsized_fn_params))]
#![cfg_attr(not(feature = "ferrocene_certified"), feature(with_negative_coherence))]
#![cfg_attr(not(feature = "ferrocene_certified"), rustc_preserve_ub_checks)]
// not-alphbetical-end
//
// Rustdoc features:
#![cfg_attr(not(feature = "ferrocene_certified"), feature(doc_cfg))]
#![feature(doc_cfg_hide)]
// Technically, this is a bug in rustdoc: rustdoc sees the documentation on `#[lang = slice_alloc]`
// blocks is for `&[T]`, which also has documentation using this feature in `core`, and gets mad
// that the feature-gate isn't enabled. Ideally, it wouldn't check for the feature gate for docs
// from other crates, but since this can only appear for lang items, it doesn't seem worth fixing.
#![cfg_attr(not(feature = "ferrocene_certified"), feature(intra_doc_pointers))]
//
// Ferrocene lints/features:
#![cfg_attr(feature = "ferrocene_certified", allow(rustdoc::broken_intra_doc_links))]

// Module with internal macros used by other modules (needs to be included before other modules).
#[macro_use]
#[cfg(not(feature = "ferrocene_certified"))]
mod macros;

#[cfg(not(feature = "ferrocene_certified"))]
mod raw_vec;

// Heaps provided for low-level allocation strategies
#[cfg(not(feature = "ferrocene_certified"))]
pub mod alloc;

// Primitive types using the heaps above

// Need to conditionally define the mod from `boxed.rs` to avoid
// duplicating the lang-items when building in test cfg; but also need
// to allow code to have `use boxed::Box;` declarations.
#[cfg(not(feature = "ferrocene_certified"))]
pub mod borrow;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod boxed;
#[unstable(feature = "bstr", issue = "134915")]
#[cfg(not(feature = "ferrocene_certified"))]
pub mod bstr;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod collections;
#[cfg(all(not(no_rc), not(no_sync), not(no_global_oom_handling)))]
#[cfg(not(feature = "ferrocene_certified"))]
pub mod ffi;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod fmt;
#[cfg(not(no_rc))]
#[cfg(not(feature = "ferrocene_certified"))]
pub mod rc;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod slice;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod str;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod string;
#[cfg(all(not(no_rc), not(no_sync), target_has_atomic = "ptr"))]
#[cfg(not(feature = "ferrocene_certified"))]
pub mod sync;
#[cfg(all(not(no_global_oom_handling), not(no_rc), not(no_sync)))]
#[cfg(not(feature = "ferrocene_certified"))]
pub mod task;
#[cfg(not(feature = "ferrocene_certified"))]
pub mod vec;
#[cfg(all(not(no_rc), not(no_sync), not(no_global_oom_handling)))]
pub mod wtf8;

#[doc(hidden)]
#[unstable(feature = "liballoc_internals", issue = "none", reason = "implementation detail")]
#[cfg(not(feature = "ferrocene_certified"))]
pub mod __export {
    pub use core::format_args;
    pub use core::hint::must_use;
}
