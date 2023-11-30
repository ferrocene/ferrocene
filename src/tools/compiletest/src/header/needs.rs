use crate::common::{Config, Debugger};
use crate::header::IgnoreDecision;
use crate::util;

pub(super) fn handle_needs(
    cache: &CachedNeedsConditions,
    config: &Config,
    ln: &str,
) -> IgnoreDecision {
    // Note thet we intentionally still put the needs- prefix here to make the file show up when
    // grepping for a directive name, even though we could technically strip that.
    let needs = &[
        Need {
            name: "needs-asm-support",
            condition: config.has_asm_support(),
            ignore_reason: "ignored on targets without inline assembly support",
        },
        Need {
            name: "needs-sanitizer-support",
            condition: cache.sanitizer_support,
            ignore_reason: "ignored on targets without sanitizers support",
        },
        Need {
            name: "needs-sanitizer-address",
            condition: cache.sanitizer_address,
            ignore_reason: "ignored on targets without address sanitizer",
        },
        Need {
            name: "needs-sanitizer-cfi",
            condition: cache.sanitizer_cfi,
            ignore_reason: "ignored on targets without CFI sanitizer",
        },
        Need {
            name: "needs-sanitizer-kcfi",
            condition: cache.sanitizer_kcfi,
            ignore_reason: "ignored on targets without kernel CFI sanitizer",
        },
        Need {
            name: "needs-sanitizer-leak",
            condition: cache.sanitizer_leak,
            ignore_reason: "ignored on targets without leak sanitizer",
        },
        Need {
            name: "needs-sanitizer-memory",
            condition: cache.sanitizer_memory,
            ignore_reason: "ignored on targets without memory sanitizer",
        },
        Need {
            name: "needs-sanitizer-thread",
            condition: cache.sanitizer_thread,
            ignore_reason: "ignored on targets without thread sanitizer",
        },
        Need {
            name: "needs-sanitizer-hwaddress",
            condition: cache.sanitizer_hwaddress,
            ignore_reason: "ignored on targets without hardware-assisted address sanitizer",
        },
        Need {
            name: "needs-sanitizer-memtag",
            condition: cache.sanitizer_memtag,
            ignore_reason: "ignored on targets without memory tagging sanitizer",
        },
        Need {
            name: "needs-sanitizer-shadow-call-stack",
            condition: cache.sanitizer_shadow_call_stack,
            ignore_reason: "ignored on targets without shadow call stacks",
        },
        Need {
            name: "needs-run-enabled",
            condition: config.run_enabled(),
            ignore_reason: "ignored when running the resulting test binaries is disabled",
        },
        Need {
            name: "needs-unwind",
            condition: config.can_unwind(),
            ignore_reason: "ignored on targets without unwinding support",
        },
        Need {
            name: "needs-profiler-support",
            condition: std::env::var_os("RUSTC_PROFILER_SUPPORT").is_some(),
            ignore_reason: "ignored when profiler support is disabled",
        },
        Need {
            name: "needs-matching-clang",
            condition: config.run_clang_based_tests_with.is_some(),
            ignore_reason: "ignored when the used clang does not match the built LLVM",
        },
        Need {
            name: "needs-rust-lld",
            condition: cache.rust_lld,
            ignore_reason: "ignored on targets without Rust's LLD",
        },
        Need {
            name: "needs-rust-lldb",
            condition: config.debugger != Some(Debugger::Lldb) || config.lldb_native_rust,
            ignore_reason: "ignored on targets without Rust's LLDB",
        },
        Need {
            name: "needs-dynamic-linking",
            condition: config.target_cfg().dynamic_linking,
            ignore_reason: "ignored on targets without dynamic linking",
        },
    ];

    let (name, comment) = match ln.split_once([':', ' ']) {
        Some((name, comment)) => (name, Some(comment)),
        None => (ln, None),
    };

    if !name.starts_with("needs-") {
        return IgnoreDecision::Continue;
    }

    // Handled elsewhere.
    if name == "needs-llvm-components" {
        return IgnoreDecision::Continue;
    }

    let mut found_valid = false;
    for need in needs {
        if need.name == name {
            if need.condition {
                found_valid = true;
                break;
            } else {
                return IgnoreDecision::Ignore {
                    reason: if let Some(comment) = comment {
                        format!("{} ({comment})", need.ignore_reason)
                    } else {
                        need.ignore_reason.into()
                    },
                };
            }
        }
    }

    if found_valid {
        IgnoreDecision::Continue
    } else {
        IgnoreDecision::Error { message: format!("invalid needs directive: {name}") }
    }
}

struct Need {
    name: &'static str,
    condition: bool,
    ignore_reason: &'static str,
}

pub(super) struct CachedNeedsConditions {
    sanitizer_support: bool,
    sanitizer_address: bool,
    sanitizer_cfi: bool,
    sanitizer_kcfi: bool,
    sanitizer_leak: bool,
    sanitizer_memory: bool,
    sanitizer_thread: bool,
    sanitizer_hwaddress: bool,
    sanitizer_memtag: bool,
    sanitizer_shadow_call_stack: bool,
    rust_lld: bool,
}

impl CachedNeedsConditions {
    pub(super) fn load(config: &Config) -> Self {
        let target = &&*config.target;
        Self {
            sanitizer_support: std::env::var_os("RUSTC_SANITIZER_SUPPORT").is_some(),
            sanitizer_address: util::ASAN_SUPPORTED_TARGETS.contains(target),
            sanitizer_cfi: util::CFI_SUPPORTED_TARGETS.contains(target),
            sanitizer_kcfi: util::KCFI_SUPPORTED_TARGETS.contains(target),
            sanitizer_leak: util::LSAN_SUPPORTED_TARGETS.contains(target),
            sanitizer_memory: util::MSAN_SUPPORTED_TARGETS.contains(target),
            sanitizer_thread: util::TSAN_SUPPORTED_TARGETS.contains(target),
            sanitizer_hwaddress: util::HWASAN_SUPPORTED_TARGETS.contains(target),
            sanitizer_memtag: util::MEMTAG_SUPPORTED_TARGETS.contains(target),
            sanitizer_shadow_call_stack: util::SHADOWCALLSTACK_SUPPORTED_TARGETS.contains(target),

            // For tests using the `needs-rust-lld` directive (e.g. for `-Zgcc-ld=lld`), we need to find
            // whether `rust-lld` is present in the compiler under test.
            //
            // The --compile-lib-path is the path to host shared libraries, but depends on the OS. For
            // example:
            // - on linux, it can be <sysroot>/lib
            // - on windows, it can be <sysroot>/bin
            //
            // However, `rust-lld` is only located under the lib path, so we look for it there.
            rust_lld: config
                .compile_lib_path
                .parent()
                .expect("couldn't traverse to the parent of the specified --compile-lib-path")
                .join("lib")
                .join("rustlib")
                .join(target)
                .join("bin")
                .join(if config.host.contains("windows") { "rust-lld.exe" } else { "rust-lld" })
                .exists(),
        }
    }
}
