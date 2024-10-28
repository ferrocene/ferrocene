// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf, absolute};

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::sign::signature_files::CacheSignatureFiles;
use crate::ferrocene::test_outcomes::TestOutcomesDir;
use crate::utils::exec::BootstrapCommand;

pub(crate) trait IsSphinxBook {
    const SOURCE: &'static str;
    const DEST: &'static str;
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
        builder.copy_link(&src, &dest);
    }
}

#[derive(Clone)]
struct VirtualEnv {
    path: PathBuf,
}

impl VirtualEnv {
    fn cmd(&self, bin: &str) -> BootstrapCommand {
        #[cfg(not(target_os = "windows"))]
        const BIN_DIR: &str = "bin";
        #[cfg(target_os = "windows")]
        const BIN_DIR: &str = "scripts";

        BootstrapCommand::new(self.path.join(BIN_DIR).join(bin))
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
        BootstrapCommand::new(
            builder
                .config
                .python
                .as_ref()
                .expect("Python is required to build Sphinx documentation"),
        )
        .args(&["-m", "venv"])
        .arg(&venv)
        .run(builder);
        let venv = VirtualEnv { path: venv };
        BootstrapCommand::from(venv.cmd("pip"))
            .args(&["install", "--require-hashes", "-r"])
            .arg(&requirements)
            .run(builder);
        builder.copy_link(&requirements, &installed_requirements);

        venv
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SphinxBook<P: Step + IsSphinxBook> {
    mode: SphinxMode,
    target: TargetSelection,
    name: String,
    fresh_build: bool,
    signature: SignatureStatus,
    inject_all_other_document_ids: bool,
    require_test_outcomes: bool,
    require_relnotes: bool,
    parent: P,
}

impl<P: Step + IsSphinxBook> Step for SphinxBook<P> {
    type Output = PathBuf;
    const DEFAULT: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let src = builder.src.join(P::SOURCE).join("src");
        let out = match self.mode {
            SphinxMode::Html => builder.out.join(self.target.triple).join("doc").join(P::DEST),
            SphinxMode::OnlyObjectsInv => builder
                .out
                .join(self.target.triple)
                .join("ferrocene")
                .join("objectsinv-out")
                .join(P::DEST),
            SphinxMode::XmlDoctrees => builder
                .out
                .join(self.target.triple)
                .join("ferrocene")
                .join("doctrees-out")
                .join(P::DEST),
        };
        let doctrees =
            builder.out.join(self.target.triple).join("ferrocene").join(match self.mode {
                SphinxMode::Html | SphinxMode::XmlDoctrees => {
                    format!("{}-doctrees", self.name)
                }
                SphinxMode::OnlyObjectsInv => {
                    format!("{}-doctrees-objectsinv", self.name)
                }
            });
        let shared_resources =
            builder.src.join("ferrocene").join("doc").join("sphinx-shared-resources");
        let substitutions =
            builder.src.join("ferrocene").join("doc").join("sphinx-substitutions.toml");
        let target_names = builder.src.join("ferrocene").join("doc").join("target-names.toml");
        let breadcrumbs = builder.src.join("ferrocene").join("doc").join("breadcrumbs");

        // In some cases we have to perform a fresh build to guarantee deterministic output (for
        // example to generate signatures). We want to purge the old build artifacts only when
        // necessary, to avoid thrashing incremental builds.
        if self.fresh_build || builder.config.cmd.fresh() {
            for path in [&out, &doctrees] {
                if path.exists() {
                    builder.remove_dir(path);
                }
            }
        }

        if let SphinxMode::Html = self.mode {
            builder
                .ensure(BreadcrumbsAssets { target: self.target, dest: Some(out.join("_static")) });
        }
        let venv = builder.ensure(SphinxVirtualEnv { target: self.target });

        let path_to_root = std::iter::repeat("..")
            .take(P::DEST.chars().filter(|c| *c == '/').count() + 1)
            .collect::<Vec<_>>()
            .join("/");

        let should_serve = match self.mode {
            SphinxMode::Html => builder.should_serve::<P>(),
            SphinxMode::XmlDoctrees => false,
            SphinxMode::OnlyObjectsInv => false,
        };

        let ferrocene_version =
            fs::read_to_string(&builder.src.join("ferrocene").join("version")).unwrap();
        let ferrocene_version = ferrocene_version.trim();

        // Note that we must pass all paths to Sphinx relative to the directory containing conf.py.
        // Absolute paths break our reproducibility, and paths relative from other directories
        // don't really work with Sphinx, as it treats all paths as relative from the directory
        // containing conf.py even if a different current directory is passed.
        let mut cmd = venv.cmd(if should_serve { "sphinx-autobuild" } else { "sphinx-build" });
        cmd.current_dir(&src)
            .arg(relative_path(&src, &src))
            .arg(relative_path(&src, &out))
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
            .arg("-D")
            .arg(format!(
                "rustfmt_version={}",
                builder.crates.get("rustfmt-nightly").unwrap().version,
            ))
            // Provide the correct substitutions:
            .arg("-D")
            .arg(path_define("ferrocene_substitutions_path", &relative_path(&src, &substitutions)))
            // Provide the correct target names:
            .arg("-D")
            .arg(path_define("ferrocene_target_names_path", &relative_path(&src, &target_names)))
            // Toolchain versions
            .arg("-D")
            .arg(format!("ferrocene_version={ferrocene_version}"))
            .arg("-D")
            .arg(format!(
                "rust_version={}",
                fs::read_to_string(&builder.src.join("src").join("version")).unwrap().trim(),
            ))
            // Load extensions from the shared resources as well:
            .env("PYTHONPATH", relative_path(&src, &shared_resources.join("exts")));

        if builder.config.cmd.fresh() {
            // The `-E` flag forces Sphinx to ignore any saved environment and build everything
            // from scratch. This is not strictly required during normal builds or initial builds
            // with --serve, as code above already clears the directory before invoking Sphinx.
            //
            // Without this code, followup builds of --serve would be incremental rather than
            // fresh, as our code to delete directories only runs once. Passing `-E` fixes that.
            cmd.arg("-E");
        }

        if self.require_relnotes {
            cmd.arg("-D").arg(path_define(
                "rust_release_notes",
                &relative_path(&src, &builder.src.join("RELEASES.md")),
            ));
        }

        if builder.config.cmd.debug_sphinx() {
            cmd
                // Only run one parallel job, as Sphinx occasionally cannot show the error message
                // with the parallel backend.
                .args(["-j", "1"])
                // Show full traceback on exceptions.
                .arg("-T");
        } else {
            // Build in parallel
            cmd.args(["-j", "auto"]);
        }

        let mut allow_injecting_ids = true;
        match self.mode {
            SphinxMode::Html => {
                intersphinx_ensure_steps(builder, self.target);
                add_external_sphinx_needs_argument(&self, builder, &src, &mut cmd);

                builder.info(&format!("Building {}", P::SOURCE));

                cmd.args(&["-b", "html"]);
                // Warn about missing references:
                cmd.arg("-n");
                // Fail the build if there are warnings (but don't abort at the first warning):
                if !should_serve {
                    cmd.args(&["-W", "--keep-going"]);
                }
            }
            SphinxMode::XmlDoctrees => {
                intersphinx_ensure_steps(builder, self.target);
                add_external_sphinx_needs_argument(&self, builder, &src, &mut cmd);

                builder.info(&format!("Building XML doctrees of {}", P::SOURCE));
                cmd.args(&["-b", "xml"]);

                // This is intentionally more lax than HTML builds, especially around warnings.
                // This will never be executed by CI if the HTML build fails anyways.
            }
            SphinxMode::OnlyObjectsInv => {
                // We use InterSphinx to add links between books, which requires the inventory files
                // (object.inv) of the other books to be available before building. Unfortunately, there
                // are circular dependencies between books.
                //
                // To solve them, we first "gather" the object.inv files by building all the documentation
                // with a special Sphinx builder that doesn't write any other output file, and then we
                // do a full build of the documents we want.
                builder.info(&format!("Gathering references of {}", P::SOURCE));

                // When gathering the objects.inv files, we use a custom Sphinx builder that only
                // outputs the objects.inv file.
                cmd.args(&["-b", "ferrocene-intersphinx"]);

                // Avoid circular dependencies.
                allow_injecting_ids = false;
            }
        }

        add_intersphinx_arguments(&self, builder, &src, &mut cmd);

        if self.inject_all_other_document_ids && allow_injecting_ids {
            for (document, file) in gather_other_document_ids(builder, self.target).into_iter() {
                if file.exists() {
                    cmd.env(format!("FERROCENE_DOCUMENT_ID_{document}"), file);
                }
            }
        }

        match (&builder.config.ferrocene_document_signatures, self.signature) {
            // Always treat a document as not needing a signature if signatures are ignored.
            (crate::core::config::FerroceneDocumentSignatures::Disabled, _) => {}

            (_, SignatureStatus::NotNeeded) => {}

            (_, SignatureStatus::Present) => {
                let private_signature_files_dir = builder.ensure(CacheSignatureFiles::<P>::new());

                cmd.args(["-D", "ferrocene_signature=present"]);
                // Provide the directory containing the cached private signature files:
                cmd.arg("-D").arg(path_define(
                    "ferrocene_private_signature_files_dir",
                    &relative_path(&src, &private_signature_files_dir),
                ));
            }
            (_, SignatureStatus::Missing) => {
                cmd.args(["-D", "ferrocene_signature=missing"]);
            }
        }

        if self.require_test_outcomes {
            if let Some(path) = builder.ensure(TestOutcomesDir) {
                cmd.env("FERROCENE_TEST_OUTCOMES_DIR", path);
            }
        }

        if should_serve && builder.config.cmd.open() {
            cmd.arg("--open-browser");
        }

        cmd.run(builder);

        if !should_serve {
            builder.maybe_open_in_browser::<P>(&out.join("index.html"));
        }
        out
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum SphinxMode {
    Html,
    XmlDoctrees,
    OnlyObjectsInv,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum SignatureStatus {
    Present,
    Missing,
    NotNeeded,
}

fn add_intersphinx_arguments<P: Step + IsSphinxBook>(
    book: &SphinxBook<P>,
    builder: &Builder<'_>,
    src: &Path,
    cmd: &mut BootstrapCommand,
) {
    #[derive(serde_derive::Serialize)]
    struct Inventory {
        name: String,
        html_root: String,
        inventory: PathBuf,
    }

    let path_to_root = std::iter::repeat("..")
        .take(P::DEST.chars().filter(|c| *c == '/').count() + 1)
        .collect::<Vec<_>>()
        .join("/");

    let empty_objects_inv = builder.src.join("ferrocene").join("etc").join("empty-objects.inv");

    let mut inventories = Vec::new();
    for config in intersphinx_configs() {
        let html_root = format!("{path_to_root}/{}", config.dest);

        // To avoid warnings due to missing objects.inv files when gathering them, we provide all
        // the mappings even during gathering. Since not all objects.inv will be available during
        // gathering, all of them are replaced with an empty objects.inv file during gathering.
        let inv = match book.mode {
            SphinxMode::Html | SphinxMode::XmlDoctrees => builder
                .out
                .join(book.target.triple)
                .join("ferrocene")
                .join("objectsinv-out")
                .join(config.dest)
                .join("objects.inv"),
            SphinxMode::OnlyObjectsInv => empty_objects_inv.clone(),
        };

        inventories.push(Inventory {
            name: config.name.into(),
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

fn add_external_sphinx_needs_argument<P: Step + IsSphinxBook>(
    book: &SphinxBook<P>,
    builder: &Builder<'_>,
    src: &Path,
    cmd: &mut BootstrapCommand,
) {
    #[derive(serde_derive::Serialize)]
    struct ExternalNeed {
        base_url: String,
        json_path: PathBuf,
    }

    #[derive(serde_derive::Deserialize)]
    struct NeedsJson {
        versions: HashMap<String, serde_json::Value>,
    }

    match book.mode {
        SphinxMode::Html | SphinxMode::XmlDoctrees => {}
        SphinxMode::OnlyObjectsInv => return,
    }
    if builder.config.dry_run() {
        return;
    }

    let path_to_root = std::iter::repeat("..")
        .take(P::DEST.chars().filter(|c| *c == '/').count() + 1)
        .collect::<Vec<_>>()
        .join("/");

    let mut needs = Vec::new();
    for config in intersphinx_configs() {
        // Sphinx-needs complains if you provide the same document's needs.json as external.
        if config.dest == P::DEST {
            continue;
        }

        let json_path = builder
            .out
            .join(book.target.triple)
            .join("ferrocene")
            .join("objectsinv-out")
            .join(config.dest)
            .join("needs.json");

        // When there are no requirements defined in a document, sphinx-needs still generates a
        // needs.json file, but it doesn't contain information about any version. This results in
        // the sphinx-needs extension then failing to load the external needs.json, as it expects
        // the version to be there. We thus parse the files before including them in the array to
        // discard any empty one.
        let json_raw = std::fs::read(&json_path).unwrap();
        let json_contents: NeedsJson = serde_json::from_slice(&json_raw).unwrap();
        if json_contents.versions.is_empty() {
            continue;
        }

        needs.push(ExternalNeed {
            base_url: format!("{path_to_root}/{}", config.dest),
            json_path: relative_path(src, &json_path),
        });
    }

    let serialized = serde_json::to_string(&needs).unwrap();
    cmd.arg("-D").arg(format!("ferrocene_external_needs={serialized}"));
}

fn path_define(key: &str, value: &Path) -> OsString {
    let mut string = OsString::new();
    string.push(key);
    string.push("=");
    string.push(value);
    string
}

struct InterSphinxConfig {
    name: &'static str,
    dest: &'static str,
}

macro_rules! sphinx_books {
    ($({
        ty: $ty:ident,
        name: $name:expr,
        src: $src:expr,
        dest: $dest:expr,
        $(inject_all_other_document_ids: $inject_all_other_document_ids:expr,)?
        $(require_test_outcomes: $require_test_outcomes:expr,)?
        $(require_relnotes: $require_relnotes:expr,)?
    },)*) => {
        $(
            #[derive(Debug, PartialEq, Eq, Hash, Clone)]
            pub(crate) struct $ty {
                pub(crate) mode: SphinxMode,
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
                        mode: SphinxMode::Html,
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

                    #[allow(unused_mut, unused_assignments)]
                    let mut require_test_outcomes = false;
                    $(require_test_outcomes = $require_test_outcomes;)*

                    #[allow(unused_mut, unused_assignments)]
                    let mut require_relnotes = false;
                    $(require_relnotes = $require_relnotes;)*

                    builder.ensure(SphinxBook {
                        mode: self.mode,
                        target: self.target,
                        name: $name.into(),
                        fresh_build: self.fresh_build,
                        signature,
                        inject_all_other_document_ids,
                        require_test_outcomes,
                        require_relnotes,
                        parent: self,
                    })
                }
            }

            impl IsSphinxBook for $ty {
                const SOURCE: &'static str = $src;
                const DEST: &'static str = $dest;
            }
        )*

        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        pub(crate) struct AllSphinxDocuments {
            pub(crate) target: TargetSelection,
        }

        impl Step for AllSphinxDocuments {
            type Output = ();
            const DEFAULT: bool = false;

            fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
                run.path("ferrocene/doc")
            }

            fn make_run(run: RunConfig<'_>) {
                run.builder.ensure(AllSphinxDocuments {
                    target: run.target,
                });
            }

            fn run(self, builder: &Builder<'_>) {
                $(
                    builder.ensure($ty {
                        mode: SphinxMode::Html,
                        target: self.target,
                        fresh_build: false,
                    });
                )*

                // Also regenerate the index file, so that the "Ferrocene documentation" link in
                // the breadcrumbs doesn't break.
                builder.ensure(Index { target: self.target });
            }
        }

        fn intersphinx_ensure_steps(builder: &Builder<'_>, target: TargetSelection) {
            $(
                builder.ensure($ty {
                    mode: SphinxMode::OnlyObjectsInv,
                    target,
                    fresh_build: false,
                });
            )*
        }

        fn intersphinx_configs() -> Vec<InterSphinxConfig> {
            let mut configs = Vec::new();
            $(
                configs.push(InterSphinxConfig {
                    name: $name,
                    dest: $ty::DEST,
                });
            )*
            configs
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
                            mode: SphinxMode::Html,
                            target,
                            fresh_build: false,
                        }).join("document-id.txt"),
                    );
                }
            )*

            document_ids
        }

        pub(crate) fn ensure_all_xml_doctrees(
            builder: &Builder<'_>,
            target: TargetSelection,
        ) -> HashMap<&'static str, PathBuf> {
            let mut paths = HashMap::new();
            $(paths.insert(
                $name,
                builder.ensure($ty {
                    mode: SphinxMode::XmlDoctrees,
                    target,
                    fresh_build: false,
                })
            );)*
            paths
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
    {
        ty: ReleaseNotes,
        name: "release-notes",
        src: "ferrocene/doc/release-notes",
        dest: "release-notes",
        require_relnotes: true,
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
        require_test_outcomes: true,
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
    {
        ty: Requirements,
        name: "requirements",
        src: "ferrocene/doc/requirements",
        dest: "requirements",
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
        run.alias("ferrocene-technical-report").default_condition(
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
                fs::create_dir_all(parent).unwrap();
            }
            builder.config.download_file(url, &cache_path, "");
        }

        let mut output_file = output_dir.join("technical-report.pdf");

        builder.create_dir(&output_dir);
        builder.copy_link(&cache_path, &output_file);

        // Include the technical report file only in the signatures subset.
        output_file.as_mut_os_string().push(".ferrocene-subset");
        builder.create(&output_file, "signatures");
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
        builder.cp_link_r(
            &builder.src.join("ferrocene").join("doc").join("index"),
            &builder.out.join(self.target.triple).join("doc"),
        );
    }
}

// Note: this function is correct for the use made in this module, but it will not work correctly
// if the paths do not have any segments in common.
fn relative_path(base: &Path, path: &Path) -> PathBuf {
    let base = absolute(base).unwrap_or_else(|_| base.to_owned());
    let path = absolute(path).unwrap_or_else(|_| path.to_owned());

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
