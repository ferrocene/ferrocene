#![deny(ferrocene::unvalidated)]

use std::panic::{PanicHookInfo, set_hook};

fn unvalidated_panic_hook(_: &PanicHookInfo) {}
fn main() {
    set_hook(Box::new(unvalidated_panic_hook)); //~ ERROR unvalidated
}
