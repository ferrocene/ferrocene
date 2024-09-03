use crate::spec::{PanicStrategy, RelocModel, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "lynxos178".into(),
        vendor: "lynx".into(),
        linker: Some("i386-lynx-lynxos178-gcc-7.1.0".into()),
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        emit_debug_gdb_scripts: false,
        has_thread_local: true,
        crt_static_default: true,
        crt_static_respected: true,
        crt_static_allows_dylibs: false,
        ..Default::default()
    }
}
