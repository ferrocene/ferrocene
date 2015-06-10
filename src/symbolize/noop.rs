use libc::c_void;

use Symbol;

struct Dummy;

impl Symbol for Dummy {}

pub fn resolve(_addr: *mut c_void, cb: &mut FnMut(&Symbol)) {
    cb(&Dummy)
}

