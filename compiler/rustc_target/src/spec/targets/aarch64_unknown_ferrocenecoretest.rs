//! This custom target (only available in Ferrocene) is meant to be used internally to test whether
//! the core library works, as we need a standard library to run tests.
//!
//! The target has the exact same configuration as the target we want to test, with just the bits
//! specific to the standard library enabled.
//!
//! THIS IS TEMPORARY. We implemented this solution as we needed to run bare-metal tests for
//! qualification, but we're planning a cleaner implementation to upstream.

use crate::spec::{cvs, Target};

pub fn target() -> Target {
    let mut target = super::aarch64_unknown_none::target();
    target.os = "linux".into();
    target.env = "gnu".into();
    target.families = cvs!["unix"];
    target
}
