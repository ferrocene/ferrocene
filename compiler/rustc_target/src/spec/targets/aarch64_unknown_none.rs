// Generic AArch64 target for bare-metal code - Floating point enabled
//
// Can be used in conjunction with the `target-feature` and
// `target-cpu` compiler flags to opt-in more hardware-specific
// features.
//
// For example, `-C target-cpu=cortex-a53`.

<<<<<<< HEAD:compiler/rustc_target/src/spec/aarch64_unknown_none.rs
use super::{PanicStrategy, RelocModel, SanitizerSet, Target, TargetOptions};
=======
use crate::spec::{
    Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, SanitizerSet, Target, TargetOptions,
};
>>>>>>> pull-upstream-temp--do-not-use-for-real-code:compiler/rustc_target/src/spec/targets/aarch64_unknown_none.rs

pub fn target() -> Target {
    let opts = TargetOptions {
        // FERROCENE: use the default linker rather than rust-lld
        //linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        //linker: Some("rust-lld".into()),
        features: "+v8a,+strict-align,+neon,+fp-armv8".into(),
        supported_sanitizers: SanitizerSet::KCFI | SanitizerSet::KERNELADDRESS,
        relocation_model: RelocModel::Static,
        disable_redzone: true,
        max_atomic_width: Some(128),
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    };
    Target {
        llvm_target: "aarch64-unknown-none".into(),
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: opts,
    }
}
