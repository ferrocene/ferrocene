use crate::abi::Endian;
use crate::spec::{base, Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-mspe"]);
    base.max_atomic_width = Some(32);
    base.stack_probes = StackProbeType::Inline;

    Target {
        llvm_target: "powerpc-unknown-linux-gnuspe".into(),
        description: None,
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-Fn32-i64:64-n32".into(),
        arch: "powerpc".into(),
        options: TargetOptions {
            abi: "spe".into(),
            endian: Endian::Big,
            mcount: "_mcount".into(),
            ..base
        },
    }
}
