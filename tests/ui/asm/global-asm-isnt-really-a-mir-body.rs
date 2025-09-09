//@ revisions: emit_mir instrument cfi

//@ compile-flags: -C unsafe-allow-abi-mismatch=sanitizer

// Make sure we don't try to emit MIR for it.
//@[emit_mir] compile-flags: --emit=mir

// Make sure we don't try to instrument it.
//@[instrument] compile-flags: -Cinstrument-coverage -Zno-profiler-runtime
//@[instrument] only-linux

// Make sure we don't try to CFI encode it.
//@[cfi] compile-flags: -Zsanitizer=cfi -Ccodegen-units=1 -Clto -Ctarget-feature=-crt-static -Clink-dead-code=true
//@[cfi] needs-sanitizer-cfi
//@[cfi] no-prefer-dynamic
// FIXME(#122848) Remove only-linux once OSX CFI binaries work
//@[cfi] only-linux

//@ build-pass
//@ needs-asm-support

use std::arch::global_asm;

fn foo() {}

global_asm!("/* {} */", sym foo);

fn main() {}

// Ferrocene addition:
//
// Linker fails with "undefined symbol: __atomic_fetch_add_8" without this stub on these targets:
// - thumbv7em-ferrocene.facade-eabi
// - thumbv7em-ferrocene.facade-eabihf
// This is due to `-C instrument-coverage` resulting in 64-bit atomic operations inserted,
// but those do not exist in Cortex-M processors.
//
// Note that the code is not executed, so a stub is appropriate.
#[cfg(target_arch = "arm")]
#[cfg(not(target_has_atomic = "64"))]
#[unsafe(no_mangle)]
extern "C" fn __atomic_fetch_add_8() {
    // This is added so we will get alerted if this test changes to being executed.
    // The test is only compiled and linked at the moment.
    unreachable!();
}
