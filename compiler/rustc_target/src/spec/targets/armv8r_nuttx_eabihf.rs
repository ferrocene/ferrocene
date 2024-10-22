// Targets the Cortex-M33 processor (Armv8-M Mainline architecture profile),
// with the Floating Point extension.

use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, RelocModel, Target, TargetOptions};

const LINKER_SCRIPT: &str = include_str!("./armv8r_nuttx_eabihf_linker_script.ld");

pub(crate) fn target() -> Target {
    let mut options = TargetOptions {
        // nuttx-specific
        // families: cvs!["unix"],
        os: "nuttx".into(),
        link_script: Some(LINKER_SCRIPT.into()),

        abi: "eabihf".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        relocation_model: RelocModel::Static,
        panic_strategy: PanicStrategy::Abort,
        // The Cortex-R52 has two variants with respect to floating-point support:
        // 1. fp-armv8, SP-only, with 16 DP (32 SP) registers
        // 2. neon-fp-armv8, SP+DP, with 32 DP registers
        // Use the lesser of these two options as the default, as it will produce code
        // compatible with either variant.
        //
        // Reference:
        // Arm Cortex-R52 Processor Technical Reference Manual
        // - Chapter 15 Advanced SIMD and floating-point support
        features: "+fp-armv8,-fp64,-d32".into(),
        max_atomic_width: Some(64),
        emit_debug_gdb_scripts: false,
        // GCC defaults to 8 for arm-none here.
        c_enum_min_bits: Some(8),
        ..Default::default()
    };

    // nuttx-specific
    options.add_pre_link_args(LinkerFlavor::Gnu(Cc::No, Lld::Yes), &["-r", "-emain"]);

    Target {
        llvm_target: "armv8r-nuttx-eabihf".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),
        options,
    }
}
