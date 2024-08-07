use super::unsupported;
use crate::error::Error as StdError;
use crate::ffi::{OsStr, OsString};
use crate::marker::PhantomData;
use crate::os::xous::ffi::Error as XousError;
use crate::path::{self, PathBuf};
use crate::{fmt, io};

#[cfg(not(test))]
#[cfg(feature = "panic_unwind")]
mod eh_unwinding {
    pub(crate) struct EhFrameFinder(usize /* eh_frame */);
    pub(crate) static mut EH_FRAME_SETTINGS: EhFrameFinder = EhFrameFinder(0);
    impl EhFrameFinder {
        pub(crate) unsafe fn init(&mut self, eh_frame: usize) {
            unsafe {
                EH_FRAME_SETTINGS.0 = eh_frame;
            }
        }
    }
    unsafe impl unwind::EhFrameFinder for EhFrameFinder {
        fn find(&self, _pc: usize) -> Option<unwind::FrameInfo> {
            Some(unwind::FrameInfo {
                text_base: None,
                kind: unwind::FrameInfoKind::EhFrame(self.0),
            })
        }
    }
}

#[cfg(not(test))]
mod c_compat {
    use crate::os::xous::ffi::exit;
    extern "C" {
        fn main() -> u32;
    }

    #[no_mangle]
    pub extern "C" fn abort() {
        exit(1);
    }

    #[no_mangle]
    pub extern "C" fn _start(eh_frame: usize) {
        #[cfg(feature = "panic_unwind")]
        unsafe {
            super::eh_unwinding::EH_FRAME_SETTINGS.init(eh_frame);
            unwind::set_custom_eh_frame_finder(&super::eh_unwinding::EH_FRAME_SETTINGS).ok();
        }
        exit(unsafe { main() });
    }

    // This function is needed by the panic runtime. The symbol is named in
    // pre-link args for the target specification, so keep that in sync.
    #[no_mangle]
    // NB. used by both libunwind and libpanic_abort
    pub extern "C" fn __rust_abort() -> ! {
        exit(101);
    }
}

pub fn errno() -> i32 {
    0
}

pub fn error_string(errno: i32) -> String {
    Into::<XousError>::into(errno).to_string()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(!, PhantomData<&'a ()>);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on this platform yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        "not supported on this platform yet"
    }
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub struct Env(!);

impl Env {
    // FIXME(https://github.com/rust-lang/rust/issues/114583): Remove this when <OsStr as Debug>::fmt matches <str as Debug>::fmt.
    pub fn str_debug(&self) -> impl fmt::Debug + '_ {
        let Self(inner) = self;
        match *inner {}
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        match *inner {}
    }
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        self.0
    }
}

pub fn env() -> Env {
    panic!("not supported on this platform")
}

pub fn getenv(_: &OsStr) -> Option<OsString> {
    None
}

pub unsafe fn setenv(_: &OsStr, _: &OsStr) -> io::Result<()> {
    Err(io::const_io_error!(io::ErrorKind::Unsupported, "cannot set env vars on this platform"))
}

pub unsafe fn unsetenv(_: &OsStr) -> io::Result<()> {
    Err(io::const_io_error!(io::ErrorKind::Unsupported, "cannot unset env vars on this platform"))
}

pub fn temp_dir() -> PathBuf {
    panic!("no filesystem on this platform")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(code: i32) -> ! {
    crate::os::xous::ffi::exit(code as u32);
}

pub fn getpid() -> u32 {
    panic!("no pids on this platform")
}
