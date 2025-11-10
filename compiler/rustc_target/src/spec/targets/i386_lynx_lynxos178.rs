use crate::spec::{Arch, Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, Target, base};

pub(crate) fn target() -> Target {
    let mut base = base::lynxos178::opts();
    base.cpu = "i386".into();
    base.max_atomic_width = Some(32);
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m32"]);
    base.stack_probes = StackProbeType::Inline;
    base.vendor = "lynx".into();
    base.linker = Some("i386-lynx-lynxos178-gcc-7.1.0".into());
    base.relocation_model = RelocModel::Static;
    base.emit_debug_gdb_scripts = false;
    base.has_thread_local = true;
    base.crt_static_default = true;
    base.crt_static_allows_dylibs = false;

    Target {
        llvm_target: "i686-unknown-linux-gnu".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i128:128-f64:32:64-f80:32-n8:16:32-S128"
            .into(),
        arch: Arch::X86,
        options: base,
    }
}
