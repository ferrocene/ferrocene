use core::{fmt, str};

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::path::Path;
        use std::prelude::v1::*;
    }
}

use rustc_demangle::{try_demangle, Demangle};
use types::{c_void, BytesOrWideString};

use backtrace::Frame;

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
/// file/line pairs for that address (if available).
///
/// Note that if you have a `Frame` then it's recommended to use the
/// `resolve_frame` function instead of this one.
///
/// # Required features
///
/// This function requires the `std` feature of the `backtrace` crate to be
/// enabled, and the `std` feature is enabled by default.
///
/// # Panics
///
/// This function strives to never panic, but if the `cb` provided panics then
/// some platforms will force a double panic to abort the process. Some
/// platforms use a C library which internally uses callbacks which cannot be
/// unwound through, so panicking from `cb` may trigger a process abort.
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
#[cfg(feature = "std")]
pub fn resolve<F: FnMut(&Symbol)>(addr: *mut c_void, cb: F) {
    let _guard = ::lock::lock();
    unsafe { resolve_unsynchronized(addr, cb) }
}

/// Resolve a previously capture frame to a symbol, passing the symbol to the
/// specified closure.
///
/// This functin performs the same function as `resolve` except that it takes a
/// `Frame` as an argument instead of an address. This can allow some platform
/// implementations of backtracing to provide more accurate symbol information
/// or information about inline frames for example. It's recommended to use this
/// if you can.
///
/// # Required features
///
/// This function requires the `std` feature of the `backtrace` crate to be
/// enabled, and the `std` feature is enabled by default.
///
/// # Panics
///
/// This function strives to never panic, but if the `cb` provided panics then
/// some platforms will force a double panic to abort the process. Some
/// platforms use a C library which internally uses callbacks which cannot be
/// unwound through, so panicking from `cb` may trigger a process abort.
///
/// # Example
///
/// ```
/// extern crate backtrace;
///
/// fn main() {
///     backtrace::trace(|frame| {
///         backtrace::resolve_frame(frame, |symbol| {
///             // ...
///         });
///
///         false // only look at the top frame
///     });
/// }
/// ```
#[cfg(feature = "std")]
pub fn resolve_frame<F: FnMut(&Symbol)>(frame: &Frame, cb: F) {
    let _guard = ::lock::lock();
    unsafe { resolve_frame_unsynchronized(frame, cb) }
}

pub enum ResolveWhat<'a> {
    Address(*mut c_void),
    Frame(&'a Frame),
}

impl<'a> ResolveWhat<'a> {
    #[allow(dead_code)]
    fn address_or_ip(&self) -> *mut c_void {
        match *self {
            ResolveWhat::Address(a) => a,
            ResolveWhat::Frame(ref f) => f.ip(),
        }
    }
}

/// Same as `resolve`, only unsafe as it's unsynchronized.
///
/// This function does not have synchronization guarentees but is available when
/// the `std` feature of this crate isn't compiled in. See the `resolve`
/// function for more documentation and examples.
///
/// # Panics
///
/// See information on `resolve` for caveats on `cb` panicking.
pub unsafe fn resolve_unsynchronized<F>(addr: *mut c_void, mut cb: F)
where
    F: FnMut(&Symbol),
{
    resolve_imp(ResolveWhat::Address(addr), &mut cb)
}

/// Same as `resolve_frame`, only unsafe as it's unsynchronized.
///
/// This function does not have synchronization guarentees but is available
/// when the `std` feature of this crate isn't compiled in. See the
/// `resolve_frame` function for more documentation and examples.
///
/// # Panics
///
/// See information on `resolve_frame` for caveats on `cb` panicking.
pub unsafe fn resolve_frame_unsynchronized<F>(frame: &Frame, mut cb: F)
where
    F: FnMut(&Symbol),
{
    resolve_imp(ResolveWhat::Frame(frame), &mut cb)
}

/// A trait representing the resolution of a symbol in a file.
///
/// This trait is yielded as a trait object to the closure given to the
/// `backtrace::resolve` function, and it is virtually dispatched as it's
/// unknown which implementation is behind it.
///
/// A symbol can give contextual information about a function, for example the
/// name, filename, line number, precise address, etc. Not all information is
/// always available in a symbol, however, so all methods return an `Option`.
pub struct Symbol {
    inner: SymbolImp,
}

impl Symbol {
    /// Returns the name of this function.
    ///
    /// The returned structure can be used to query various properties about the
    /// symbol name:
    ///
    /// * The `Display` implementation will print out the demangled symbol.
    /// * The raw `str` value of the symbol can be accessed (if it's valid
    ///   utf-8).
    /// * The raw bytes for the symbol name can be accessed.
    pub fn name(&self) -> Option<SymbolName> {
        self.inner.name()
    }

    /// Returns the starting address of this function.
    pub fn addr(&self) -> Option<*mut c_void> {
        self.inner.addr().map(|p| p as *mut _)
    }

    /// Returns the raw filename as a slice. This is mainly useful for `no_std`
    /// environments.
    pub fn filename_raw(&self) -> Option<BytesOrWideString> {
        self.inner.filename_raw()
    }

    /// Returns the line number for where this symbol is currently executing.
    ///
    /// This return value is typically `Some` if `filename` returns `Some`, and
    /// is consequently subject to similar caveats.
    pub fn lineno(&self) -> Option<u32> {
        self.inner.lineno()
    }

    /// Returns the file name where this function was defined.
    ///
    /// This is currently only available when libbacktrace is being used (e.g.
    /// unix platforms other than OSX) and when a binary is compiled with
    /// debuginfo. If neither of these conditions is met then this will likely
    /// return `None`.
    ///
    /// # Required features
    ///
    /// This function requires the `std` feature of the `backtrace` crate to be
    /// enabled, and the `std` feature is enabled by default.
    #[cfg(feature = "std")]
    #[allow(unreachable_code)]
    pub fn filename(&self) -> Option<&Path> {
        self.inner.filename()
    }
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

        #[cfg(feature = "std")]
        {
            if let Some(filename) = self.filename() {
                d.field("filename", &filename);
            }
        }

        if let Some(lineno) = self.lineno() {
            d.field("lineno", &lineno);
        }
        d.finish()
    }
}

cfg_if! {
    if #[cfg(feature = "cpp_demangle")] {
        // Maybe a parsed C++ symbol, if parsing the mangled symbol as Rust
        // failed.
        struct OptionCppSymbol<'a>(Option<::cpp_demangle::BorrowedSymbol<'a>>);

        impl<'a> OptionCppSymbol<'a> {
            fn parse(input: &'a [u8]) -> OptionCppSymbol<'a> {
                OptionCppSymbol(::cpp_demangle::BorrowedSymbol::new(input).ok())
            }

            fn none() -> OptionCppSymbol<'a> {
                OptionCppSymbol(None)
            }
        }
    } else {
        use core::marker::PhantomData;

        // Make sure to keep this zero-sized, so that the `cpp_demangle` feature
        // has no cost when disabled.
        struct OptionCppSymbol<'a>(PhantomData<&'a ()>);

        impl<'a> OptionCppSymbol<'a> {
            fn parse(_: &'a [u8]) -> OptionCppSymbol<'a> {
                OptionCppSymbol(PhantomData)
            }

            fn none() -> OptionCppSymbol<'a> {
                OptionCppSymbol(PhantomData)
            }
        }
    }
}

/// A wrapper around a symbol name to provide ergonomic accessors to the
/// demangled name, the raw bytes, the raw string, etc.
// Allow dead code for when the `cpp_demangle` feature is not enabled.
#[allow(dead_code)]
pub struct SymbolName<'a> {
    bytes: &'a [u8],
    demangled: Option<Demangle<'a>>,
    cpp_demangled: OptionCppSymbol<'a>,
}

impl<'a> SymbolName<'a> {
    /// Creates a new symbol name from the raw underlying bytes.
    pub fn new(bytes: &'a [u8]) -> SymbolName<'a> {
        let str_bytes = str::from_utf8(bytes).ok();
        let demangled = str_bytes.and_then(|s| try_demangle(s).ok());

        let cpp = if demangled.is_none() {
            OptionCppSymbol::parse(bytes)
        } else {
            OptionCppSymbol::none()
        };

        SymbolName {
            bytes: bytes,
            demangled: demangled,
            cpp_demangled: cpp,
        }
    }

    /// Returns the raw (mangled) symbol name as a `str` if the symbol is valid utf-8.
    ///
    /// Use the `Display` implementation if you want the demangled version.
    pub fn as_str(&self) -> Option<&'a str> {
        self.demangled
            .as_ref()
            .map(|s| s.as_str())
            .or_else(|| str::from_utf8(self.bytes).ok())
    }

    /// Returns the raw symbol name as a list of bytes
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

fn format_symbol_name(
    fmt: fn(&str, &mut fmt::Formatter) -> fmt::Result,
    mut bytes: &[u8],
    f: &mut fmt::Formatter,
) -> fmt::Result {
    while bytes.len() > 0 {
        match str::from_utf8(bytes) {
            Ok(name) => {
                fmt(name, f)?;
                break;
            }
            Err(err) => {
                fmt("\u{FFFD}", f)?;

                match err.error_len() {
                    Some(len) => bytes = &bytes[err.valid_up_to() + len..],
                    None => break,
                }
            }
        }
    }
    Ok(())
}

cfg_if! {
    if #[cfg(feature = "cpp_demangle")] {
        impl<'a> fmt::Display for SymbolName<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if let Some(ref s) = self.demangled {
                    s.fmt(f)
                } else if let Some(ref cpp) = self.cpp_demangled.0 {
                    cpp.fmt(f)
                } else {
                    format_symbol_name(fmt::Display::fmt, self.bytes, f)
                }
            }
        }
    } else {
        impl<'a> fmt::Display for SymbolName<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if let Some(ref s) = self.demangled {
                    s.fmt(f)
                } else {
                    format_symbol_name(fmt::Display::fmt, self.bytes, f)
                }
            }
        }
    }
}

cfg_if! {
    if #[cfg(all(feature = "std", feature = "cpp_demangle"))] {
        impl<'a> fmt::Debug for SymbolName<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use std::fmt::Write;

                if let Some(ref s) = self.demangled {
                    return s.fmt(f)
                }

                // This may to print if the demangled symbol isn't actually
                // valid, so handle the error here gracefully by not propagating
                // it outwards.
                if let Some(ref cpp) = self.cpp_demangled.0 {
                    let mut s = String::new();
                    if write!(s, "{}", cpp).is_ok() {
                        return s.fmt(f)
                    }
                }

                format_symbol_name(fmt::Debug::fmt, self.bytes, f)
            }
        }
    } else {
        impl<'a> fmt::Debug for SymbolName<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if let Some(ref s) = self.demangled {
                    s.fmt(f)
                } else {
                    format_symbol_name(fmt::Debug::fmt, self.bytes, f)
                }
            }
        }
    }
}

mod dladdr;

cfg_if! {
    if #[cfg(all(windows, target_env = "msvc", feature = "dbghelp"))] {
        mod dbghelp;
        use self::dbghelp::resolve as resolve_imp;
        use self::dbghelp::Symbol as SymbolImp;
    } else if #[cfg(all(feature = "std",
                        feature = "gimli-symbolize",
                        target_os = "linux"))] {
        mod gimli;
        use self::gimli::resolve as resolve_imp;
        use self::gimli::Symbol as SymbolImp;
    // Note that we only enable coresymbolication on iOS when debug assertions
    // are enabled because it's helpful in debug mode but it looks like apps get
    // rejected from the app store if they use this API, see #92 for more info
    } else if #[cfg(all(feature = "coresymbolication",
                        any(target_os = "macos",
                            all(target_os = "ios", debug_assertions))))] {
        mod coresymbolication;
        use self::coresymbolication::resolve as resolve_imp;
        use self::coresymbolication::Symbol as SymbolImp;
    } else if #[cfg(all(feature = "libbacktrace",
                        any(unix, all(windows, target_env = "gnu")),
                        not(target_os = "fuchsia"),
                        not(target_os = "emscripten")))] {
        mod libbacktrace;
        use self::libbacktrace::resolve as resolve_imp;
        use self::libbacktrace::Symbol as SymbolImp;
    } else if #[cfg(all(unix,
                        not(target_os = "emscripten"),
                        feature = "dladdr"))] {
        mod dladdr_resolve;
        use self::dladdr_resolve::resolve as resolve_imp;
        use self::dladdr_resolve::Symbol as SymbolImp;
    } else {
        mod noop;
        use self::noop::resolve as resolve_imp;
        use self::noop::Symbol as SymbolImp;
    }
}
