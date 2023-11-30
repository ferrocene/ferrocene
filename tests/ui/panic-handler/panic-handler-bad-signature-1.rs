// compile-flags:-C panic=abort

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(
    info: PanicInfo, //~ ERROR argument should be `&PanicInfo`
) -> () //~ ERROR return type should be `!`
{
}

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: um_rustc_C_panic
