//! This custom target (only available in Ferrocene) is meant to be used internally to test whether
//! the core library works, as we need a standard library to run tests.
//!
//! The target has the exact same configuration as the target we want to test, with just the bits
//! specific to the standard library enabled.
//!
//! THIS IS TEMPORARY. We implemented this solution as we needed to run bare-metal tests for
//! qualification, but we're planning a cleaner implementation to upstream.

use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetOptions, cvs};

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_none::target();
    target.os = "linux".into();
    target.env = "gnu".into();
    target.families = cvs!["unix"];
    // aarch64-unknown-none uses Cc:No, LLd::Yes. Here we want to use
    // the same underlying linker, but we want to use GCC/clang as the linker
    // driver so it can add system-specific paths to the linker command line for
    // us.
    //
    // Setting Lld::Yes causes `-fuse-ld=lld` to be added to the linker args
    // (and by linker I mean `cc` as the linker as we use Cc::Yes)
    target.linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::Yes);
    target.linker = Some("aarch64-linux-gnu-gcc".into());
    // Enable the Cortex-A53 errata 843419 mitigation by default
    //
    // NB: You have to pass Lld::No here; both ::No and ::Yes variants are added
    // to the options list by this function.
    target.pre_link_args = TargetOptions::link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &[
        "-Wl,--fix-cortex-a53-843419",
    ]);
    // This setting causes `-B path/to/gcc-ld` to be added to the linker args.
    // This means that that the `ld.lld` wrapper for `rust-lld` appears in the
    // path that `cc` uses to find `ld.lld` (also the path used for `cpp` and
    // `cc1`, but we don't supply those so it goes back to the defaults for
    // those tools).
    target.link_self_contained = crate::spec::LinkSelfContainedDefault::with_linker();
    target
}
