use libc::c_void;

pub trait Context {
    fn ip(&self) -> *mut c_void;
    fn symbol_address(&self) -> *mut c_void;
}

pub type Callback<'a> = FnMut(&Context) -> bool + 'a;

cascade! {
    if #[cfg(feature = "libunwind")] {
        mod libunwind;
        pub use self::libunwind::trace;
    } else if #[cfg(feature = "unix-backtrace")] {
        mod unix_backtrace;
        pub use self::unix_backtrace::trace;
    } else {
        mod noop;
        pub use self::noop::trace;
    }
}
