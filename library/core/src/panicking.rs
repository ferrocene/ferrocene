//! Panic support for core
//!
//! The core library cannot define panicking, but it does *declare* panicking. This
//! means that the functions inside of core are allowed to panic, but to be
//! useful an upstream crate must define panicking for core to use. The current
//! interface for panicking is:
//!
//! ```
//! fn panic_impl(pi: &core::panic::PanicInfo<'_>) -> !
//! # { loop {} }
//! ```
//!
//! This definition allows for panicking with any general message, but it does not
//! allow for failing with a `Box<Any>` value. (`PanicInfo` just contains a `&(dyn Any + Send)`,
//! for which we fill in a dummy value in `PanicInfo::internal_constructor`.)
//! The reason for this is that core is not allowed to allocate.
//!
//! This module contains a few other panicking functions, but these are just the
//! necessary lang items for the compiler. All panics are funneled through this
//! one function. The actual symbol is declared through the `#[panic_handler]` attribute.

#![allow(dead_code, missing_docs)]
#![unstable(
    feature = "panic_internals",
    reason = "internal details of the implementation of the `panic!` and related macros",
    issue = "none"
)]

use crate::fmt;
use crate::panic::{Location, PanicInfo};

#[cfg(feature = "panic_immediate_abort")]
const _: () = assert!(cfg!(panic = "abort"), "panic_immediate_abort requires -C panic=abort");

// First we define the two main entry points that all panics go through.
// In the end both are just convenience wrappers around `panic_impl`.

/// The entry point for panicking with a formatted message.
///
/// This is designed to reduce the amount of code required at the call
/// site as much as possible (so that `panic!()` has as low an impact
/// on (e.g.) the inlining of other functions as possible), by moving
/// the actual formatting into this shared place.
// If panic_immediate_abort, inline the abort call,
// otherwise avoid inlining because of it is cold path.
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
#[lang = "panic_fmt"] // needed for const-evaluated panics
#[rustc_do_not_const_check] // hooked by const-eval
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn panic_fmt(fmt: fmt::Arguments<'_>) -> ! {
    if cfg!(feature = "panic_immediate_abort") {
        super::intrinsics::abort()
    }

    // NOTE This function never crosses the FFI boundary; it's a Rust-to-Rust call
    // that gets resolved to the `#[panic_handler]` function.
    extern "Rust" {
        #[lang = "panic_impl"]
        fn panic_impl(pi: &PanicInfo<'_>) -> !;
    }

    let pi = PanicInfo::internal_constructor(
        Some(&fmt),
        Location::caller(),
        /* can_unwind */ true,
        /* force_no_backtrace */ false,
    );

    // SAFETY: `panic_impl` is defined in safe Rust code and thus is safe to call.
    unsafe { panic_impl(&pi) }
}

/// Like `panic_fmt`, but for non-unwinding panics.
///
/// Has to be a separate function so that it can carry the `rustc_nounwind` attribute.
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
// This attribute has the key side-effect that if the panic handler ignores `can_unwind`
// and unwinds anyway, we will hit the "unwinding out of nounwind function" guard,
// which causes a "panic in a function that cannot unwind".
#[rustc_nounwind]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn panic_nounwind_fmt(fmt: fmt::Arguments<'_>, force_no_backtrace: bool) -> ! {
    #[inline] // this should always be inlined into `panic_nounwind_fmt`
    #[track_caller]
    fn runtime(fmt: fmt::Arguments<'_>, force_no_backtrace: bool) -> ! {
        if cfg!(feature = "panic_immediate_abort") {
            super::intrinsics::abort()
        }

        // NOTE This function never crosses the FFI boundary; it's a Rust-to-Rust call
        // that gets resolved to the `#[panic_handler]` function.
        extern "Rust" {
            #[lang = "panic_impl"]
            fn panic_impl(pi: &PanicInfo<'_>) -> !;
        }

        // PanicInfo with the `can_unwind` flag set to false forces an abort.
        let pi = PanicInfo::internal_constructor(
            Some(&fmt),
            Location::caller(),
            /* can_unwind */ false,
            force_no_backtrace,
        );

        // SAFETY: `panic_impl` is defined in safe Rust code and thus is safe to call.
        unsafe { panic_impl(&pi) }
    }

    #[inline]
    #[track_caller]
    const fn comptime(fmt: fmt::Arguments<'_>, _force_no_backtrace: bool) -> ! {
        // We don't unwind anyway at compile-time so we can call the regular `panic_fmt`.
        panic_fmt(fmt);
    }

    #[cfg_attr(not(bootstrap), allow(unused_unsafe))] // on bootstrap bump, remove unsafe block
    // SAFETY: const panic does not care about unwinding
    unsafe {
        super::intrinsics::const_eval_select((fmt, force_no_backtrace), comptime, runtime);
    }
}

// Next we define a bunch of higher-level wrappers that all bottom out in the two core functions
// above.

/// The underlying implementation of core's `panic!` macro when no formatting is used.
// never inline unless panic_immediate_abort to avoid code
// bloat at the call sites as much as possible
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
#[lang = "panic"] // needed by codegen for panic on overflow and other `Assert` MIR terminators
pub const fn panic(expr: &'static str) -> ! {
    // Use Arguments::new_v1 instead of format_args!("{expr}") to potentially
    // reduce size overhead. The format_args! macro uses str's Display trait to
    // write expr, which calls Formatter::pad, which must accommodate string
    // truncation and padding (even though none is used here). Using
    // Arguments::new_v1 may allow the compiler to omit Formatter::pad from the
    // output binary, saving up to a few kilobytes.
    panic_fmt(fmt::Arguments::new_const(&[expr]));
}

/// Like `panic`, but without unwinding and track_caller to reduce the impact on codesize on the caller.
/// If you want `#[track_caller]` for nicer errors, call `panic_nounwind_fmt` directly.
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[lang = "panic_nounwind"] // needed by codegen for non-unwinding panics
#[rustc_nounwind]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn panic_nounwind(expr: &'static str) -> ! {
    panic_nounwind_fmt(fmt::Arguments::new_const(&[expr]), /* force_no_backtrace */ false);
}

/// Like `panic_nounwind`, but also inhibits showing a backtrace.
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[rustc_nounwind]
pub fn panic_nounwind_nobacktrace(expr: &'static str) -> ! {
    panic_nounwind_fmt(fmt::Arguments::new_const(&[expr]), /* force_no_backtrace */ true);
}

#[inline]
#[track_caller]
#[rustc_diagnostic_item = "panic_str"]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn panic_str(expr: &str) -> ! {
    panic_display(&expr);
}

#[track_caller]
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn panic_explicit() -> ! {
    panic_display(&"explicit panic");
}

#[inline]
#[track_caller]
#[rustc_diagnostic_item = "unreachable_display"] // needed for `non-fmt-panics` lint
pub fn unreachable_display<T: fmt::Display>(x: &T) -> ! {
    panic_fmt(format_args!("internal error: entered unreachable code: {}", *x));
}

#[inline]
#[track_caller]
#[rustc_do_not_const_check] // hooked by const-eval
// enforce a &&str argument in const-check and hook this by const-eval
#[rustc_const_panic_str]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn panic_display<T: fmt::Display>(x: &T) -> ! {
    panic_fmt(format_args!("{}", *x));
}

#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
#[lang = "panic_bounds_check"] // needed by codegen for panic on OOB array/slice access
fn panic_bounds_check(index: usize, len: usize) -> ! {
    if cfg!(feature = "panic_immediate_abort") {
        super::intrinsics::abort()
    }

    panic!("index out of bounds: the len is {len} but the index is {index}")
}

#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
#[lang = "panic_misaligned_pointer_dereference"] // needed by codegen for panic on misaligned pointer deref
#[rustc_nounwind] // `CheckAlignment` MIR pass requires this function to never unwind
fn panic_misaligned_pointer_dereference(required: usize, found: usize) -> ! {
    if cfg!(feature = "panic_immediate_abort") {
        super::intrinsics::abort()
    }

    panic_nounwind_fmt(
        format_args!(
            "misaligned pointer dereference: address must be a multiple of {required:#x} but is {found:#x}"
        ),
        /* force_no_backtrace */ false,
    )
}

/// Panic because we cannot unwind out of a function.
///
/// This is a separate function to avoid the codesize impact of each crate containing the string to
/// pass to `panic_nounwind`.
/// This function is called directly by the codegen backend, and must not have
/// any extra arguments (including those synthesized by track_caller).
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[lang = "panic_cannot_unwind"] // needed by codegen for panic in nounwind function
#[rustc_nounwind]
fn panic_cannot_unwind() -> ! {
    // Keep the text in sync with `UnwindTerminateReason::as_str` in `rustc_middle`.
    panic_nounwind("panic in a function that cannot unwind")
}

/// Panic because we are unwinding out of a destructor during cleanup.
///
/// This is a separate function to avoid the codesize impact of each crate containing the string to
/// pass to `panic_nounwind`.
/// This function is called directly by the codegen backend, and must not have
/// any extra arguments (including those synthesized by track_caller).
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[lang = "panic_in_cleanup"] // needed by codegen for panic in nounwind function
#[rustc_nounwind]
fn panic_in_cleanup() -> ! {
    // Keep the text in sync with `UnwindTerminateReason::as_str` in `rustc_middle`.
    panic_nounwind_nobacktrace("panic in a destructor during cleanup")
}

/// This function is used instead of panic_fmt in const eval.
#[lang = "const_panic_fmt"]
#[rustc_const_unstable(feature = "panic_internals", issue = "none")]
pub const fn const_panic_fmt(fmt: fmt::Arguments<'_>) -> ! {
    if let Some(msg) = fmt.as_str() {
        // The panic_display function is hooked by const eval.
        panic_display(&msg);
    } else {
        // SAFETY: This is only evaluated at compile time, which reliably
        // handles this UB (in case this branch turns out to be reachable
        // somehow).
        unsafe { crate::hint::unreachable_unchecked() };
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub enum AssertKind {
    Eq,
    Ne,
    Match,
}

/// Internal function for `assert_eq!` and `assert_ne!` macros
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
#[doc(hidden)]
pub fn assert_failed<T, U>(
    kind: AssertKind,
    left: &T,
    right: &U,
    args: Option<fmt::Arguments<'_>>,
) -> !
where
    T: fmt::Debug + ?Sized,
    U: fmt::Debug + ?Sized,
{
    assert_failed_inner(kind, &left, &right, args)
}

/// Internal function for `assert_match!`
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
#[doc(hidden)]
pub fn assert_matches_failed<T: fmt::Debug + ?Sized>(
    left: &T,
    right: &str,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    // The pattern is a string so it can be displayed directly.
    struct Pattern<'a>(&'a str);
    impl fmt::Debug for Pattern<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }
    assert_failed_inner(AssertKind::Match, &left, &Pattern(right), args);
}

/// Non-generic version of the above functions, to avoid code bloat.
#[cfg_attr(not(feature = "panic_immediate_abort"), inline(never), cold)]
#[cfg_attr(feature = "panic_immediate_abort", inline)]
#[track_caller]
fn assert_failed_inner(
    kind: AssertKind,
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    let op = match kind {
        AssertKind::Eq => "==",
        AssertKind::Ne => "!=",
        AssertKind::Match => "matches",
    };

    match args {
        Some(args) => panic!(
            r#"assertion `left {op} right` failed: {args}
  left: {left:?}
 right: {right:?}"#
        ),
        None => panic!(
            r#"assertion `left {op} right` failed
  left: {left:?}
 right: {right:?}"#
        ),
    }
}
