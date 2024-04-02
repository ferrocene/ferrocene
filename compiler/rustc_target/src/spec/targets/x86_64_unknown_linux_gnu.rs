use crate::spec::{base, Cc, LinkerFlavor, Lld, SanitizerSet, StackProbeType, Target};

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.cpu = "x86-64".into();
    base.plt_by_default = false;
    base.max_atomic_width = Some(64);
    // Ferrocene elects to use lld as the linker on this target.
    // Upstream still uses ld.
    //
    // Setting Lld::Yes causes `-fuse-ld=lld` to be added to the linker args
    // (and by linker I mean `cc` as the linker as we use Cc::Yes)
    base.linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::Yes);
    // This setting causes `-B path/to/gcc-ld` to be added to the linker args.
    // This means that the `ld.lld` wrapper for `rust-lld` appears in the
    // path that `cc` uses to find `ld.lld` (also the path used for `cpp` and
    // `cc1`, but we don't supply those so it goes back to the defaults for
    // those tools).
    base.link_self_contained = crate::spec::LinkSelfContainedDefault::with_linker();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-m64"]);
    base.stack_probes = StackProbeType::Inline;
    base.static_position_independent_executables = true;
    base.supported_sanitizers = SanitizerSet::ADDRESS
        | SanitizerSet::CFI
        | SanitizerSet::KCFI
        | SanitizerSet::DATAFLOW
        | SanitizerSet::LEAK
        | SanitizerSet::MEMORY
        | SanitizerSet::SAFESTACK
        | SanitizerSet::THREAD;
    base.supports_xray = true;
    base.llvm_args = crate::spec::cvs!["--protect-from-escaped-allocas"];

    Target {
        llvm_target: "x86_64-unknown-linux-gnu".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
    }
}
