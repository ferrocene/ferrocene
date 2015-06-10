extern crate libc;

#[macro_use]
mod macros;

pub use backtrace::{trace, Frame};
mod backtrace;

pub use symbolize::{Symbol, resolve};
mod symbolize;

pub use demangle::demangle;
mod demangle;

#[allow(dead_code)]
struct Bomb {
    enabled: bool,
}

#[allow(dead_code)]
impl Drop for Bomb {
    fn drop(&mut self) {
        if self.enabled {
            panic!("cannot panic during the backtrace function");
        }
    }
}
