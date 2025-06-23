//@ compile-flags:-C panic=abort

#![feature(lang_items)]
#![feature(no_core)]
#![no_core]
#![no_main]

#[panic_handler]
fn panic() -> ! {
    //~^ ERROR requires `panic_info` lang_item
    loop {}
}

#[lang = "pointee_sized"]
pub trait PointeeSized {}

#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}

#[lang = "sized"]
trait Sized: MetaSized {}

// ferrocene-annotations: fls_fh27ljezn3qz
// Attribute no_main
//
// ferrocene-annotations: um_rustc_C_panic
