extern crate libc;

#[macro_use]
mod macros;

pub use backtrace::{trace, Context, Callback};
mod backtrace;

pub use symbolize::{Symbol, resolve};
mod symbolize;

struct Bomb {
    enabled: bool,
}

impl Drop for Bomb {
    fn drop(&mut self) {
        if self.enabled {
            panic!("cannot panic during the backtrace function");
        }
    }
}
