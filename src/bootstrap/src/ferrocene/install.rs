// Based off ../core/build_steps/install.rs

//! Implementation of the install aspects of the compiler.
//!
//! This module is responsible for installing the standard library,
//! compiler, and documentation.

// use std::env;
use std::fs;
use std::path::{PathBuf, Path};
// use std::process::Command;

use walkdir::WalkDir;

use crate::core::build_steps::dist;
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::{Config, TargetSelection};
use crate::utils::tarball::GeneratedTarball;
use crate::{Compiler, Kind};

fn install(
    builder: &Builder<'_>,
    package: &str,
    stage: u32,
    host: Option<TargetSelection>,
    tarball: &GeneratedTarball,
) {
    let _guard = builder.msg(Kind::Install, stage, package, host, host);

    let tarball_output = tarball.work_dir();
    let image_dir = tarball_output.join("image");

    let prefix = {
        let mut prefix = default_path(&builder.config.prefix, "/usr/local");
        let destdir_env = std::env::var_os("DESTDIR").map(PathBuf::from);
        // The DESTDIR environment variable is a standard way to install software in a subdirectory
        // while keeping the original directory structure, even if the prefix or other directories
        // contain absolute paths.
        //
        // More information on the environment variable is available here:
        // https://www.gnu.org/prep/standards/html_node/DESTDIR.html
        if let Some(destdir) = destdir_env {
            let without_destdir = prefix.clone();
            prefix.clone_from(&destdir);
            // Custom .join() which ignores disk roots.
            for part in without_destdir.components() {
                if let std::path::Component::Normal(s) = part {
                    prefix.push(s)
                }
            }
        }
        prefix
    };
    
    let remap = |from: &Path, to: &Path| -> std::io::Result<()> {
        if !from.exists() {
            return Ok(())
        }
        for maybe_entry in WalkDir::new(&from) {
            let entry = maybe_entry.unwrap();
            let entry_relative = entry.path().strip_prefix(&from).unwrap();
            let destination = to.join(entry_relative);
            let ty = entry.file_type();
            if ty.is_dir() && !destination.exists() {
                fs::create_dir_all(&destination)?;
            } else if ty.is_file() {
                fs::rename(entry.path(), destination).unwrap();
            }
        }
        Ok(())
    };

    let sysconfdir = prefix.join(default_path(&builder.config.sysconfdir, "etc"));
    remap(&image_dir.join("etc"), &sysconfdir).unwrap();

    let datadir = prefix.join(default_path(&builder.config.datadir, "share"));
    remap(&image_dir.join("share"), &datadir).unwrap();

    let docdir = prefix.join(default_path(&builder.config.docdir, &format!("share/doc/{package}")));
    remap(&image_dir.join("share/doc"), &docdir).unwrap();

    let mandir = prefix.join(default_path(&builder.config.mandir, "share/man"));
    remap(&image_dir.join("share/man"), &mandir).unwrap();

    let libdir = prefix.join(default_path(&builder.config.libdir, "lib"));
    remap(&image_dir.join("lib"), &libdir).unwrap();

    let bindir = prefix.join(&builder.config.bindir); // Default in config.rs
    remap(&image_dir.join("bin"), &bindir).unwrap();
}

fn default_path(config: &Option<PathBuf>, default: &str) -> PathBuf {
    config.as_ref().cloned().unwrap_or_else(|| PathBuf::from(default))
}

macro_rules! install {
    (($sel:ident, $builder:ident, $_config:ident),
       $($name:ident,
       $condition_name: ident = $path_or_alias: literal,
       $default_cond:expr,
       only_hosts: $only_hosts:expr,
       $run_item:block $(, $c:ident)*;)+) => {
        $(
            #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        pub struct $name {
            pub compiler: Compiler,
            pub target: TargetSelection,
        }

        impl $name {
            #[allow(dead_code)]
            fn should_build(config: &Config) -> bool {
                config.extended && config.tools.as_ref()
                    .map_or(true, |t| t.contains($path_or_alias))
            }
        }

        impl Step for $name {
            type Output = ();
            const DEFAULT: bool = true;
            const ONLY_HOSTS: bool = $only_hosts;
            $(const $c: bool = true;)*

            fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                let $_config = &run.builder.config;
                run.$condition_name($path_or_alias).default_condition($default_cond)
            }

            fn make_run(run: RunConfig<'_>) {
                run.builder.ensure($name {
                    compiler: run.builder.compiler(run.builder.top_stage, run.builder.config.build),
                    target: run.target,
                });
            }

            fn run($sel, $builder: &Builder<'_>) {
                $run_item
            }
        })+
    }
}

install!((self, builder, _config),
    Docs, path = "src/doc", _config.docs, only_hosts: false, {
        let tarball = builder.ensure(dist::Docs { host: self.target }).expect("missing docs");
        install(builder, "docs", self.compiler.stage, Some(self.target), &tarball);
    };
    Std, path = "library/std", true, only_hosts: false, {
        // `expect` should be safe, only None when host != build, but this
        // only runs when host == build
        let tarball = builder.ensure(dist::Std {
            compiler: self.compiler,
            target: self.target
        }).expect("missing std");
        install(builder, "std", self.compiler.stage, Some(self.target), &tarball);
    };
    Cargo, alias = "cargo", Self::should_build(_config), only_hosts: true, {
        let tarball = builder
            .ensure(dist::Cargo { compiler: self.compiler, target: self.target })
            .expect("missing cargo");
        install(builder, "cargo", self.compiler.stage, Some(self.target), &tarball);
    };
    RustAnalyzer, alias = "rust-analyzer", Self::should_build(_config), only_hosts: true, {
        if let Some(tarball) =
            builder.ensure(dist::RustAnalyzer { compiler: self.compiler, target: self.target })
        {
            install(builder, "rust-analyzer", self.compiler.stage, Some(self.target), &tarball);
        } else {
            builder.info(
                &format!("skipping Install rust-analyzer stage{} ({})", self.compiler.stage, self.target),
            );
        }
    };
    Clippy, alias = "clippy", Self::should_build(_config), only_hosts: true, {
        let tarball = builder
            .ensure(dist::Clippy { compiler: self.compiler, target: self.target })
            .expect("missing clippy");
        install(builder, "clippy", self.compiler.stage, Some(self.target), &tarball);
    };
    Miri, alias = "miri", Self::should_build(_config), only_hosts: true, {
        if let Some(tarball) = builder.ensure(dist::Miri { compiler: self.compiler, target: self.target }) {
            install(builder, "miri", self.compiler.stage, Some(self.target), &tarball);
        } else {
            // Miri is only available on nightly
            builder.info(
                &format!("skipping Install miri stage{} ({})", self.compiler.stage, self.target),
            );
        }
    };
    LlvmTools, alias = "llvm-tools", Self::should_build(_config), only_hosts: true, {
        if let Some(tarball) = builder.ensure(dist::LlvmTools { target: self.target }) {
            install(builder, "llvm-tools", self.compiler.stage, Some(self.target), &tarball);
        } else {
            builder.info(
                &format!("skipping llvm-tools stage{} ({}): external LLVM", self.compiler.stage, self.target),
            );
        }
    };
    Rustfmt, alias = "rustfmt", Self::should_build(_config), only_hosts: true, {
        if let Some(tarball) = builder.ensure(dist::Rustfmt {
            compiler: self.compiler,
            target: self.target
        }) {
            install(builder, "rustfmt", self.compiler.stage, Some(self.target), &tarball);
        } else {
            builder.info(
                &format!("skipping Install Rustfmt stage{} ({})", self.compiler.stage, self.target),
            );
        }
    };
    RustDemangler, alias = "rust-demangler", Self::should_build(_config), only_hosts: true, {
        // NOTE: Even though `should_build` may return true for `extended` default tools,
        // dist::RustDemangler may still return None, unless the target-dependent `profiler` config
        // is also true, or the `tools` array explicitly includes "rust-demangler".
        if let Some(tarball) = builder.ensure(dist::RustDemangler {
            compiler: self.compiler,
            target: self.target
        }) {
            install(builder, "rust-demangler", self.compiler.stage, Some(self.target), &tarball);
        } else {
            builder.info(
                &format!("skipping Install RustDemangler stage{} ({})",
                         self.compiler.stage, self.target),
            );
        }
    };
    Rustc, path = "compiler/rustc", true, only_hosts: true, {
        let tarball = builder.ensure(dist::Rustc {
            compiler: builder.compiler(builder.top_stage, self.target),
        });
        install(builder, "rustc", self.compiler.stage, Some(self.target), &tarball);
    };
    RustcCodegenCranelift, alias = "rustc-codegen-cranelift", Self::should_build(_config), only_hosts: true, {
        if let Some(tarball) = builder.ensure(dist::CodegenBackend {
            compiler: self.compiler,
            backend: "cranelift".to_string(),
        }) {
            install(builder, "rustc-codegen-cranelift", self.compiler.stage, Some(self.target), &tarball);
        } else {
            builder.info(
                &format!("skipping Install CodegenBackend(\"cranelift\") stage{} ({})",
                         self.compiler.stage, self.target),
            );
        }
    };
    LlvmBitcodeLinker, alias = "llvm-bitcode-linker", Self::should_build(_config), only_hosts: true, {
        if let Some(tarball) = builder.ensure(dist::LlvmBitcodeLinker { compiler: self.compiler, target: self.target }) {
            install(builder, "llvm-bitcode-linker", self.compiler.stage, Some(self.target), &tarball);
        } else {
            builder.info(
                &format!("skipping llvm-bitcode-linker stage{} ({})", self.compiler.stage, self.target),
            );
        }
    };
);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Src {
    pub stage: u32,
}

impl Step for Src {
    type Output = ();
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let config = &run.builder.config;
        let cond = config.extended && config.tools.as_ref().map_or(true, |t| t.contains("src"));
        run.path("src").default_condition(cond)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Src { stage: run.builder.top_stage });
    }

    fn run(self, builder: &Builder<'_>) {
        let tarball = builder.ensure(dist::Src);
        install(builder, "src", self.stage, None, &tarball);
    }
}
