// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::config::TargetSelection;
use crate::ferrocene::sign::CacheSignatureFiles;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) trait WithSource {
    const SOURCE: &'static str;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BreadcrumbsAssets {
    target: TargetSelection,
    dest: Option<PathBuf>,
}

impl Step for BreadcrumbsAssets {
    type Output = ();
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let src = builder.src.join("ferrocene/doc/breadcrumbs/ferrocene-breadcrumbs.css");
        let dest = self
            .dest
            .unwrap_or_else(|| builder.doc_out(self.target))
            .join("ferrocene-breadcrumbs.css");

        if let Some(parent) = dest.parent() {
            if !parent.is_dir() {
                builder.create_dir(parent);
            }
        }
        builder.copy(&src, &dest);
    }
}

#[derive(Clone)]
struct VirtualEnv {
    path: PathBuf,
}

impl VirtualEnv {
    fn cmd(&self, bin: &str) -> Command {
        #[cfg(not(target_os = "windows"))]
        const BIN_DIR: &str = "bin";
        #[cfg(target_os = "windows")]
        const BIN_DIR: &str = "scripts";

        Command::new(self.path.join(BIN_DIR).join(bin))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SphinxVirtualEnv {
    target: TargetSelection,
}

impl Step for SphinxVirtualEnv {
    type Output = VirtualEnv;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let venv = builder.out.join(self.target.triple).join("ferrocene").join("sphinx-venv");
        let shared_resources =
            builder.src.join("ferrocene").join("doc").join("sphinx-shared-resources");
        let requirements = shared_resources.join("requirements.txt");
        let installed_requirements = venv.join("installed-requirements.txt");

        // Avoid rebuilding the venv if it's up to date.
        if installed_requirements.is_file() {
            if builder.read(&requirements) == builder.read(&installed_requirements) {
                return VirtualEnv { path: venv };
            }
        }

        if venv.is_dir() {
            builder.remove_dir(&venv);
        }
        builder.info("Installing dependencies for building Sphinx documentation");
        builder.run(
            Command::new(
                builder
                    .config
                    .python
                    .as_ref()
                    .expect("Python is required to build Sphinx documentation"),
            )
            .args(&["-m", "venv"])
            .arg(&venv),
        );
        let venv = VirtualEnv { path: venv };
        builder
            .run(venv.cmd("pip").args(&["install", "--require-hashes", "-r"]).arg(&requirements));
        builder.copy(&requirements, &installed_requirements);

        venv
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SphinxBook<P: Step = EmptyStep> {
    target: TargetSelection,
    name: String,
    src: String,
    dest: String,
    only_gather_objects_inv: bool,
    fresh_build: bool,
    signature: SignatureStatus,
    inject_all_other_document_ids: bool,
    parent: Option<P>,
}

impl<P: Step> Step for SphinxBook<P> {
    type Output = PathBuf;
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let src = builder.src.join(&self.src).join("src");
        let out = builder.out.join(self.target.triple).join("doc").join(&self.dest);
        let doctrees = builder.out.join(self.target.triple).join("ferrocene").join(
            if self.only_gather_objects_inv {
                format!("{}-doctrees-objectsinv", self.name)
            } else {
                format!("{}-doctrees", self.name)
            },
        );
        let shared_resources =
            builder.src.join("ferrocene").join("doc").join("sphinx-shared-resources");
        let substitutions =
            builder.src.join("ferrocene").join("doc").join("sphinx-substitutions.toml");
        let breadcrumbs = builder.src.join("ferrocene").join("doc").join("breadcrumbs");

        // In some cases we have to perform a fresh build to guarantee deterministic output (for
        // example to generate signatures). We want to purge the old build artifacts only when
        // necessary, to avoid thrashing incremental builds.
        if self.fresh_build {
            for path in [&out, &doctrees] {
                if path.exists() {
                    builder.remove_dir(path);
                }
            }
        }

        builder.ensure(BreadcrumbsAssets { target: self.target, dest: Some(out.join("_static")) });
        let venv = builder.ensure(SphinxVirtualEnv { target: self.target });

        let path_to_root = std::iter::repeat("..")
            .take(self.dest.chars().filter(|c| *c == '/').count() + 1)
            .collect::<Vec<_>>()
            .join("/");

        let should_serve = self.parent.is_some() && builder.should_serve::<P>();

        // Note that we must pass all paths to Sphinx relative to the directory containing conf.py.
        // Absolute paths break our reproducibility, and paths relative from other directories
        // don't really work with Sphinx, as it treats all paths as relative from the directory
        // containing conf.py even if a different current directory is passed.
        let mut cmd = venv.cmd(if should_serve { "sphinx-autobuild" } else { "sphinx-build" });
        cmd.current_dir(&src)
            .arg(relative_path(&src, &src))
            .arg(relative_path(&src, &out))
            // Build in parallel
            .args(&["-j", "auto"])
            // Store doctrees outside the output directory:
            .arg("-d")
            .arg(relative_path(&src, &doctrees))
            // Load the theme from the Sphinx shared resources:
            .arg("-D")
            .arg(path_define(
                "html_theme_path",
                &relative_path(&src, &shared_resources.join("themes")),
            ))
            // Include the breadcrumbs
            .arg("-D")
            .arg(path_define(
                "html_theme_options.include_in_header",
                &relative_path(&src, &breadcrumbs.join("sphinx-template.html")),
            ))
            .arg("-D")
            .arg(format!("html_css_files=ferrocene-breadcrumbs.css"))
            .arg("-A")
            .arg(format!("ferrocene_breadcrumbs_index={path_to_root}/index.html"))
            // Provide the correct substitutions:
            .arg("-D")
            .arg(path_define("ferrocene_substitutions_path", &relative_path(&src, &substitutions)))
            // Toolchain versions
            .arg("-D")
            .arg(format!(
                "ferrocene_version={}",
                std::fs::read_to_string(&builder.src.join("ferrocene").join("version")).unwrap()
            ))
            .arg("-D")
            .arg(format!(
                "rust_version={}",
                std::fs::read_to_string(&builder.src.join("src").join("version")).unwrap(),
            ))
            // Load extensions from the shared resources as well:
            .env("PYTHONPATH", relative_path(&src, &shared_resources.join("exts")));

        // We use InterSphinx to add links between books, which requires the inventory files
        // (object.inv) of the other books to be available before building. Unfortunately, there
        // are circular dependencies between books.
        //
        // To solve them, we first "gather" the object.inv files by building all the documentation
        // with a special Sphinx builder that doesn't write any other output file, and then we
        // do a full build of the documents we want.
        if self.only_gather_objects_inv {
            builder.info(&format!("Gathering references of {}", self.src));

            // When gathering the objects.inv files, we use a custom Sphinx builder that only
            // outputs the objects.inv file.
            cmd.args(&["-b", "ferrocene-intersphinx"]);
        } else {
            for step in intersphinx_gather_steps(self.target) {
                builder.ensure(step);
            }

            builder.info(&format!("Building {}", self.src));

            cmd.args(&["-b", "html"]);
            // Warn about missing references:
            cmd.arg("-n");
            // Fail the build if there are warnings (but don't abort at the first warning):
            if !should_serve {
                cmd.args(&["-W", "--keep-going"]);
            }
        };

        add_intersphinx_arguments(&self, builder, &src, &mut cmd);

        if self.inject_all_other_document_ids {
            for (document, file) in gather_other_document_ids(builder, self.target).into_iter() {
                if file.exists() {
                    cmd.env(format!("FERROCENE_DOCUMENT_ID_{document}"), file);
                }
            }
        }

        match self.signature {
            // Always treat a document as not needing a signature if signatures are ignored.
            _ if builder.config.ferrocene_ignore_document_signatures => {}
            SignatureStatus::NotNeeded => {}

            SignatureStatus::Present => {
                let private_signature_files_dir =
                    builder.ensure(CacheSignatureFiles { source_dir: builder.src.join(&self.src) });

                cmd.args(["-D", "ferrocene_signature=present"]);
                // Provide the directory containing the cached private signature files:
                cmd.arg("-D").arg(path_define(
                    "ferrocene_private_signature_files_dir",
                    &relative_path(&src, &private_signature_files_dir),
                ));
            }
            SignatureStatus::Missing => {
                cmd.args(["-D", "ferrocene_signature=missing"]);
            }
        }

        if let Some(test_outcomes_dir) = &builder.config.ferrocene_test_outcomes_dir {
            cmd.env(
                "FERROCENE_TEST_OUTCOMES_DIR",
                std::fs::canonicalize(test_outcomes_dir).unwrap(),
            );
        }

        if should_serve && builder.config.cmd.open() {
            cmd.arg("--open-browser");
        }

        builder.run(&mut cmd);

        if !should_serve {
            builder.maybe_open_in_browser::<P>(&out.join("index.html"));
        }
        out
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum SignatureStatus {
    Present,
    Missing,
    NotNeeded,
}

fn add_intersphinx_arguments<P: Step>(
    book: &SphinxBook<P>,
    builder: &Builder<'_>,
    src: &Path,
    cmd: &mut Command,
) {
    #[derive(serde::Serialize)]
    struct Inventory {
        name: String,
        html_root: String,
        inventory: PathBuf,
    }

    let path_to_root = std::iter::repeat("..")
        .take(book.dest.chars().filter(|c| *c == '/').count() + 1)
        .collect::<Vec<_>>()
        .join("/");

    let empty_objects_inv = builder.src.join("ferrocene").join("etc").join("empty-objects.inv");

    let mut inventories = Vec::new();
    for step in intersphinx_gather_steps(book.target) {
        let html_root = format!("{path_to_root}/{}", step.dest);

        // To avoid warnings due to missing objects.inv files when gathering them, we provide all
        // the mappings even during gathering. Since not all objects.inv will be available during
        // gathering, all of them are replaced with an empty objects.inv file during gathering.
        let inv = if book.only_gather_objects_inv {
            empty_objects_inv.clone()
        } else {
            builder.doc_out(book.target).join(&step.dest).join("objects.inv")
        };

        inventories.push(Inventory {
            name: step.name,
            html_root,
            inventory: relative_path(src, &inv),
        });
    }

    // We cannot set the intersphinx_mapping configuration directly from the CLI (with the -D flag)
    // since the intersphinx_mapping type is too complex to be set with it (namely, tuples are not
    // supported). To work around the issue, the ferrocene_intersphinx_support extension has a
    // configuration key we can set, that accepts the JSON representation of the mappings. The
    // extension then takes care of registering the mappings with InterSphinx.
    let serialized = serde_json::to_string(&inventories).unwrap();
    cmd.arg("-D").arg(format!("ferrocene_intersphinx_mappings={serialized}"));
}

fn path_define(key: &str, value: &Path) -> OsString {
    let mut string = OsString::new();
    string.push(key);
    string.push("=");
    string.push(value);
    string
}

macro_rules! sphinx_books {
    ($({
        ty: $ty:ident,
        name: $name:expr,
        src: $src:expr,
        dest: $dest:expr,
        $(inject_all_other_document_ids: $inject_all_other_document_ids:expr,)?
    },)*) => {
        $(
            #[derive(Debug, PartialEq, Eq, Hash, Clone)]
            pub(crate) struct $ty {
                pub(crate) target: TargetSelection,
                pub(crate) fresh_build: bool,
            }

            impl Step for $ty {
                type Output = PathBuf;
                const DEFAULT: bool = true;

                fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                    let builder = run.builder;
                    run.path($src).default_condition(builder.config.docs)
                }

                fn make_run(run: RunConfig<'_>) {
                    run.builder.ensure(Self {
                        target: run.target,
                        fresh_build: false,
                    });
                }

                fn run(self, builder: &Builder<'_>) -> Self::Output {
                    let signature_dir = builder.src
                        .join($src)
                        .join("signature");
                    let signature = if signature_dir.join("config.toml").is_file() {
                        if signature_dir.join("signature.toml").is_file() {
                            SignatureStatus::Present
                        } else {
                            SignatureStatus::Missing
                        }
                    } else {
                        SignatureStatus::NotNeeded
                    };

                    #[allow(unused_mut, unused_assignments)]
                    let mut inject_all_other_document_ids = false;
                    $(inject_all_other_document_ids = $inject_all_other_document_ids;)*

                    builder.ensure(SphinxBook {
                        target: self.target,
                        name: $name.into(),
                        src: $src.into(),
                        dest: $dest.into(),
                        only_gather_objects_inv: false,
                        fresh_build: self.fresh_build,
                        signature,
                        inject_all_other_document_ids,
                        parent: Some(self),
                    })
                }
            }

            impl WithSource for $ty {
                const SOURCE: &'static str = $src;
            }
        )*

        fn intersphinx_gather_steps(target: TargetSelection) -> Vec<SphinxBook> {
            let mut steps = Vec::new();
            $(
                steps.push(SphinxBook {
                    target,
                    name: $name.into(),
                    src: $src.into(),
                    dest: $dest.into(),
                    only_gather_objects_inv: true,
                    fresh_build: false,
                    signature: SignatureStatus::NotNeeded,
                    inject_all_other_document_ids: false,
                    parent: None,
                });
            )*
            steps
        }

        fn gather_other_document_ids(builder: &Builder<'_>, target: TargetSelection) -> HashMap<&'static str, PathBuf> {
            let mut document_ids = HashMap::new();

            $(
                #[allow(unused_mut, unused_assignments)]
                let mut inject_all_other_document_ids = false;
                $(inject_all_other_document_ids = $inject_all_other_document_ids;)*

                // Avoid recursive builds.
                if !inject_all_other_document_ids {
                    document_ids.insert(
                        $name,
                        builder.ensure($ty {
                            target,
                            fresh_build: false,
                        }).join("document-id.txt"),
                    );
                }
            )*

            document_ids
        }
    };
}

sphinx_books! [
    // Basic Documents
    {
        ty: Specification,
        name: "specification",
        src: "ferrocene/doc/specification",
        dest: "specification",
    },
    {
        ty: UserManual,
        name: "user-manual",
        src: "ferrocene/doc/user-manual",
        dest: "user-manual",
    },
    // Qualification Documents
    {
        ty: DocumentList,
        name: "document-list",
        src: "ferrocene/doc/document-list",
        dest: "qualification/document-list",
        inject_all_other_document_ids: true,
    },
    {
        ty: EvaluationPlan,
        name: "evaluation-plan",
        src: "ferrocene/doc/evaluation-plan",
        dest: "qualification/evaluation-plan",
    },
    {
        ty: EvaluationReport,
        name: "evaluation-report",
        src: "ferrocene/doc/evaluation-report",
        dest: "qualification/evaluation-report",
    },
    {
        ty: QualificationPlan,
        name: "qualification-plan",
        src: "ferrocene/doc/qualification-plan",
        dest: "qualification/plan",
    },
    {
        ty: QualificationReport,
        name: "qualification-report",
        src: "ferrocene/doc/qualification-report",
        dest: "qualification/report",
    },
    {
        ty: SafetyManual,
        name: "safety-manual",
        src: "ferrocene/doc/safety-manual",
        dest: "safety-manual",
    },
    {
        ty: InternalProcedures,
        name: "internal-procedures",
        src: "ferrocene/doc/internal-procedures",
        dest: "qualification/internal-procedures",
    },
];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct TraceabilityMatrix {
    target: TargetSelection,
}

impl Step for TraceabilityMatrix {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.path("ferrocene/tools/traceability-matrix").default_condition(builder.config.docs)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TraceabilityMatrix { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.ensure(crate::ferrocene::run::TraceabilityMatrix { target: self.target });
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct TechnicalReport {
    target: TargetSelection,
}

impl Step for TechnicalReport {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.path("ferrocene-technical-report").default_condition(
            builder.config.docs && builder.config.ferrocene_technical_report_url.is_some(),
        )
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(TechnicalReport { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let url = builder
            .config
            .ferrocene_technical_report_url
            .as_deref()
            .expect("ferrocene.technical-report-url is not configured");
        let cache_path = builder
            .out
            .join("cache")
            .join("ferrocene")
            .join(url.rsplit_once('/').map(|(_, name)| name).unwrap_or(url));
        let output_dir = builder.doc_out(self.target).join("qualification");

        if builder.config.dry_run() {
            return;
        }

        if !cache_path.exists() {
            if let Some(parent) = cache_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            builder.config.download_file(url, &cache_path, "");
        }

        let output_file = output_dir.join("technical-report.pdf");

        builder.create_dir(&output_dir);
        builder.copy(&cache_path, &output_file);

        // Include the technical report only in the signatures subset.
        let mut subset_file = output_file.into_os_string();
        subset_file.push(".ferrocene-subset");
        builder.create(&PathBuf::from(subset_file), "signatures");
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Index {
    target: TargetSelection,
}

impl Step for Index {
    type Output = ();
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        let builder = run.builder;
        run.path("ferrocene/doc/index").default_condition(builder.config.docs)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Index { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.ensure(BreadcrumbsAssets { target: self.target, dest: None });
        builder.cp_r(
            &builder.src.join("ferrocene").join("doc").join("index"),
            &builder.out.join(self.target.triple).join("doc"),
        );
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct EmptyStep;

impl Step for EmptyStep {
    type Output = ();

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, _: &Builder<'_>) -> Self::Output {}
}

// Note: this function is correct for the use made in this module, but it will not work correctly
// if paths don't exists or the paths do not have any segments in common.
fn relative_path(base: &Path, path: &Path) -> PathBuf {
    let base = std::fs::canonicalize(base).unwrap_or_else(|_| base.to_path_buf());
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    let common = base
        .components()
        .zip(path.components())
        .filter(|(a, b)| a == b)
        .fuse() // Stop at the first non-equal component
        .map(|(a, _)| a)
        .collect::<PathBuf>();

    let mut result = PathBuf::new();
    for _ in base.strip_prefix(&common).unwrap() {
        result.push("..");
    }
    for component in path.strip_prefix(&common).unwrap() {
        result.push(component);
    }

    if result.components().count() == 0 { PathBuf::from(".") } else { result }
}
