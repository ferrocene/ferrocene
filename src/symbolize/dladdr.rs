use libc::{c_void, c_char, c_int};
use std::mem;
use std::ffi::CStr;

use Symbol;

#[repr(C)]
struct Dl_info {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

impl<'a> Symbol for Option<&'a Dl_info> {
    fn name(&self) -> Option<&[u8]> {
        self.and_then(|p| {
            if p.dli_sname.is_null() {None} else {Some(p.dli_sname)}
        }).map(|p| unsafe {
            CStr::from_ptr(p).to_bytes()
        })
    }
    fn addr(&self) -> Option<*mut c_void> {
        self.map(|p| p.dli_saddr)
    }
}

extern {
    fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int;
}

pub fn resolve(addr: *mut c_void, cb: &mut FnMut(&Symbol)) {
    let mut info: Dl_info = unsafe { mem::zeroed() };
    if unsafe { dladdr(addr, &mut info) == 0 } {
        cb(&None::<&Dl_info>)
    } else {
        cb(&Some(&info))
    }
}
