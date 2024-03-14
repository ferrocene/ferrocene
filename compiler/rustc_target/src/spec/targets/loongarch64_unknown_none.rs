use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "loongarch64-unknown-none".into(),
        description: None,
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n64-S128".into(),
        arch: "loongarch64".into(),
        options: TargetOptions {
            cpu: "generic".into(),
            features: "+f,+d".into(),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            llvm_abiname: "lp64d".into(),
            max_atomic_width: Some(64),
            relocation_model: RelocModel::Static,
            panic_strategy: PanicStrategy::Abort,
            code_model: Some(CodeModel::Small),
            ..Default::default()
        },
    }
}
