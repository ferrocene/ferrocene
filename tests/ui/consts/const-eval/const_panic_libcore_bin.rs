#![crate_type = "bin"]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

const Z: () = panic!("cheese");
//~^ ERROR evaluation panicked

const Y: () = unreachable!();
//~^ ERROR evaluation panicked

const X: () = unimplemented!();
//~^ ERROR evaluation panicked

#[lang = "eh_personality"]
fn eh() {}
#[lang = "eh_catch_typeinfo"]
static EH_CATCH_TYPEINFO: u8 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
