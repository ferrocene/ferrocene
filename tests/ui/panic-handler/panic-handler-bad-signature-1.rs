//@ compile-flags:-C panic=abort

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: PanicInfo) -> () {}
<<<<<<< HEAD
//~^ `#[panic_handler]` function has wrong type [E0308]

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: um_rustc_C_panic
=======
//~^ ERROR `#[panic_handler]` function has wrong type [E0308]
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
