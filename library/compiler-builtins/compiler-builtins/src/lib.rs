#![cfg_attr(feature = "compiler-builtins", compiler_builtins)]
#![cfg_attr(all(target_family = "wasm"), feature(wasm_numeric_instr))]
#![feature(abi_custom)]
#![feature(abi_unadjusted)]
#![feature(asm_experimental_arch)]
#![feature(cfg_target_has_atomic)]
#![feature(compiler_builtins)]
#![feature(core_intrinsics)]
#![feature(linkage)]
#![feature(naked_functions)]
#![feature(repr_simd)]
#![feature(macro_metavar_expr_concat)]
#![feature(rustc_attrs)]
#![feature(float_bits_const)]
#![cfg_attr(f16_enabled, feature(f16))]
#![cfg_attr(f128_enabled, feature(f128))]
#![no_builtins]
#![no_std]
#![allow(unused_features)]
#![allow(internal_features)]
// `mem::swap` cannot be used because it may generate references to memcpy in unoptimized code.
#![allow(clippy::manual_swap)]
// Support compiling on both stage0 and stage1 which may differ in supported stable features.
#![allow(stable_features)]
// By default, disallow this as it is forbidden in edition 2024. There is a lot of unsafe code to
// be migrated, however, so exceptions exist.
#![warn(unsafe_op_in_unsafe_fn)]

// We disable #[no_mangle] for tests so that we can verify the test results
// against the native compiler-rt implementations of the builtins.

// NOTE cfg(all(feature = "c", ..)) indicate that compiler-rt provides an arch optimized
// implementation of that intrinsic and we'll prefer to use that

// NOTE(aapcs, aeabi, arm) ARM targets use intrinsics named __aeabi_* instead of the intrinsics
// that follow "x86 naming convention" (e.g. addsf3). Those aeabi intrinsics must adhere to the
// AAPCS calling convention (`extern "aapcs"`) because that's how LLVM will call them.

#[cfg(test)]
#[cfg(not(feature = "ferrocene_subset"))]
extern crate core;

#[macro_use]
#[cfg(not(feature = "ferrocene_subset"))]
mod macros;

#[cfg(not(feature = "ferrocene_subset"))]
pub mod float;
#[cfg(not(feature = "ferrocene_subset"))]
pub mod int;
#[cfg(not(feature = "ferrocene_subset"))]
pub mod math;
#[cfg(not(feature = "ferrocene_subset"))]
pub mod mem;

// `libm` expects its `support` module to be available in the crate root.
#[cfg(not(feature = "ferrocene_subset"))]
use math::libm_math::support;

#[cfg(target_arch = "arm")]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod arm;

#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod aarch64;

#[cfg(all(target_arch = "aarch64", target_feature = "outline-atomics"))]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod aarch64_outline_atomics;

#[cfg(all(
    kernel_user_helpers,
    any(target_os = "linux", target_os = "android"),
    target_arch = "arm"
))]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod arm_linux;

#[cfg(target_arch = "avr")]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod avr;

#[cfg(target_arch = "hexagon")]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod hexagon;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod riscv;

#[cfg(target_arch = "x86")]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod x86;

#[cfg(target_arch = "x86_64")]
#[cfg(not(feature = "ferrocene_subset"))]
pub mod x86_64;

#[cfg(not(feature = "ferrocene_subset"))]
pub mod probestack;

// ferrocene addition: symbol needed by profiler-builtins on 32-bit ARM
#[cfg(all(ferrocene_facade_secretsauce, target_arch = "arm"))]
#[unsafe(no_mangle)]
fn __atomic_fetch_add_8(ptr: *mut u64, val: u64, model: i32) -> u64 {
    use core::sync::atomic::Ordering::*;

    let ptr = ptr.cast::<u32>();
    let atomic_lsb = unsafe { core::sync::atomic::AtomicU32::from_ptr(ptr) };

    let Ok(val) = u32::try_from(val) else {
        panic!("__atomic_fetch_add_8 val is too large");
    };

    let Ok(model) = i8::try_from(model) else {
        panic!("__atomic_fetch_add_8 model is outside range");
    };

    const RELAXED: i8 = Relaxed as _;
    const RELEASE: i8 = Release as _;
    const ACQUIRE: i8 = Acquire as _;
    const ACQ_REL: i8 = AcqRel as _;
    const SEQ_CST: i8 = SeqCst as _;

    let ordering = match model {
        RELAXED => Relaxed,
        RELEASE => Release,
        ACQUIRE => Acquire,
        ACQ_REL => AcqRel,
        SEQ_CST => SeqCst,
        _ => panic!("__atomic_fetch_add_8 model is invalid"),
    };

    let fetch = atomic_lsb.fetch_add(val, ordering);
    if fetch == u32::MAX {
        panic!("__atomic_fetch_add_8 overflowed");
    }

    fetch.into()
}
