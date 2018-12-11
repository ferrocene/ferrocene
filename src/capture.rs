use std::prelude::v1::*;
use std::fmt;
use std::path::{Path, PathBuf};

use {trace, resolve, SymbolName};
use types::c_void;

/// Representation of an owned and self-contained backtrace.
///
/// This structure can be used to capture a backtrace at various points in a
/// program and later used to inspect what the backtrace was at that time.
///
/// `Backtrace` supports pretty-printing of backtraces by implementing
/// `Display` and `Debug` (which do the same thing).
#[derive(Clone)]
#[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "serialize-serde", derive(Deserialize, Serialize))]
pub struct Backtrace {
    // Frames here are listed from top-to-bottom of the stack
    frames: Vec<BacktraceFrame>,
    // The index we believe is the actual start of the backtrace, omitting
    // frames like `Backtrace::new` and `backtrace::trace`.
    actual_start_index: usize,
}

/// Captured version of a frame in a backtrace.
///
/// This type is returned as a list from `Backtrace::frames` and represents one
/// stack frame in a captured backtrace.
#[derive(Clone)]
#[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "serialize-serde", derive(Deserialize, Serialize))]
pub struct BacktraceFrame {
    ip: usize,
    symbol_address: usize,
    symbols: Option<Vec<BacktraceSymbol>>,
}

/// Captured version of a symbol in a backtrace.
///
/// This type is returned as a list from `BacktraceFrame::symbols` and
/// represents the metadata for a symbol in a backtrace.
#[derive(Clone)]
#[cfg_attr(feature = "serialize-rustc", derive(RustcDecodable, RustcEncodable))]
#[cfg_attr(feature = "serialize-serde", derive(Deserialize, Serialize))]
pub struct BacktraceSymbol {
    name: Option<Vec<u8>>,
    addr: Option<usize>,
    filename: Option<PathBuf>,
    lineno: Option<u32>,
}

impl Backtrace {
    /// Captures a backtrace at the callsite of this function, returning an
    /// owned representation.
    ///
    /// This function is useful for representing a backtrace as an object in
    /// Rust. This returned value can be sent across threads and printed
    /// elsewhere, and the purpose of this value is to be entirely self
    /// contained.
    ///
    /// # Examples
    ///
    /// ```
    /// use backtrace::Backtrace;
    ///
    /// let current_backtrace = Backtrace::new();
    /// ```
    #[inline(never)] // want to make sure there's a frame here to remove
    pub fn new() -> Backtrace {
        let mut bt = Self::create(Self::new as usize);
        bt.resolve();
        bt
    }

    /// Similar to `new` except that this does not resolve any symbols, this
    /// simply captures the backtrace as a list of addresses.
    ///
    /// At a later time the `resolve` function can be called to resolve this
    /// backtrace's symbols into readable names. This function exists because
    /// the resolution process can sometimes take a significant amount of time
    /// whereas any one backtrace may only be rarely printed.
    ///
    /// # Examples
    ///
    /// ```
    /// use backtrace::Backtrace;
    ///
    /// let mut current_backtrace = Backtrace::new_unresolved();
    /// println!("{:?}", current_backtrace); // no symbol names
    /// current_backtrace.resolve();
    /// println!("{:?}", current_backtrace); // symbol names now present
    /// ```
    #[inline(never)] // want to make sure there's a frame here to remove
    pub fn new_unresolved() -> Backtrace {
        Self::create(Self::new_unresolved as usize)
    }

    fn create(ip: usize) -> Backtrace {
        let ip_lo = ip;
        let ip_hi = ip + 128;

        let mut frames = Vec::new();
        let mut actual_start_index = None;
        trace(|frame| {
            let ip = frame.ip() as usize;
            frames.push(BacktraceFrame {
                ip,
                symbol_address: frame.symbol_address() as usize,
                symbols: None,
            });

            if cfg!(not(all(target_os = "windows", target_arch = "x86"))) &&
                ip >= ip_lo &&
                ip <= ip_hi &&
                actual_start_index.is_none()
            {
                actual_start_index = Some(frames.len());
            }
            true
        });

        Backtrace {
            frames,
            actual_start_index: actual_start_index.unwrap_or(0),
        }
    }

    /// Returns the frames from when this backtrace was captured.
    ///
    /// The first entry of this slice is likely the function `Backtrace::new`,
    /// and the last frame is likely something about how this thread or the main
    /// function started.
    pub fn frames(&self) -> &[BacktraceFrame] {
        &self.frames[self.actual_start_index..]
    }

    /// If this backtrace was created from `new_unresolved` then this function
    /// will resolve all addresses in the backtrace to their symbolic names.
    ///
    /// If this backtrace has been previously resolved or was created through
    /// `new`, this function does nothing.
    pub fn resolve(&mut self) {
        for frame in self.frames.iter_mut().filter(|f| f.symbols.is_none()) {
            let mut symbols = Vec::new();
            resolve(frame.ip as *mut _, |symbol| {
                symbols.push(BacktraceSymbol {
                    name: symbol.name().map(|m| m.as_bytes().to_vec()),
                    addr: symbol.addr().map(|a| a as usize),
                    filename: symbol.filename().map(|m| m.to_owned()),
                    lineno: symbol.lineno(),
                });
            });
            frame.symbols = Some(symbols);
        }
    }
}

impl From<Vec<BacktraceFrame>> for Backtrace {
    fn from(frames: Vec<BacktraceFrame>) -> Self {
        Backtrace {
            frames,
            actual_start_index: 0,
        }
    }
}

impl Into<Vec<BacktraceFrame>> for Backtrace {
    fn into(self) -> Vec<BacktraceFrame> {
        self.frames
    }
}

impl BacktraceFrame {
    /// Same as `Frame::ip`
    pub fn ip(&self) -> *mut c_void {
        self.ip as *mut c_void
    }

    /// Same as `Frame::symbol_address`
    pub fn symbol_address(&self) -> *mut c_void {
        self.symbol_address as *mut c_void
    }

    /// Returns the list of symbols that this frame corresponds to.
    ///
    /// Normally there is only one symbol per frame, but sometimes if a number
    /// of functions are inlined into one frame then multiple symbols will be
    /// returned. The first symbol listed is the "innermost function", whereas
    /// the last symbol is the outermost (last caller).
    ///
    /// Note that if this frame came from an unresolved backtrace then this will
    /// return an empty list.
    pub fn symbols(&self) -> &[BacktraceSymbol] {
        self.symbols.as_ref().map(|s| &s[..]).unwrap_or(&[])
    }
}

impl BacktraceSymbol {
    /// Same as `Symbol::name`
    pub fn name(&self) -> Option<SymbolName> {
        self.name.as_ref().map(|s| SymbolName::new(s))
    }

    /// Same as `Symbol::addr`
    pub fn addr(&self) -> Option<*mut c_void> {
        self.addr.map(|s| s as *mut c_void)
    }

    /// Same as `Symbol::filename`
    pub fn filename(&self) -> Option<&Path> {
        self.filename.as_ref().map(|p| &**p)
    }

    /// Same as `Symbol::lineno`
    pub fn lineno(&self) -> Option<u32> {
        self.lineno
    }
}

impl fmt::Debug for Backtrace {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "stack backtrace:")?;

        let iter = if fmt.alternate() {
            self.frames.iter()
        } else {
            self.frames[self.actual_start_index..].iter()
        };

        for (idx, frame) in iter.enumerate() {
            let ip = frame.ip();
            write!(fmt, "\n{:4}: ", idx)?;

            let symbols = match frame.symbols {
                Some(ref s) => s,
                None => {
                    write!(fmt, "<unresolved> ({:?})", ip)?;
                    continue
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
}

impl Default for Backtrace {
    fn default() -> Backtrace {
        Backtrace::new()
    }
}
