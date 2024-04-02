// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::process::Command;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::utils::tarball::GeneratedTarball;

use super::subsetter::Subsetter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SourceTarball;

impl Step for SourceTarball {
    type Output = Vec<GeneratedTarball>;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.alias("ferrocene-src").default_condition(builder.config.rust_dist_src)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SourceTarball);
    }

    fn run(self, builder: &Builder<'_>) -> Vec<GeneratedTarball> {
        // Configuration of what should be included in the tarball.
        const DIRS: &[&str] =
            &["src", "compiler", "library", "tests", "ferrocene", "LICENSES", ".reuse"];
        const FILES: &[&str] = &[
            "COPYRIGHT",
            "LICENSE-APACHE",
            "LICENSE-MIT",
            "README.md",
            "RELEASES.md",
            "config.example.toml",
            "Cargo.toml",
            "Cargo.lock",
            "configure",
            "x",
            "x.py",
            "x.ps1",
        ];
        const EXTRA_CARGO_TOMLS: &[&str] = &[
            "compiler/rustc_codegen_cranelift/Cargo.toml",
            "src/bootstrap/Cargo.toml",
            "src/tools/rust-analyzer/Cargo.toml",
        ];

        let mut subsetter = Subsetter::new(builder, "ferrocene-src", "");

        // Copy raw source files
        for item in DIRS {
            subsetter.add_directory(&builder.src, &builder.src.join(item));
        }
        for item in FILES {
            subsetter.add_file(&builder.src, &builder.src.join(item));
        }

        let generic_tarball = subsetter
            .tarballs
            .get(&None)
            .expect("generic tarball was not generated, all the files were part of a subset")
            .clone();
        let dest_dir = &generic_tarball.image_dir();

        // Include metadata about the git commit. This will be picked up by bootstrap when building
        // Ferrocene from the tarball, so that the final build will include the right git commit
        // even though it didn't come from the repository.
        if let Some(info) = builder.rust_info().info() {
            crate::utils::channel::write_commit_info_file(&dest_dir, info);
        }

        // Vendor Rust dependencies
        let mut vendor = Command::new(&builder.initial_cargo);
        vendor.arg("vendor").arg("vendor/rust").current_dir(&dest_dir);
        vendor.env("RUSTC_BOOTSTRAP", "1"); // std's Cargo.toml uses unstable features
        for extra in EXTRA_CARGO_TOMLS {
            vendor.arg("--sync").arg(&builder.src.join(extra));
        }
        if !builder.config.dry_run() {
            let config = crate::output(&mut vendor);
            builder.create_dir(&dest_dir.join(".cargo"));
            builder.create(&dest_dir.join(".cargo").join("config.toml"), &config);
        }

        drop(generic_tarball);
        subsetter
            .into_tarballs()
            .map(|mut tarball| {
                tarball.permit_symlinks(true);
                tarball.bare()
            })
            .collect()
    }
}
