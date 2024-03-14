use crate::spec::{base, RelocModel, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let base = base::solid::opts("asp3");
    Target {
        llvm_target: "aarch64-unknown-none".into(),
        description: None,
        pointer_width: 64,
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        arch: "aarch64".into(),
        options: TargetOptions {
            linker: Some("aarch64-kmc-elf-gcc".into()),
            features: "+v8a,+neon,+fp-armv8".into(),
            relocation_model: RelocModel::Static,
            disable_redzone: true,
            max_atomic_width: Some(128),
            stack_probes: StackProbeType::Inline,
            ..base
        },
    }
}
