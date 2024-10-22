#![stable(feature = "rust1", since = "1.0.0")]

pub mod ffi;
pub mod fs;
pub mod io;
pub mod process;

/// A prelude for conveniently writing platform-specific code.
///
/// Includes all extension traits, and some important type definitions.
#[stable(feature = "rust1", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::fs::DirEntryExt;
    #[doc(no_inline)]
    #[stable(feature = "file_offset", since = "1.15.0")]
    pub use super::fs::FileExt;
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::fs::{FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
    #[doc(no_inline)]
    #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::process::ExitStatusExt;
    // pub use super::process::{CommandExt, ExitStatusExt};
    // #[doc(no_inline)]
    // #[stable(feature = "rust1", since = "1.0.0")]
    // pub use super::thread::JoinHandleExt;
}
