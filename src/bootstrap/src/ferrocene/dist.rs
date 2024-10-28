// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

pub(crate) mod flip_link;
pub(crate) mod probe_rs;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

use serde_json::json;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::doc::ensure_all_xml_doctrees;
use crate::ferrocene::test_outcomes::TestOutcomesDir;
use crate::t;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Docs {
    pub(crate) target: TargetSelection,
}

impl Step for Docs {
    type Output = Vec<GeneratedTarball>;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.docs;
        run.alias("ferrocene-docs").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Docs { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        // Build all of the documentation.
        builder.default_doc(&[]);
        let doc_out = builder.out.join(&self.target.triple).join("doc");

        let mut subsetter = Subsetter::new(builder, "ferrocene-docs", "share/doc/ferrocene/html");
        subsetter.add_directory(&doc_out, &doc_out);

        subsetter.into_tarballs().map(|tarball| tarball.generate()).collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct DocsDoctrees {
    target: TargetSelection,
}

impl Step for DocsDoctrees {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let default = run.builder.config.docs;
        run.alias("ferrocene-docs-doctrees").default_condition(default)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let paths = ensure_all_xml_doctrees(builder, self.target);

        let tarball = Tarball::new_targetless(builder, "ferrocene-docs-doctrees");
        for (name, path) in paths {
            tarball.add_dir(path, format!("share/doc/ferrocene/xml-doctrees/{name}"));
        }

        tarball.generate()
    }
}

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
        const DIRS: &[&str] = &["src", "compiler", "library", "tests", "ferrocene", "LICENSES"];
        const FILES: &[&str] = &[
            "COPYRIGHT",
            "LICENSE-APACHE",
            "LICENSE-MIT",
            "README.md",
            "RELEASES.md",
            "REUSE.toml",
            "config.example.toml",
            "Cargo.toml",
            "Cargo.lock",
            "configure",
            "x",
            "x.py",
            "x.ps1",
            ".gitmodules",
        ];
        const EXTRA_CARGO_TOMLS: &[&str] = &[
            "compiler/rustc_codegen_cranelift/Cargo.toml",
            "compiler/rustc_codegen_gcc/Cargo.toml",
            "library/Cargo.toml",
            "src/bootstrap/Cargo.toml",
            "src/tools/cargo/Cargo.toml",
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
        vendor.arg("--versioned-dirs"); // See https://github.com/rust-lang/rust/pull/122892
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

struct Subsetter<'a> {
    builder: &'a Builder<'a>,
    name_prefix: String,
    output_prefix: PathBuf,

    tarballs: BTreeMap<Option<String>, Rc<Tarball<'a>>>,
    directory_subset: Option<String>,
}

impl<'a> Subsetter<'a> {
    fn new(builder: &'a Builder<'a>, name_prefix: &str, output_prefix: &str) -> Self {
        Self {
            builder,
            name_prefix: name_prefix.into(),
            output_prefix: output_prefix.into(),
            tarballs: BTreeMap::new(),
            directory_subset: None,
        }
    }

    fn add_directory(&mut self, root: &Path, path: &Path) {
        let old_subset = self.directory_subset.clone();

        let subset_file = path.join("ferrocene-subset");
        match std::fs::read_to_string(&subset_file) {
            Ok(data) => self.directory_subset = Some(self.parse_subset_file(&subset_file, &data)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => panic!("failed to read ferrocene-subset in {}: {err}", path.display()),
        }

        for entry in std::fs::read_dir(path).unwrap() {
            let path = entry.as_ref().unwrap().path();
            if path.is_file() {
                self.add_file(root, &path);
            } else if path.is_dir() {
                self.add_directory(root, &path);
            }
        }

        self.directory_subset = old_subset;
    }

    fn add_file(&mut self, root: &Path, path: &Path) {
        let mut subset = self.directory_subset.clone();

        // Allow overriding the directory subset with per-file subsets.
        let mut subset_file = path.to_path_buf();
        if subset_file.to_str().map(|p| !p.ends_with(".ferrocene-subset")).unwrap_or(true) {
            // If the file itself is the $name.ferrocene-subset file, include it in the same subset
            // it references.
            subset_file.as_mut_os_string().push(".ferrocene-subset");
        }
        match std::fs::read_to_string(&subset_file) {
            Ok(data) => subset = Some(self.parse_subset_file(&subset_file, &data)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => panic!("failed to read {}: {err}", subset_file.display()),
        }

        let tarball = match self.tarballs.get(&subset) {
            Some(tarball) => tarball.clone(),
            None => {
                let name = match &subset {
                    Some(name) => format!("{}-{name}", self.name_prefix),
                    None => self.name_prefix.clone(),
                };
                let tarball = Rc::new(Tarball::new_targetless(self.builder, &name));
                self.tarballs.insert(subset, tarball.clone());
                tarball
            }
        };

        let relative = path.strip_prefix(root).unwrap();
        let mode = if self.is_executable(&path) { 0o755 } else { 0o644 };
        tarball.add_file(&path, self.output_prefix.join(relative).parent().unwrap(), mode);
    }

    #[cfg(unix)]
    fn is_executable(&self, path: &Path) -> bool {
        use std::os::unix::prelude::PermissionsExt;
        std::fs::metadata(path).unwrap().permissions().mode() & 0o111 > 0
    }

    // This is kind of a blunt instrument, but Windows lacks file modes to check otherwise.
    #[cfg(windows)]
    fn is_executable(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() { extension == "exe" } else { false }
    }

    fn parse_subset_file(&self, path: &Path, contents: &str) -> String {
        let mut lines = contents
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with('#'))
            .filter(|line| !line.is_empty());

        let Some(subset) = lines.next() else {
            panic!("no content in subset file {}", path.display());
        };
        if !subset.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            panic!("subset name {subset:?} contains invalid chars (in {})", path.display());
        }
        if lines.next().is_some() {
            panic!("multiple subset names in {}", path.display());
        }

        subset.into()
    }

    fn into_tarballs(self) -> impl Iterator<Item = Tarball<'a>> {
        self.tarballs.into_values().map(|tarball| Rc::try_unwrap(tarball).map_err(|_| ()).unwrap())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SelfTest {
    pub(crate) target: TargetSelection,
}

impl Step for SelfTest {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-self-test")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(SelfTest { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let self_test = builder.ensure(crate::ferrocene::tool::SelfTest { target: self.target });

        let mut tarball = Tarball::new(builder, "ferrocene-self-test", &self.target.triple);
        tarball.add_file(self_test, "bin", 0o755);

        tarball.ferrocene_proxied_binary("bin/ferrocene-self-test");
        tarball.generate()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TestOutcomes;

impl Step for TestOutcomes {
    type Output = Option<GeneratedTarball>;
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-test-outcomes")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TestOutcomes);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let Some(test_outcomes) = builder.ensure(TestOutcomesDir) else { return None };

        let tarball = Tarball::new_targetless(builder, "ferrocene-test-outcomes");
        tarball.add_dir(test_outcomes, "share/ferrocene/test-outcomes");
        Some(tarball.generate())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GenerateBuildMetadata;

impl Step for GenerateBuildMetadata {
    type Output = ();
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-build-metadata")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(GenerateBuildMetadata);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        if builder.config.dry_run() {
            return;
        }

        let git_info = builder.rust_info();
        let (Some(sha1_full), Some(sha1_short)) = (&git_info.sha(), &git_info.sha_short()) else {
            panic!("generating the build metadata requires git information");
        };

        let dist_dir = "build/dist";

        let ferrocene_version = t!(fs::read_to_string("ferrocene/version"));
        let ferrocene_version = ferrocene_version.trim();
        let ferrocene_channel = &builder.config.ferrocene_raw_channel;
        let src_version = t!(fs::read_to_string("src/version"));
        let src_version = src_version.trim();

        // Perform validation on the contents of the version field, to avoid generating
        // artifacts that will break the release process.
        if ferrocene_channel == "rolling" && ferrocene_version != "rolling" {
            panic!(
                "error: ferrocene/version must be 'rolling' when ferrocene/ci/channel is 'rolling' but it was '{ferrocene_version}'"
            );
        }

        let channel = crate::ferrocene::ferrocene_channel(builder, ferrocene_version);

        // Whenever the contents of this JSON file change, even just adding new fields,
        // make sure to increase the metadata version number and update publish-release
        // accordingly. Note that new releases *won't* be made until publish-release and
        // this use the same version number.
        let data = json!({
            "metadata_version": 4,
            "rust_version": src_version,
            "rust_channel": builder.config.channel,
            "ferrocene_version": ferrocene_version,
            "ferrocene_channel": ferrocene_channel,
            "channel": channel,
            "sha1_full": sha1_full,
            "sha1_short": sha1_short,
        })
        .to_string();

        builder.create_dir(dist_dir.as_ref());

        builder.create(format!("{dist_dir}/ferrocene-ci-metadata.json").as_ref(), &data);

        // Add the list of packages to include in the release to the artifacts, so that
        // publish-release knows what to expect for this commit.
        builder.copy_link_to_folder("ferrocene/packages.toml".as_ref(), dist_dir.as_ref());
    }
}
