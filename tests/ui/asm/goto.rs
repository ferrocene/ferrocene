//@ run-pass
//@ needs-asm-support

use std::arch::asm;

fn main() {
    let mut value = false;
    unsafe {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64", target_arch = "arm64ec"))]
        asm!(
            "b {}",
            label {
                value = true;
            }
        );
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        asm!(
            "j {}",
            label {
                value = true;
            }
        );
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        asm!(
            "jmp {}",
            label {
                value = true;
            }
        );
    }
    assert!(value);
}

// ferrocene-annotations: fls_MW7mtH5oOeQ1
// Label block
