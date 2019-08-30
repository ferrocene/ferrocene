use super::BacktraceFrame;
use core::fmt;

pub(super) fn fmt_backtrace(frames: &[BacktraceFrame], fmt: &mut fmt::Formatter) -> fmt::Result {
    crate::fmt_fuchsia_backtrace(frames.iter().map(|frame| frame.ip()), fmt)
}
