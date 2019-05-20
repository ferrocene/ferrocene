//! A module to assist in managing dbghelp bindings on Windows
//!
//! Backtraces on Windows (at least for MSVC) are largely powered through
//! `dbghelp.dll` and the various functions that it contains. These functions
//! are currently loaded *dynamically* rather than linking to `dbghelp.dll`
//! statically. This is currently done by the standard library (and is in theory
//! required there), but is an effort to help reduce the static dll dependencies
//! of a library since backtraces are typically pretty optional. That being
//! said, `dbghelp.dll` almost always successfully loads on Windows.
//!
//! Note though that since we're loading all this support dynamically we can't
//! actually use the raw definitions in `winapi`, but rather we need to define
//! the function pointer types ourselves and use that. We don't really want to
//! be in the business of duplicating winapi, so we have a Cargo feature
//! `verify-winapi` which asserts that all bindings match those in winapi and
//! this feature is enabled on CI.

#![allow(non_snake_case)]

use core::mem;
use core::ptr;
use windows::*;

// Work around `SymGetOptions` and `SymSetOptions` not being present in winapi
// itself. Otherwise this is only used when we're double-checking types against
// winapi.
#[cfg(feature = "verify-winapi")]
mod dbghelp {
    pub use winapi::um::dbghelp::{
        StackWalk64, SymCleanup, SymFromAddrW, SymFunctionTableAccess64, SymGetLineFromAddrW64,
        SymGetModuleBase64, SymInitializeW,
    };
    use windows::*;

    extern "system" {
        // Not defined in winapi yet
        pub fn SymGetOptions() -> u32;
        pub fn SymSetOptions(_: u32);

        // This is defined in winapi, but it's incorrect (FIXME winapi-rs#768)
        pub fn StackWalkEx(
            MachineType: DWORD,
            hProcess: HANDLE,
            hThread: HANDLE,
            StackFrame: LPSTACKFRAME_EX,
            ContextRecord: PVOID,
            ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE64,
            FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE64,
            GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE64,
            TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE64,
            Flags: DWORD,
        ) -> BOOL;

        // Not defined in winapi yet
        pub fn SymFromInlineContextW(
            hProcess: HANDLE,
            Address: DWORD64,
            InlineContext: ULONG,
            Displacement: PDWORD64,
            Symbol: PSYMBOL_INFOW,
        ) -> BOOL;
        pub fn SymGetLineFromInlineContextW(
            hProcess: HANDLE,
            dwAddr: DWORD64,
            InlineContext: ULONG,
            qwModuleBaseAddress: DWORD64,
            pdwDisplacement: PDWORD,
            Line: PIMAGEHLP_LINEW64,
        ) -> BOOL;
    }

    pub fn assert_equal_types<T>(a: T, _b: T) -> T {
        a
    }
}

// This macro is used to define a `Dbghelp` structure which internally contains
// all the function pointers that we might load.
macro_rules! dbghelp {
    (extern "system" {
        $(fn $name:ident($($arg:ident: $argty:ty),*) -> $ret: ty;)*
    }) => (
        pub struct Dbghelp {
            /// The loaded DLL for `dbghelp.dll`
            dll: HMODULE,

            // Each function pointer for each function we might use
            $($name: usize,)*
        }

        static mut DBGHELP: Dbghelp = Dbghelp {
            // Initially we haven't loaded the DLL
            dll: 0 as *mut _,
            // Initiall all functions are set to zero to say they need to be
            // dynamically loaded.
            $($name: 0,)*
        };

        // Convenience typedef for each function type.
        $(pub type $name = unsafe extern "system" fn($($argty),*) -> $ret;)*

        impl Dbghelp {
            /// Attempts to open `dbghelp.dll`. Returns success if it works or
            /// error if `LoadLibraryW` fails.
            ///
            /// Panics if library is already loaded.
            fn open(&mut self) -> Result<(), ()> {
                assert!(self.dll.is_null());
                let lib = b"dbghelp.dll\0";
                unsafe {
                    self.dll = LoadLibraryA(lib.as_ptr() as *const i8);
                    if self.dll.is_null() {
                        Err(())
                    }  else {
                        Ok(())
                    }
                }
            }

            /// Unloads `dbghelp.dll`, resetting all function pointers to zero
            /// as well.
            fn close(&mut self) {
                assert!(!self.dll.is_null());
                unsafe {
                    $(self.$name = 0;)*
                    FreeLibrary(self.dll);
                    self.dll = ptr::null_mut();
                }
            }

            // Function for each method we'd like to use. When called it will
            // either read the cached function pointer or load it and return the
            // loaded value. Loads are asserted to succeed.
            $(pub fn $name(&mut self) -> Option<$name> {
                unsafe {
                    if self.$name == 0 {
                        let name = concat!(stringify!($name), "\0");
                        self.$name = self.symbol(name.as_bytes())?;
                    }
                    let ret = mem::transmute::<usize, $name>(self.$name);
                    #[cfg(feature = "verify-winapi")]
                    dbghelp::assert_equal_types(ret, dbghelp::$name);
                    Some(ret)
                }
            })*

            fn symbol(&self, symbol: &[u8]) -> Option<usize> {
                unsafe {
                    match GetProcAddress(self.dll, symbol.as_ptr() as *const _) as usize {
                        0 => None,
                        n => Some(n),
                    }
                }
            }
        }

        // Convenience proxy to use the cleanup locks to reference dbghelp
        // functions.
        #[allow(dead_code)]
        impl Cleanup {
            $(pub fn $name(&self) -> $name {
                unsafe {
                    DBGHELP.$name().unwrap()
                }
            })*

            pub fn dbghelp(&self) -> *mut Dbghelp {
                unsafe {
                    &mut DBGHELP
                }
            }
        }
    )

}

const SYMOPT_DEFERRED_LOADS: DWORD = 0x00000004;

dbghelp! {
    extern "system" {
        fn SymGetOptions() -> DWORD;
        fn SymSetOptions(options: DWORD) -> ();
        fn SymInitializeW(
            handle: HANDLE,
            path: PCWSTR,
            invade: BOOL
        ) -> BOOL;
        fn SymCleanup(handle: HANDLE) -> BOOL;
        fn StackWalk64(
            MachineType: DWORD,
            hProcess: HANDLE,
            hThread: HANDLE,
            StackFrame: LPSTACKFRAME64,
            ContextRecord: PVOID,
            ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE64,
            FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE64,
            GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE64,
            TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE64
        ) -> BOOL;
        fn SymFunctionTableAccess64(
            hProcess: HANDLE,
            AddrBase: DWORD64
        ) -> PVOID;
        fn SymGetModuleBase64(
            hProcess: HANDLE,
            AddrBase: DWORD64
        ) -> DWORD64;
        fn SymFromAddrW(
            hProcess: HANDLE,
            Address: DWORD64,
            Displacement: PDWORD64,
            Symbol: PSYMBOL_INFOW
        ) -> BOOL;
        fn SymGetLineFromAddrW64(
            hProcess: HANDLE,
            dwAddr: DWORD64,
            pdwDisplacement: PDWORD,
            Line: PIMAGEHLP_LINEW64
        ) -> BOOL;
        fn StackWalkEx(
            MachineType: DWORD,
            hProcess: HANDLE,
            hThread: HANDLE,
            StackFrame: LPSTACKFRAME_EX,
            ContextRecord: PVOID,
            ReadMemoryRoutine: PREAD_PROCESS_MEMORY_ROUTINE64,
            FunctionTableAccessRoutine: PFUNCTION_TABLE_ACCESS_ROUTINE64,
            GetModuleBaseRoutine: PGET_MODULE_BASE_ROUTINE64,
            TranslateAddress: PTRANSLATE_ADDRESS_ROUTINE64,
            Flags: DWORD
        ) -> BOOL;
        fn SymFromInlineContextW(
            hProcess: HANDLE,
            Address: DWORD64,
            InlineContext: ULONG,
            Displacement: PDWORD64,
            Symbol: PSYMBOL_INFOW
        ) -> BOOL;
        fn SymGetLineFromInlineContextW(
            hProcess: HANDLE,
            dwAddr: DWORD64,
            InlineContext: ULONG,
            qwModuleBaseAddress: DWORD64,
            pdwDisplacement: PDWORD,
            Line: PIMAGEHLP_LINEW64
        ) -> BOOL;
    }
}

pub struct Cleanup;

// Number of times `init` has been called on this thread. This is externally
// synchronized and doesn't use internal synchronization on our behalf.
static mut COUNT: usize = 0;

// Used to restore `SymSetOptions` and `SymGetOptions` values.
static mut OPTS_ORIG: DWORD = 0;

/// Unsafe because this requires external synchronization, must be done
/// inside of the same lock as all other backtrace operations.
///
/// Note that the `Dbghelp` returned must also be dropped within the same
/// lock.
#[cfg(all(windows, feature = "dbghelp"))]
pub unsafe fn init() -> Result<Cleanup, ()> {
    // Initializing symbols has significant overhead, but initializing only
    // once without cleanup causes problems for external sources. For
    // example, the standard library checks the result of SymInitializeW
    // (which returns an error if attempting to initialize twice) and in
    // the event of an error, will not print a backtrace on panic.
    // Presumably, external debuggers may have similar issues.
    //
    // As a compromise, we'll keep track of the number of internal
    // initialization requests within a single API call in order to
    // minimize the number of init/cleanup cycles.
    if COUNT > 0 {
        COUNT += 1;
        return Ok(Cleanup);
    }

    // Actually load `dbghelp.dll` into the process here, returning an error if
    // that fails.
    DBGHELP.open()?;

    OPTS_ORIG = DBGHELP.SymGetOptions().unwrap()();

    // Ensure that the `SYMOPT_DEFERRED_LOADS` flag is set, because
    // according to MSVC's own docs about this: "This is the fastest, most
    // efficient way to use the symbol handler.", so let's do that!
    DBGHELP.SymSetOptions().unwrap()(OPTS_ORIG | SYMOPT_DEFERRED_LOADS);

    let ret = DBGHELP.SymInitializeW().unwrap()(GetCurrentProcess(), ptr::null_mut(), TRUE);
    if ret != TRUE {
        // Symbols may have been initialized by another library or an
        // external debugger
        DBGHELP.SymSetOptions().unwrap()(OPTS_ORIG);
        DBGHELP.close();
        Err(())
    } else {
        COUNT += 1;
        Ok(Cleanup)
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        unsafe {
            COUNT -= 1;
            if COUNT != 0 {
                return;
            }

            // Clean up after ourselves by cleaning up symbols and restoring the
            // symbol options to their original value. This is currently
            // required to cooperate with libstd as libstd's backtracing will
            // assert symbol initialization succeeds and will clean up after the
            // backtrace is finished.
            DBGHELP.SymCleanup().unwrap()(GetCurrentProcess());
            DBGHELP.SymSetOptions().unwrap()(OPTS_ORIG);

            // We can in theory leak this to stay in a global and we simply
            // always reuse it, but for now let's be tidy and release all our
            // resources. If we get bug reports the we could basically elide
            // this `close()` (and the one above) and then update `open` to be a
            // noop if it's already opened.
            DBGHELP.close();
        }
    }
}
