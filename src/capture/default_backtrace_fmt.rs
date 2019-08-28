use super::BacktraceFrame;
use std::ffi::c_void;
use std::fmt;

pub(super) fn fmt_backtrace(frames: &[BacktraceFrame], fmt: &mut fmt::Formatter) -> fmt::Result {
    for (idx, frame) in frames.iter().enumerate() {
        // To reduce TCB size in Sgx enclave, we do not want to implement symbol resolution functionality.
        // Rather, we can print the offset of the address here, which could be later mapped to
        // correct function.
        let ip: *mut c_void;
        #[cfg(target_env = "sgx")]
        {
            ip = usize::wrapping_sub(
                frame.ip() as _,
                std::os::fortanix_sgx::mem::image_base() as _,
            ) as _;
        }
        #[cfg(not(target_env = "sgx"))]
        {
            ip = frame.ip();
        }

        write!(fmt, "\n{:4}: ", idx)?;

        let symbols = match frame.symbols {
            Some(ref s) => s,
            None => {
                write!(fmt, "<unresolved> ({:?})", ip)?;
                continue;
            }
        };
        if symbols.len() == 0 {
            write!(fmt, "<no info> ({:?})", ip)?;
            continue;
        }

        for (idx, symbol) in symbols.iter().enumerate() {
            if idx != 0 {
                write!(fmt, "\n      ")?;
            }

            if let Some(name) = symbol.name() {
                write!(fmt, "{}", name)?;
            } else {
                write!(fmt, "<unknown>")?;
            }

            if idx == 0 {
                write!(fmt, " ({:?})", ip)?;
            }

            if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                write!(fmt, "\n             at {}:{}", file.display(), line)?;
            }
        }
    }

    Ok(())
}
