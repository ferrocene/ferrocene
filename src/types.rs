//! Platform dependent types.

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::os::raw::c_void;
        use std::borrow::Cow;
        use std::fmt;
        use std::path::PathBuf;
        use std::prelude::v1::*;
    } else {
        pub use core::ffi::c_void;
    }
}

/// A platform independent representation of a string. When working with `std`
/// enabled it is recommended to the convenience methods for providing
/// conversions to `std` types.
#[derive(Debug)]
pub enum BytesOrWideString<'a> {
    /// A slice, typically provided on Unix platforms.
    Bytes(&'a [u8]),
    /// Wide strings typically from Windows.
    Wide(&'a [u16]),
}

#[cfg(feature = "std")]
impl<'a> BytesOrWideString<'a> {
    /// Lossy converts to a `Cow<str>`, will allocate if `Bytes` is not valid
    /// UTF-8 or if `BytesOrWideString` is `Wide`.
    pub fn to_str_lossy(&self) -> Cow<'a, str> {
        use self::BytesOrWideString::*;

        match self {
            Bytes(slice) => String::from_utf8_lossy(slice),
            Wide(wide) => Cow::Owned(String::from_utf16_lossy(wide)),
        }
    }

    /// Provides a `Path` representation of `BytesOrWideString`.
    #[cfg(not(windows))]
    pub fn into_path_buf(self) -> PathBuf {
        use self::BytesOrWideString::*;
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        match self {
            Bytes(slice) => PathBuf::from(OsStr::from_bytes(slice)),
            _ => unreachable!(),
        }
    }

    /// Provides a `Path` representation of `BytesOrWideString`.
    #[cfg(windows)]
    pub fn into_path_buf(self) -> PathBuf {
        use self::BytesOrWideString::*;
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        match self {
            Wide(slice) => PathBuf::from(OsString::from_wide(slice)),
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> fmt::Display for BytesOrWideString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_str_lossy().fmt(f)
    }
}
