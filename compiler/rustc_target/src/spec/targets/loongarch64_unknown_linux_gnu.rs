use crate::spec::{base, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "loongarch64-unknown-linux-gnu".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n64-S128".into(),
        arch: "loongarch64".into(),
        options: TargetOptions {
            cpu: "generic".into(),
            features: "+f,+d".into(),
            llvm_abiname: "lp64d".into(),
            max_atomic_width: Some(64),
            direct_access_external_data: Some(false),
            ..base::linux_gnu::opts()
        },
    }
}
