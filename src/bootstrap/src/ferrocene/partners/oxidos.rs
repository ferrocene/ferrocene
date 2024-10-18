// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::ffi::OsStr;
use std::path::PathBuf;

use crate::builder::{Builder, Kind, RunConfig, ShouldRun, Step};
use crate::core::build_steps::compile::run_cargo;
use crate::core::build_steps::tool::SourceType;
use crate::core::config::TargetSelection;
use crate::utils::tarball::Tarball;
use crate::{Compiler, Mode, t};

const OXIDOS_CRATES: &[&str] = &[
    // List of OxidOS crates to prebuild. Their dependencies will be built and included in the
    // tarball as well.
    "kernel",
    "components",
    "deno",
    "wasm",
    "capsules-core",
    "capsules-extra",
];
const OXIDOS_DEBUG_FEATURES: &[&str] = &[
    // We build two variants of OxidOS: the standard variant, and a variant meant for
    // debugging. In the latter, we enable the following Cargo features.
    "kernel/trace_syscalls",
    "kernel/debug_load_processes",
    "kernel/debug_process_credentials",
];
const OXIDOS_CHECK_CFG: &[(&str, &[&str])] = &[
    // The build system enables check-cfg, which errors for unknown cfgs. Since OxidOS doesn't
    // provide its own list of cfgs, we define it here.
    ("has_i128", &[]),
    ("debug", &["true"]),
];
const OXIDOS_ALLOW_UNSTABLE_FEATURES: &[&str] = &[
    // Prevent OxidOS from adding unstable features over time by limiting the unstable features
    // it's allowed to use.
    "core_intrinsics",
    "asm_experimental_arch",
];
const OXIDOS_ALLOW_LINTS: &[&str] = &[
    // Allow internal features to be used. This is needed due to the core_intrinsics feature.
    "internal_features",
    // Allow a warning that will be an error in 2023 edition.
    "static-mut-ref",
    // The source contains redundant prelude imports which are now linted against
    "unused-imports",
    // Avoid "struct `Foo` is never constructed" errors
    "dead-code",
    // Allow "elided lifetime has a name" warnings
    "elided_named_lifetimes",
];

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub(crate) struct DistOxidOs {
    compiler: Compiler,
    target: TargetSelection,
}

impl Step for DistOxidOs {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-oxidos")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(DistOxidOs {
            compiler: run.builder.compiler_for(
                run.builder.top_stage,
                run.builder.config.build,
                run.target,
            ),
            target: run.target,
        });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let variants = [
            BuildOxidOS { compiler: self.compiler, target: self.target, debug: false },
            BuildOxidOS { compiler: self.compiler, target: self.target, debug: true },
        ];

        let tarball = Tarball::new(builder, "oxidos", &self.target.triple);

        if !builder.config.dry_run() {
            for variant in variants {
                let path = builder.ensure(variant);
                let dest = format!("lib/rustlib/{}/lib/builtin/{}", self.target, variant.name());
                for file in t!(std::fs::read_dir(path.join("deps"))) {
                    let file = t!(file).path();
                    if file.extension().and_then(OsStr::to_str) == Some("rlib") {
                        tarball.add_file(&file, &dest, 0o644);
                    }
                }
            }
        }

        tarball.generate();
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct BuildOxidOS {
    compiler: Compiler,
    target: TargetSelection,
    debug: bool,
}

impl BuildOxidOS {
    fn name(&self) -> &'static str {
        match self.debug {
            true => "oxidos-debug",
            false => "oxidos",
        }
    }
}

impl Step for BuildOxidOS {
    type Output = PathBuf;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let compiler = self.compiler;
        let target = self.target;
        let source = builder.ensure(SourceCode);

        builder.ensure(crate::core::build_steps::compile::Std::new(compiler, target));
        if target != compiler.host {
            builder.ensure(crate::core::build_steps::compile::Std::new(compiler, compiler.host));
        }

        let _guard = builder.msg(Kind::Build, compiler.stage, self.name(), compiler.host, target);

        let mode = Mode::ToolCustom { name: self.name() };
        let mut cargo = builder.cargo(compiler, mode, SourceType::InTree, target, Kind::Build);

        cargo.current_dir(&source);
        cargo.rustflag(&format!("-Zallow-features={}", OXIDOS_ALLOW_UNSTABLE_FEATURES.join(",")));

        for lint in OXIDOS_ALLOW_LINTS {
            cargo.rustflag(&format!("-A{lint}"));
        }

        for krate in OXIDOS_CRATES {
            cargo.args(["-p", *krate]);
        }

        for (cfg_name, cfg_values) in OXIDOS_CHECK_CFG {
            let mut flag = format!("--check-cfg=cfg({cfg_name},values(");

            if cfg_values.is_empty() {
                flag.push_str("none()");
            } else {
                for (i, cfg_value) in cfg_values.iter().enumerate() {
                    if i != 0 {
                        flag.push(',');
                    }
                    flag.push('"');
                    flag.push_str(cfg_value);
                    flag.push('"');
                }
            }
            flag.push_str("))");
            cargo.rustflag(&flag);
        }

        if self.debug {
            for feature in OXIDOS_DEBUG_FEATURES {
                cargo.args(["--features", *feature]);
            }
        }

        let stamp =
            builder.cargo_out(compiler, mode, target).join(format!(".{}.stamp", self.name()));
        run_cargo(builder, cargo, Vec::new(), &stamp, Vec::new(), false, false);

        builder.cargo_out(compiler, mode, target)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct SourceCode;

impl Step for SourceCode {
    type Output = PathBuf;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if builder.config.dry_run() {
            return PathBuf::new();
        }

        let Some(tarball) = &builder.config.ferrocene_oxidos_src else {
            eprintln!("Missing OxidOS source tarball. Please set ferrocene.oxidos-src in the");
            eprintln!("config.toml file, pointing to a valid OxidOS source tarball.");
            std::process::exit(1);
        };
        let tarball_name = tarball.rsplit_once('/').map(|(_dir, file)| file).unwrap_or(tarball);

        let mut invalidate_extraction = false;
        let cache = builder.out.join("cache").join("oxidos").join(tarball_name);
        if !cache.exists() {
            if let Some(parent) = cache.parent() {
                t!(std::fs::create_dir_all(parent));
            }
            builder.config.download_file(
                &tarball,
                &cache,
                "Could not download the OxidOS tarball. Ensure that the URL or local path in \
                 ferrocene.oxidos-src is valid.",
            );
            invalidate_extraction = true;
        }

        let dest = builder.out.join("ferrocene").join("oxidos-src");
        let stamp_file = dest.join(".ferrocene-ok");
        if !stamp_file.exists() || invalidate_extraction {
            if dest.exists() {
                t!(std::fs::remove_dir_all(&dest));
            }
            builder.config.unpack(&cache, &dest, "");
        }

        let mut directory_within = None;
        for entry in t!(std::fs::read_dir(&dest)) {
            let entry = t!(entry);
            if entry.path() == stamp_file {
                continue;
            }
            match directory_within {
                None => directory_within = Some(entry.file_name()),
                Some(_) => panic!("multiple top-level files or directories in the OxidOS tarball"),
            }
        }
        let Some(directory_within) = directory_within else {
            panic!("the OxidOS tarball is empty")
        };

        // Once we successfully extracted the source code, we create a "stamp file" in the
        // extracted directory to signal extraction was successful. This allows future invocations
        // to know no extraction is needed again.
        //
        // Without the stamp file we'd need to check if the extracted directory exist, but that
        // would result in a false positive if the extraction was only partial.
        t!(std::fs::write(&stamp_file, b"ok\n"));

        dest.join(directory_within)
    }
}
