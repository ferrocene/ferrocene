use std::fmt;
use std::os::raw::c_void;
use std::str;

use demangle::{demangle, Demangle};

/// A trait representing the resolution of a symbol in a file.
///
/// This trait is yielded as a trait object to the closure given to the
/// `backtrace::resolve` function, and it is virtually dispatched as it's
/// unknown which implementation is behind it.
///
/// A symbol can give contextual information about a funciton, for example the
/// name, filename, line number, precise address, etc. Not all information is
/// always available in a symbol, however, so all methods return an `Option`.
pub trait Symbol {
    /// Returns the name of this function as a demangled symbol.
    fn name(&self) -> Option<SymbolName> { None }

    /// Returns the starting address of this function.
    fn addr(&self) -> Option<*mut c_void> { None }

    /// Returns the file name where this function was defined.
    ///
    /// This is currently only available when libbacktrace is being used (e.g.
    /// unix platforms other than OSX) and when a binary is compiled with
    /// debuginfo. If neither of these conditions is met then this will likely
    /// return `None`.
    fn filename(&self) -> Option<&[u8]> { None }

    /// Returns the line number for where this symbol is currently executing.
    ///
    /// This return value is typically `Some` if `filename` returns `Some`, and
    /// is consequently subject to similar caveats.
    fn lineno(&self) -> Option<u32> { None }
}

/// A wrapper around a symbol name to provide ergonomic accessors to the
/// demangled name, the raw bytes, the raw string, etc.
pub struct SymbolName<'a> {
    bytes: &'a [u8],
    demangled: Option<Demangle<'a>>,
}

/// Resolve an address to a symbol, passing the symbol to the specified
/// closure.
///
/// This function will look up the given address in areas such as the local
/// symbol table, dynamic symbol table, or DWARF debug info (depending on the
/// activated implementation) to find symbols to yield.
///
/// The closure may not be called if resolution could not be performed, and it
/// also may be called more than once in the case of inlined functions.
///
/// Symbols yielded represent the execution at the specified `addr`, returning
/// file/line pairs for that addres (if available).
///
/// # Example
///
/// ```
/// extern crate backtrace;
///
/// fn main() {
///     backtrace::trace(|frame| {
///         let ip = frame.ip();
///
///         backtrace::resolve(ip, |symbol| {
///             // ...
///         });
///
///         false // only look at the top frame
///     });
/// }
/// ```
pub fn resolve<F: FnMut(&Symbol)>(addr: *mut c_void, mut cb: F) {
    resolve_imp(addr, &mut cb)
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("Symbol");
        if let Some(name) = self.name() {
            d.field("name", &name);
        }
        if let Some(addr) = self.addr() {
            d.field("addr", &addr);
        }
        if let Some(filename) = self.filename() {
            d.field("filename", &String::from_utf8_lossy(filename));
        }
        if let Some(lineno) = self.lineno() {
            d.field("lineno", &lineno);
        }
        d.finish()
    }
}

impl<'a> SymbolName<'a> {
    /// Creates a new symbol name from the raw underlying bytes.
    pub fn new(bytes: &'a [u8]) -> SymbolName<'a> {
        let demangled = str::from_utf8(bytes).ok().map(demangle);
        SymbolName {
            bytes: bytes,
            demangled: demangled,
        }
    }

    /// Returns the raw symbol name as `&str` if the symbols is valid utf-8.
    pub fn as_str(&self) -> Option<&'a str> {
        self.demangled.as_ref().map(|s| s.as_str())
    }

    /// Returns the raw symbol name as a list of bytes
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> fmt::Display for SymbolName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref s) = self.demangled {
            s.fmt(f)
        } else {
            String::from_utf8_lossy(self.bytes).fmt(f)
        }
    }
}

impl<'a> fmt::Debug for SymbolName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref s) = self.demangled {
            s.fmt(f)
        } else {
            String::from_utf8_lossy(self.bytes).fmt(f)
        }
    }
}

cfg_if! {
    if #[cfg(all(windows, feature = "dbghelp"))] {
        mod dbghelp;
        use self::dbghelp::resolve as resolve_imp;
    } else if #[cfg(all(feature = "libbacktrace", unix,
                        not(target_os = "macos")))] {
        mod libbacktrace;
        use self::libbacktrace::resolve as resolve_imp;
    } else if #[cfg(all(unix, feature = "dladdr"))] {
        mod dladdr;
        use self::dladdr::resolve as resolve_imp;
    } else {
        mod noop;
        use self::noop::resolve as resolve_imp;
    }
}

