//@ only-aarch64
//@ ignore-apple On Mac, we can be sure the CPU is never an a53 (Apple doesn't make those)

// Regression test for rust-lang/rust#118095

use run_make_support::regex::Regex;
use run_make_support::{llvm_objdump, rustc, target};

fn main() {
    let mut compile = rustc();
    compile.input("main.rs").target(target()).print("link-args").panic("abort");

    let linker_type = if target().contains("ferrocene.facade") {
        // This target directly uses LLD as the linker
        compile.link_arg("link.x").arg("-Clink-self-contained=no");
        LinkerType::Lld
    } else if target().contains("qnx") {
        // This target uses GCC as the linker, and needs -nostartup since our program is basically a
        // startup object.
        compile.link_arg("-Wl,-Tlink.x").link_arg("-nostartup");
        LinkerType::Gnu
    } else {
        // This target uses GCC as the linker, and needs -nostartfiles since our program is
        // basically a startup object.
        compile.link_arg("-Wl,link.x").link_arg("-nostartfiles");
        LinkerType::Gnu
    };

    let outcome = compile.run();
    // Ensure --print=link-args shows the errata fix linker flag.
    assert!(outcome.stdout_utf8().contains("--fix-cortex-a53-843419"));

    match linker_type {
        LinkerType::Lld => {
            // Check that the first two required pre-condition instructions (ADRP then LDR) are
            // there.
            assert!(grep_instruction(0xff8, "adrp[[:space:]]+x0"));
            assert!(grep_instruction(0xffc, "ldr[[:space:]]+x1"));
            // Check that the third problematic instruction (LDR) is NOT there. LLD will replace it
            // with a different instruction
            assert!(!grep_instruction(0x1000, "ldr[[:space:]]+x0"));
        }
        LinkerType::Gnu => {
            // Check that the first two required pre-condition instructions (ADRP then LDR) are NOT
            // there. GCC will replace ADRP with ADR.
            assert!(!grep_instruction(0xff8, "adrp[[:space:]]+x0"));
            assert!(grep_instruction(0xff8, "adr[[:space:]]+x0"));
            assert!(grep_instruction(0xffc, "ldr[[:space:]]+x1"));
        }
    }
}

fn grep_instruction(start: usize, regex: &str) -> bool {
    let stop = start + 4;

    let dump = llvm_objdump()
        .input("main")
        .disassemble()
        .arg("--demangle")
        .arg(format!("--start-address={start:#x}"))
        .arg(format!("--stop-address={stop:#x}"))
        .run();

    Regex::new(regex).unwrap().is_match(&dump.stdout_utf8())
}

enum LinkerType {
    Gnu,
    Lld,
}

// Example assembly
//
// - BAD: ADRP @ 0xff8, LDR, LDR
//
// ```
// 0000000000000ff0 <_start>:
//     ff0: d503201f    nop
//     ff4: d503201f    nop
//     ff8: b0000000    adrp    x0, 1000 <_start+0x10>
//     ffc: f9400021    ldr x1, [x1]
//    1000: f940b800    ldr x0, [x0, #368]
//    1004: d65f03c0    ret
// ```
//
// - LLD fix: replace LDR with a B(ranch) instruction
//
// ```
// 0000000000000ff0 <_start>:
//     ff0: d503201f    nop
//     ff4: d503201f    nop
//     ff8: b0000000    adrp    x0, 1000 <_start+0x10>
//     ffc: f9400021    ldr x1, [x1]
//    1000: 14000002    b   1008 <__CortexA53843419_1000>
//    1004: d65f03c0    ret
//
// 0000000000001008 <__CortexA53843419_1000>:
//    1008: f9400800    ldr x0, [x0, #16]
//    100c: 17fffffe    b   1004 <_start+0x14>
//
// ```
//
// - GNU LD fix: replace ADRP with ADR
//
// ```
// 0000000000000ff0 <_start>:
//     ff0: d503201f    nop
//     ff4: d503201f    nop
//     ff8: 10008040    adr x0, 2000 <e843419@0002_00000011_10+0xff0>
//     ffc: f9400021    ldr x1, [x1]
//    1000: f940b800    ldr x0, [x0, #368]
//    1004: d65f03c0    ret
//    1008: 14000400    b   2008 <e843419@0002_00000011_10+0xff8>
//    100c: d503201f    nop
//
// ```
