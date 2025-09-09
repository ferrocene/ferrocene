//@ build-pass
//@ needs-asm-support
//@ only-x86_64

use std::arch::asm;

#[target_feature(enable = "avx")]
fn foo() {
    unsafe {
        asm!(
            "/* {} */",
            out(ymm_reg) _,
        );
    }
}

fn main() {}

// ferrocene-annotations: fls_spdmit5fy7el
// Attribute target_feature
