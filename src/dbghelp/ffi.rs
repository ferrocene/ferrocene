//! A module to define the FFI definitions we use on Windows for `dbghelp.dll`
//!
//! This module uses a custom macro, `ffi!`, to wrap all definitions to
//! automatically generate tests to assert that our definitions here are the
//! same as `winapi`.
//!
//! This module largely exists to integrate into libstd itself where winapi is
//! not currently available.

#![allow(bad_style)]

use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::um::winnt::*;

macro_rules! ffi {
	() => ();

    (#[repr(C)] pub struct $name:ident { $(pub $field:ident: $ty:ty,)* } $($rest:tt)*) => (
        #[repr(C)]
        #[cfg(not(feature = "verify-winapi"))]
        pub struct $name {
            $(pub $field: $ty,)*
        }

        #[cfg(feature = "verify-winapi")]
        pub use winapi::um::dbghelp::$name;

        #[test]
        #[cfg(feature = "verify-winapi")]
        fn $name() {
            use core::mem;

            #[repr(C)]
            pub struct $name {
                $(pub $field: $ty,)*
            }

            assert_eq!(
                mem::size_of::<$name>(),
                mem::size_of::<winapi::um::dbghelp::$name>(),
            );
            assert_eq!(
                mem::align_of::<$name>(),
                mem::align_of::<winapi::um::dbghelp::$name>(),
            );

            type Winapi = winapi::um::dbghelp::$name;

            fn assert_same<T>(_: T, _: T) {}

            unsafe {
                let a = &*(mem::align_of::<$name>() as *const $name);
                let b = &*(mem::align_of::<Winapi>() as *const Winapi);

                $(
                    assert_same(&a.$field, &b.$field);
                    assert_eq!(&a.$field as *const $ty, &b.$field as *const $ty,
                               concat!("misplaced field ", stringify!($field)));
                )*
            }
        }

        ffi!($($rest)*);
    );

    (pub type $name:ident = $ty:ty; $($rest:tt)*) => (
        pub type $name = $ty;

        #[cfg(feature = "verify-winapi")]
        #[allow(dead_code)]
        const $name: () = {
            fn _foo() {
                trait SameType {}
                impl<T> SameType for (T, T) {}
                fn assert_same<T: SameType>() {}

                assert_same::<($name, winapi::um::dbghelp::$name)>();
            }
        };

        ffi!($($rest)*);
    );

    (pub const $name:ident: $ty:ty = $val:expr; $($rest:tt)*) => (
        pub const $name: $ty = $val;

        #[cfg(feature = "verify-winapi")]
        #[allow(unused_imports)]
        mod $name {
            use super::*;
            #[test]
            fn assert_valid() {
                let x: $ty = winapi::um::dbghelp::$name;
                assert_eq!(x, $val);
            }
        }


        ffi!($($rest)*);
    );
}


ffi! {
	#[repr(C)]
	pub struct STACKFRAME64 {
		pub AddrPC: ADDRESS64,
		pub AddrReturn: ADDRESS64,
		pub AddrFrame: ADDRESS64,
		pub AddrStack: ADDRESS64,
		pub AddrBStore: ADDRESS64,
		pub FuncTableEntry: PVOID,
		pub Params: [DWORD64; 4],
		pub Far: BOOL,
		pub Virtual: BOOL,
		pub Reserved: [DWORD64; 3],
		pub KdHelp: KDHELP64,
	}

	pub type LPSTACKFRAME64 = *mut STACKFRAME64;

	#[repr(C)]
	pub struct IMAGEHLP_LINEW64 {
		pub SizeOfStruct: DWORD,
		pub Key: PVOID,
		pub LineNumber: DWORD,
		pub FileName: PWSTR,
		pub Address: DWORD64,
	}

	pub type PIMAGEHLP_LINEW64 = *mut IMAGEHLP_LINEW64;

	#[repr(C)]
	pub struct SYMBOL_INFOW {
		pub SizeOfStruct: ULONG,
		pub TypeIndex: ULONG,
		pub Reserved: [ULONG64; 2],
		pub Index: ULONG,
		pub Size: ULONG,
		pub ModBase: ULONG64,
		pub Flags: ULONG,
		pub Value: ULONG64,
		pub Address: ULONG64,
		pub Register: ULONG,
		pub Scope: ULONG,
		pub Tag: ULONG,
		pub NameLen: ULONG,
		pub MaxNameLen: ULONG,
		pub Name: [WCHAR; 1],
	}

	pub type PSYMBOL_INFOW = *mut SYMBOL_INFOW;

	pub type PTRANSLATE_ADDRESS_ROUTINE64 = Option<
		unsafe extern "system" fn(hProcess: HANDLE, hThread: HANDLE, lpaddr: LPADDRESS64) -> DWORD64,
	>;
	pub type PGET_MODULE_BASE_ROUTINE64 =
		Option<unsafe extern "system" fn(hProcess: HANDLE, Address: DWORD64) -> DWORD64>;
	pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 =
		Option<unsafe extern "system" fn(ahProcess: HANDLE, AddrBase: DWORD64) -> PVOID>;
	pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option<
		unsafe extern "system" fn(
			hProcess: HANDLE,
			qwBaseAddress: DWORD64,
			lpBuffer: PVOID,
			nSize: DWORD,
			lpNumberOfBytesRead: LPDWORD,
		) -> BOOL,
	>;

	#[repr(C)]
	pub struct ADDRESS64 {
		pub Offset: DWORD64,
		pub Segment: WORD,
		pub Mode: ADDRESS_MODE,
	}

	pub type LPADDRESS64 = *mut ADDRESS64;

	pub type ADDRESS_MODE = u32;

	#[repr(C)]
	pub struct KDHELP64 {
		pub Thread: DWORD64,
		pub ThCallbackStack: DWORD,
		pub ThCallbackBStore: DWORD,
		pub NextCallback: DWORD,
		pub FramePointer: DWORD,
		pub KiCallUserMode: DWORD64,
		pub KeUserCallbackDispatcher: DWORD64,
		pub SystemRangeStart: DWORD64,
		pub KiUserExceptionDispatcher: DWORD64,
		pub StackBase: DWORD64,
		pub StackLimit: DWORD64,
		pub BuildVersion: DWORD,
		pub Reserved0: DWORD,
		pub Reserved1: [DWORD64; 4],
	}

	pub const MAX_SYM_NAME: usize = 2000;

	pub const AddrModeFlat: ADDRESS_MODE = 3;
}
