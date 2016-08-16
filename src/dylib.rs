#![macro_use]

use std::ffi::CString;
use libc::{dlopen, dlsym, dlclose, c_void};


/// Simple implementation of dynamic library loading.  We're using
/// this for loading debug support on OS X at runtime since we're
/// using a private framework there which might not be there or
/// disappear.  See `symbolize/coresymbolication` for how this is used.
pub struct Dylib {
    handle: *mut c_void,
}

unsafe impl Sync for Dylib {}

impl Dylib {
    pub fn open(path: &str) -> Dylib {
        let path = CString::new(path).unwrap();
        unsafe {
            Dylib {
                handle: dlopen(path.as_ptr() as *const _, 1)
            }
        }
    }

    pub fn is_available(&self) -> bool {
        !self.handle.is_null()
    }

    pub unsafe fn load_symbol(&self, sym: &str) -> *mut c_void {
        let name = CString::new(sym).unwrap();
        dlsym(self.handle, name.as_ptr() as *const _)
    }
}

impl Drop for Dylib {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                dlclose(self.handle);
            }
        }
    }
}


macro_rules! load_dynamically {
    (@as_item $i:item) => { $i };
    (
        #[link=$lib:tt]
        extern $cconv:tt as $libname:ident {
            $(
                fn $funcname:ident($($argnames:ident: $argtypes:ty),*)
                    -> $rv:ty;
            )*
        }
    ) => {
        lazy_static! {
            static ref $libname: ::dylib::Dylib = ::dylib::Dylib::open($lib);
        }

        $(
            load_dynamically! {
                @as_item
                #[allow(non_snake_case)]
                unsafe fn $funcname($($argnames: $argtypes),*) -> $rv {
                    #![allow(dead_code)]
                    lazy_static! {
                        static ref FN: unsafe extern $cconv fn($($argtypes),*) -> $rv = {
                            unsafe {
                                if !$libname.is_available() {
                                    panic!("Library {} is not available", $lib);
                                }
                                let ptr = $libname.load_symbol(stringify!($funcname));
                                if ptr.is_null() {
                                    panic!("Symbol {} not found in {}",
                                           stringify!($funcname), $lib);
                                }
                                ::std::mem::transmute(ptr)
                            }
                        };
                    }
                    (FN)($($argnames),*)
                }
            }
        )*
    };
}
