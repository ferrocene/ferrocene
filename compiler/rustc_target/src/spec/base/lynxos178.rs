<<<<<<< HEAD
use crate::spec::{PanicStrategy, RelocModel, TargetOptions};
=======
use std::borrow::Cow;

use crate::spec::{
    PanicStrategy, RelocModel, RelroLevel, SplitDebuginfo, StackProbeType, TargetOptions, cvs,
};
>>>>>>> pull-upstream-temp--do-not-use-for-real-code

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "lynxos178".into(),
<<<<<<< HEAD
        vendor: "lynx".into(),
        linker: Some("i386-lynx-lynxos178-gcc-7.1.0".into()),
        panic_strategy: PanicStrategy::Abort,
        relocation_model: RelocModel::Static,
        emit_debug_gdb_scripts: false,
        has_thread_local: true,
        crt_static_default: true,
        crt_static_respected: true,
        crt_static_allows_dylibs: false,
=======
        dynamic_linking: false,
        families: cvs!["unix"],
        position_independent_executables: false,
        static_position_independent_executables: false,
        relro_level: RelroLevel::Full,
        has_thread_local: false,
        crt_static_respected: true,
        panic_strategy: PanicStrategy::Abort,
        linker: Some(Cow::Borrowed("x86_64-lynx-lynxos178-gcc")),
        no_default_libraries: false,
        eh_frame_header: false, // GNU ld (GNU Binutils) 2.37.50 does not support --eh-frame-hdr
        max_atomic_width: Some(64),
        supported_split_debuginfo: Cow::Borrowed(&[
            SplitDebuginfo::Packed,
            SplitDebuginfo::Unpacked,
            SplitDebuginfo::Off,
        ]),
        relocation_model: RelocModel::Static,
        stack_probes: StackProbeType::Inline,
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
        ..Default::default()
    }
}
