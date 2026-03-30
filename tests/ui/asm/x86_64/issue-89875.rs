//@ build-pass
//@ add-minicore
//@ compile-flags: --target x86_64-unknown-linux-gnu
//@ needs-llvm-components: x86
//@ needs-asm-support
#![crate_type = "lib"]
#![feature(no_core)]
#![no_core]

extern crate minicore;
use minicore::*;

#[unsafe(no_mangle)]
#[target_feature(enable = "avx")]
fn foo() {
    unsafe {
        asm!(
            "/* {} */",
            out(ymm_reg) _,
        );
    }
}
<<<<<<< HEAD

fn main() {}

// ferrocene-annotations: fls_spdmit5fy7el
// Attribute target_feature
||||||| 6bdc342ddb0

fn main() {}
=======
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
