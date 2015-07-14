use std::os::raw::c_void;

use Symbol;

pub fn resolve(_addr: *mut c_void, _cb: &mut FnMut(&Symbol)) {
}

