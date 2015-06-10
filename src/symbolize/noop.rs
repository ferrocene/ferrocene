use libc::c_void;

use Symbol;

struct Dummy;

impl Symbol for Dummy {
    fn name(&self) -> Option<&[u8]> { None }
    fn addr(&self) -> Option<*mut c_void> { None }
}

pub fn resolve(_addr: *mut c_void, cb: &mut FnMut(&Symbol)) {
    cb(&Dummy)
}

