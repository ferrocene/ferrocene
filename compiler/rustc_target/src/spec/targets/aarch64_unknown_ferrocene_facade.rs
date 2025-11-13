//! This custom target (only available in Ferrocene) is meant to be used internally to test whether
//! the core library works, as we need a standard library to run tests.
//!
//! The target has the exact same configuration as the target we want to test, with just the bits
//! specific to the standard library enabled.
//!
//! THIS IS TEMPORARY. We implemented this solution as we needed to run bare-metal tests for
//! qualification, but we're planning a cleaner implementation to upstream.

use crate::spec::{Env, LinkSelfContainedDefault, Os, Target, TargetMetadata, crt_objects, cvs};

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_none::target();

    target.metadata = TargetMetadata::default();

    // libstd port
    target.families = cvs!["unix"];
    target.os = Os::Linux;
    target.env = Env::Musl;
    target.has_thread_local = true;

    // crt and libc are self-contained
    target.pre_link_objects_self_contained = crt_objects::all("crt1.o");
    target.link_self_contained = LinkSelfContainedDefault::True;

    target
}
