# `asm_experimental_arch`

The tracking issue for this feature is: [#93335]

[#93335]: https://github.com/rust-lang/rust/issues/93335

------------------------

This feature tracks `asm!` and `global_asm!` support for the following architectures:
- NVPTX
- PowerPC
- Hexagon
- MIPS32r2 and MIPS64r2
- wasm32
- BPF
- SPIR-V
- AVR
- MSP430
- M68k
- CSKY
- s390x
- Arm64EC

## Register classes

| Architecture | Register class | Registers                          | LLVM constraint code |
| ------------ | -------------- | ---------------------------------- | -------------------- |
| MIPS         | `reg`          | `$[2-25]`                          | `r`                  |
| MIPS         | `freg`         | `$f[0-31]`                         | `f`                  |
| NVPTX        | `reg16`        | None\*                             | `h`                  |
| NVPTX        | `reg32`        | None\*                             | `r`                  |
| NVPTX        | `reg64`        | None\*                             | `l`                  |
| Hexagon      | `reg`          | `r[0-28]`                          | `r`                  |
| PowerPC      | `reg`          | `r[0-31]`                          | `r`                  |
| PowerPC      | `reg_nonzero`  | `r[1-31]`                          | `b`                  |
| PowerPC      | `freg`         | `f[0-31]`                          | `f`                  |
| PowerPC      | `cr`           | `cr[0-7]`, `cr`                    | Only clobbers        |
| PowerPC      | `xer`          | `xer`                              | Only clobbers        |
| wasm32       | `local`        | None\*                             | `r`                  |
| BPF          | `reg`          | `r[0-10]`                          | `r`                  |
| BPF          | `wreg`         | `w[0-10]`                          | `w`                  |
| AVR          | `reg`          | `r[2-25]`, `XH`, `XL`, `ZH`, `ZL`  | `r`                  |
| AVR          | `reg_upper`    | `r[16-25]`, `XH`, `XL`, `ZH`, `ZL` | `d`                  |
| AVR          | `reg_pair`     | `r3r2` .. `r25r24`, `X`, `Z`       | `r`                  |
| AVR          | `reg_iw`       | `r25r24`, `X`, `Z`                 | `w`                  |
| AVR          | `reg_ptr`      | `X`, `Z`                           | `e`                  |
| MSP430       | `reg`          | `r[0-15]`                          | `r`                  |
| M68k         | `reg`          | `d[0-7]`, `a[0-7]`                 | `r`                  |
| M68k         | `reg_data`     | `d[0-7]`                           | `d`                  |
| M68k         | `reg_addr`     | `a[0-3]`                           | `a`                  |
| CSKY         | `reg`          | `r[0-31]`                          | `r`                  |
| CSKY         | `freg`         | `f[0-31]`                          | `f`                  |
| s390x        | `reg`          | `r[0-10]`, `r[12-14]`              | `r`                  |
| s390x        | `reg_addr`     | `r[1-10]`, `r[12-14]`              | `a`                  |
| s390x        | `freg`         | `f[0-15]`                          | `f`                  |
| s390x        | `vreg`         | `v[0-31]`                          | Only clobbers        |
| s390x        | `areg`         | `a[2-15]`                          | Only clobbers        |
| Arm64EC      | `reg`          | `x[0-12]`, `x[15-22]`, `x[25-27]`, `x30` | `r`            |
| Arm64EC      | `vreg`         | `v[0-15]`                          | `w`                  |
| Arm64EC      | `vreg_low16`   | `v[0-15]`                          | `x`                  |

> **Notes**:
> - NVPTX doesn't have a fixed register set, so named registers are not supported.
>
> - WebAssembly doesn't have registers, so named registers are not supported.

# Register class supported types

| Architecture | Register class                  | Target feature | Allowed types                           |
| ------------ | ------------------------------- | -------------- | --------------------------------------- |
| MIPS32       | `reg`                           | None           | `i8`, `i16`, `i32`, `f32`               |
| MIPS32       | `freg`                          | None           | `f32`, `f64`                            |
| MIPS64       | `reg`                           | None           | `i8`, `i16`, `i32`, `i64`, `f32`, `f64` |
| MIPS64       | `freg`                          | None           | `f32`, `f64`                            |
| NVPTX        | `reg16`                         | None           | `i8`, `i16`                             |
| NVPTX        | `reg32`                         | None           | `i8`, `i16`, `i32`, `f32`               |
| NVPTX        | `reg64`                         | None           | `i8`, `i16`, `i32`, `f32`, `i64`, `f64` |
| Hexagon      | `reg`                           | None           | `i8`, `i16`, `i32`, `f32`               |
| PowerPC      | `reg`                           | None           | `i8`, `i16`, `i32`                      |
| PowerPC      | `reg_nonzero`                   | None           | `i8`, `i16`, `i32`                      |
| PowerPC      | `freg`                          | None           | `f32`, `f64`                            |
| PowerPC      | `cr`                            | N/A            | Only clobbers                           |
| PowerPC      | `xer`                           | N/A            | Only clobbers                           |
| wasm32       | `local`                         | None           | `i8` `i16` `i32` `i64` `f32` `f64`      |
| BPF          | `reg`                           | None           | `i8` `i16` `i32` `i64`                  |
| BPF          | `wreg`                          | `alu32`        | `i8` `i16` `i32`                        |
| AVR          | `reg`, `reg_upper`              | None           | `i8`                                    |
| AVR          | `reg_pair`, `reg_iw`, `reg_ptr` | None           | `i16`                                   |
| MSP430       | `reg`                           | None           | `i8`, `i16`                             |
| M68k         | `reg`, `reg_addr`               | None           | `i16`, `i32`                            |
| M68k         | `reg_data`                      | None           | `i8`, `i16`, `i32`                      |
| CSKY         | `reg`                           | None           | `i8`, `i16`, `i32`                      |
| CSKY         | `freg`                          | None           | `f32`,                                  |
| s390x        | `reg`, `reg_addr`               | None           | `i8`, `i16`, `i32`, `i64`               |
| s390x        | `freg`                          | None           | `f32`, `f64`                            |
| s390x        | `vreg`                          | N/A            | Only clobbers                           |
| s390x        | `areg`                          | N/A            | Only clobbers                           |
| Arm64EC      | `reg`                           | None           | `i8`, `i16`, `i32`, `f32`, `i64`, `f64` |
| Arm64EC      | `vreg`                          | None           | `i8`, `i16`, `i32`, `f32`, `i64`, `f64`, <br> `i8x8`, `i16x4`, `i32x2`, `i64x1`, `f32x2`, `f64x1`, <br> `i8x16`, `i16x8`, `i32x4`, `i64x2`, `f32x4`, `f64x2` |

## Register aliases

| Architecture | Base register | Aliases   |
| ------------ | ------------- | --------- |
| Hexagon      | `r29`         | `sp`      |
| Hexagon      | `r30`         | `fr`      |
| Hexagon      | `r31`         | `lr`      |
| BPF          | `r[0-10]`     | `w[0-10]` |
| AVR          | `XH`          | `r27`     |
| AVR          | `XL`          | `r26`     |
| AVR          | `ZH`          | `r31`     |
| AVR          | `ZL`          | `r30`     |
| MSP430       | `r0`          | `pc`      |
| MSP430       | `r1`          | `sp`      |
| MSP430       | `r2`          | `sr`      |
| MSP430       | `r3`          | `cg`      |
| MSP430       | `r4`          | `fp`      |
| M68k         | `a5`          | `bp`      |
| M68k         | `a6`          | `fp`      |
| M68k         | `a7`          | `sp`, `usp`, `ssp`, `isp` |
| CSKY         | `r[0-3]`      | `a[0-3]`  |
| CSKY         | `r[4-11]`     | `l[0-7]`  |
| CSKY         | `r[12-13]`    | `t[0-1]`  |
| CSKY         | `r14`         | `sp`      |
| CSKY         | `r15`         | `lr`      |
| CSKY         | `r[16-17]`    | `l[8-9]`  |
| CSKY         | `r[18-25]`    | `t[2-9]`  |
| CSKY         | `r28`         | `rgb`     |
| CSKY         | `r29`         | `rtb`     |
| CSKY         | `r30`         | `svbr`    |
| CSKY         | `r31`         | `tls`     |
| Arm64EC      | `x[0-30]`     | `w[0-30]` |
| Arm64EC      | `x29`         | `fp`      |
| Arm64EC      | `x30`         | `lr`      |
| Arm64EC      | `sp`          | `wsp`     |
| Arm64EC      | `xzr`         | `wzr`     |
| Arm64EC      | `v[0-15]`     | `b[0-15]`, `h[0-15]`, `s[0-15]`, `d[0-15]`, `q[0-15]` |

> **Notes**:
> - TI does not mandate a frame pointer for MSP430, but toolchains are allowed
    to use one; LLVM uses `r4`.

## Unsupported registers

| Architecture | Unsupported register                    | Reason                                                                                                                                                                              |
| ------------ | --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| All          | `sp`, `r15` (s390x)                     | The stack pointer must be restored to its original value at the end of an asm code block.                                                                                           |
| All          | `fr` (Hexagon), `$fp` (MIPS), `Y` (AVR), `r4` (MSP430), `a6` (M68k), `r11` (s390x), `x29` (Arm64EC) | The frame pointer cannot be used as an input or output.                                                                                                                             |
| All          | `r19` (Hexagon), `x19` (Arm64EC)        | This is used internally by LLVM as a "base pointer" for functions with complex stack frames.                                                                                        |
| MIPS         | `$0` or `$zero`                         | This is a constant zero register which can't be modified.                                                                                                                           |
| MIPS         | `$1` or `$at`                           | Reserved for assembler.                                                                                                                                                             |
| MIPS         | `$26`/`$k0`, `$27`/`$k1`                | OS-reserved registers.                                                                                                                                                              |
| MIPS         | `$28`/`$gp`                             | Global pointer cannot be used as inputs or outputs.                                                                                                                                 |
| MIPS         | `$ra`                                   | Return address cannot be used as inputs or outputs.                                                                                                                                 |
| Hexagon      | `lr`                                    | This is the link register which cannot be used as an input or output.                                                                                                               |
| AVR          | `r0`, `r1`, `r1r0`                      | Due to an issue in LLVM, the `r0` and `r1` registers cannot be used as inputs or outputs.  If modified, they must be restored to their original values before the end of the block. |
|MSP430        | `r0`, `r2`, `r3`                        | These are the program counter, status register, and constant generator respectively. Neither the status register nor constant generator can be written to.                          |
| M68k         | `a4`, `a5`                              | Used internally by LLVM for the base pointer and global base pointer. |
| CSKY         | `r7`, `r28`                             | Used internally by LLVM for the base pointer and global base pointer. |
| CSKY         | `r8`                                    | Used internally by LLVM for the frame pointer. |
| CSKY         | `r14`                                   | Used internally by LLVM for the stack pointer. |
| CSKY         | `r15`                                   | This is the link register. |
| CSKY         | `r[26-30]`                              | Reserved by its ABI.       |
| CSKY         | `r31`                                   | This is the TLS register.  |
| s390x        | `c[0-15]`                               | Reserved by the kernel. |
| s390x        | `a[0-1]`                                | Reserved for system use. |
| Arm64EC      | `xzr`                                   | This is a constant zero register which can't be modified. |
| Arm64EC      | `x18`                                   | This is an OS-reserved register. |
| Arm64EC      | `x13`, `x14`, `x23`, `x24`, `x28`, `v[16-31]` | These are AArch64 registers that are not supported for Arm64EC. |


## Template modifiers

| Architecture | Register class | Modifier | Example output | LLVM modifier |
| ------------ | -------------- | -------- | -------------- | ------------- |
| MIPS         | `reg`          | None     | `$2`           | None          |
| MIPS         | `freg`         | None     | `$f0`          | None          |
| NVPTX        | `reg16`        | None     | `rs0`          | None          |
| NVPTX        | `reg32`        | None     | `r0`           | None          |
| NVPTX        | `reg64`        | None     | `rd0`          | None          |
| Hexagon      | `reg`          | None     | `r0`           | None          |
| PowerPC      | `reg`          | None     | `0`            | None          |
| PowerPC      | `reg_nonzero`  | None     | `3`            | None          |
| PowerPC      | `freg`         | None     | `0`            | None          |
| s390x        | `reg`          | None     | `%r0`          | None          |
| s390x        | `reg_addr`     | None     | `%r1`          | None          |
| s390x        | `freg`         | None     | `%f0`          | None          |
| CSKY         | `reg`          | None     | `r0`           | None          |
| CSKY         | `freg`         | None     | `f0`           | None          |
| Arm64EC      | `reg`          | None     | `x0`           | `x`           |
| Arm64EC      | `reg`          | `w`      | `w0`           | `w`           |
| Arm64EC      | `reg`          | `x`      | `x0`           | `x`           |
| Arm64EC      | `vreg`         | None     | `v0`           | None          |
| Arm64EC      | `vreg`         | `v`      | `v0`           | None          |
| Arm64EC      | `vreg`         | `b`      | `b0`           | `b`           |
| Arm64EC      | `vreg`         | `h`      | `h0`           | `h`           |
| Arm64EC      | `vreg`         | `s`      | `s0`           | `s`           |
| Arm64EC      | `vreg`         | `d`      | `d0`           | `d`           |
| Arm64EC      | `vreg`         | `q`      | `q0`           | `q`           |

# Flags covered by `preserves_flags`

These flags registers must be restored upon exiting the asm block if the `preserves_flags` option is set:
- AVR
  - The status register `SREG`.
- MSP430
  - The status register `r2`.
- M68k
  - The condition code register `ccr`.
- s390x
  - The condition code register `cc`.
- Arm64EC
  - Condition flags (`NZCV` register).
  - Floating-point status (`FPSR` register).
