// revisions: stock mut_refs

#![cfg_attr(mut_refs, feature(const_mut_refs))]

static mut STDERR_BUFFER_SPACE: u8 = 0;

pub static mut STDERR_BUFFER: () = unsafe {
    *(&mut STDERR_BUFFER_SPACE) = 42;
    //[mut_refs]~^ ERROR could not evaluate static initializer
    //[stock]~^^ ERROR mutable references are not allowed in statics
    //[mut_refs]~^^^ WARN creating a mutable reference to mutable static is discouraged [static_mut_refs]
    //[stock]~^^^^ WARN creating a mutable reference to mutable static is discouraged [static_mut_refs]
};

fn main() {}
