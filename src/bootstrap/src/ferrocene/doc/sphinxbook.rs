// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod breadcrumbassets;
mod emptystep;
mod index;
mod makro;
mod sphinxvirtualenv;

use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::sign::CacheSignatureFiles;
use crate::ferrocene::test_outcomes::TestOutcomesDir;

pub(crate) use self::index::Index;
use self::{
    breadcrumbassets::BreadcrumbsAssets, emptystep::EmptyStep, makro::sphinx_books,
    sphinxvirtualenv::SphinxVirtualEnv,
};

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
];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SphinxBook<P: Step = EmptyStep> {
    mode: SphinxMode,
    target: TargetSelection,
    name: String,
    src: String,
    dest: String,
    fresh_build: bool,
    signature: SignatureStatus,
    inject_all_other_document_ids: bool,
    require_test_outcomes: bool,
    require_relnotes: bool,
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
        let out = match self.mode {
            SphinxMode::Html => builder.out.join(self.target.triple).join("doc").join(&self.dest),
            SphinxMode::OnlyObjectsInv => builder
                .out
                .join(self.target.triple)
                .join("ferrocene")
                .join("objectsinv-out")
                .join(&self.dest),
            SphinxMode::XmlDoctrees => builder
                .out
                .join(self.target.triple)
                .join("ferrocene")
                .join("doctrees-out")
                .join(&self.dest),
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
            .take(self.dest.chars().filter(|c| *c == '/').count() + 1)
            .collect::<Vec<_>>()
            .join("/");

        let should_serve = self.parent.is_some() && builder.should_serve::<P>();

        let ferrocene_version =
            std::fs::read_to_string(builder.src.join("ferrocene").join("version")).unwrap();
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
            .arg("html_css_files=ferrocene-breadcrumbs.css")
            .arg("-A")
            .arg(format!("ferrocene_breadcrumbs_index={path_to_root}/index.html"))
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
                std::fs::read_to_string(builder.src.join("src").join("version")).unwrap().trim(),
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

        match self.mode {
            SphinxMode::Html => {
                for step in intersphinx_gather_steps(self.target) {
                    builder.ensure(step);
                }

                builder.info(&format!("Building {}", self.src));

                cmd.args(["-b", "html"]);
                // Warn about missing references:
                cmd.arg("-n");
                // Fail the build if there are warnings (but don't abort at the first warning):
                if !should_serve {
                    cmd.args(["-W", "--keep-going"]);
                }
            }
            SphinxMode::XmlDoctrees => {
                for step in intersphinx_gather_steps(self.target) {
                    builder.ensure(step);
                }

                builder.info(&format!("Building XML doctrees of {}", self.src));
                cmd.args(["-b", "xml"]);

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
                builder.info(&format!("Gathering references of {}", self.src));

                // When gathering the objects.inv files, we use a custom Sphinx builder that only
                // outputs the objects.inv file.
                cmd.args(["-b", "ferrocene-intersphinx"]);
            }
        }

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

        if self.require_test_outcomes {
            if let Some(path) = builder.ensure(TestOutcomesDir) {
                cmd.env("FERROCENE_TEST_OUTCOMES_DIR", path);
            }
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

pub(crate) trait WithSource {
    const SOURCE: &'static str;
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

fn add_intersphinx_arguments<P: Step>(
    book: &SphinxBook<P>,
    builder: &Builder<'_>,
    src: &Path,
    cmd: &mut Command,
) {
    #[derive(serde_derive::Serialize)]
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
        let inv = match book.mode {
            SphinxMode::Html | SphinxMode::XmlDoctrees => builder
                .out
                .join(book.target.triple)
                .join("ferrocene")
                .join("objectsinv-out")
                .join(&step.dest)
                .join("objects.inv"),
            SphinxMode::OnlyObjectsInv => empty_objects_inv.clone(),
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
