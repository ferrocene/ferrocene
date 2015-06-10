use libc::c_void;

pub trait Symbol {
    fn name(&self) -> Option<&[u8]> { None }
    fn addr(&self) -> Option<*mut c_void> { None }
    fn filename(&self) -> Option<&[u8]> { None }
    fn lineno(&self) -> Option<u32> { None }
}

cascade! {
    if #[cfg(all(feature = "libbacktrace", unix))] {
        mod libbacktrace;
        pub use self::libbacktrace::resolve;
    } else if #[cfg(feature = "dladdr")] {
        mod dladdr;
        pub use self::dladdr::resolve;
    } else {
        mod noop;
        pub use self::noop::resolve;
    }
}

