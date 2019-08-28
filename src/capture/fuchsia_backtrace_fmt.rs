use super::BacktraceFrame;
use libc::c_void;
use std::fmt;

pub(super) fn fmt_backtrace(frames: &[BacktraceFrame], fmt: &mut fmt::Formatter) -> fmt::Result {
    // Print the DSO context to tell the logger where our libs are loaded into memory.
    let mut dso_cx = vec![];
    fuchsia_backtrace::print_dso_context(&mut dso_cx).map_err(|_| fmt::Error)?;
    if let Ok(s) = std::string::String::from_utf8(dso_cx) {
        fmt.write_str(&s)?;
    }

    // Print the addresses of the backtrace frames
    for (idx, frame) in frames.iter().enumerate() {
        let ip: *mut c_void = frame.ip();
        fmt.write_str("{{{bt:")?;
        write!(fmt, "{}:{:?}", idx, ip)?;
        fmt.write_str("}}}\n")?;
    }

    Ok(())
}
