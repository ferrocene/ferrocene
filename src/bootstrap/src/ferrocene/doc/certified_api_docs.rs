use std::path::PathBuf;

use crate::core::build_steps::doc::{self, DocumentationFormat};
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct CertifiedApiDocs {
    pub(crate) target: TargetSelection,
    pub(crate) format: DocumentationFormat,
}

impl Step for CertifiedApiDocs {
    type Output = PathBuf;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-certified-api-docs")
    }

    fn make_run(run: RunConfig<'_>) {
        let format = if run.builder.config.cmd.json() {
            DocumentationFormat::Json
        } else {
            DocumentationFormat::Html
        };
        run.builder.ensure(CertifiedApiDocs { target: run.target, format });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let certified_crates = vec!["core".into()];
        let certified_target = self
            .target
            .certified_equivalent()
            .expect(&format!("no certified equivalent exists for target \"{}\"", &self.target));

        // Build the docs for the certified target
        let certified_target_doc_out = builder.ensure(
            doc::Std::from_build_compiler(
                builder.compiler_for_std(builder.top_stage),
                certified_target,
                self.format,
            )
            .with_crates(certified_crates),
        );

        let doc_out_name = match self.format {
            DocumentationFormat::Html => {
                // Remove unwanted files/dirs
                builder.remove(&certified_target_doc_out.join("index.html"));
                "api-docs"
            }
            DocumentationFormat::Json => "api-docs-json",
        };

        // Copy the files from the certified target to the host target
        let host_target_doc_out =
            builder.doc_out(self.target).join("certification").join(doc_out_name);
        builder.create_dir(&host_target_doc_out);
        builder.cp_link_r(&certified_target_doc_out, &host_target_doc_out);

        host_target_doc_out
    }
}
