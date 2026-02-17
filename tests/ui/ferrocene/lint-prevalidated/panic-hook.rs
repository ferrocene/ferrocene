#![deny(ferrocene::uncertified)]

use std::panic::{PanicHookInfo, set_hook};

fn uncertified_panic_hook(_: &PanicHookInfo) {}
fn main() {
    set_hook(Box::new(uncertified_panic_hook)); //~ ERROR unvalidated
}
