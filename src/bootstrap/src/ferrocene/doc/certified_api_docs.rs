use std::path::PathBuf;

use crate::core::build_steps::doc;
use crate::core::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct CertifiedApiDocs {
    pub(crate) target: TargetSelection,
}

impl Step for CertifiedApiDocs {
    type Output = PathBuf;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-certified-api-docs")
    }

    fn make_run(run: RunConfig<'_>) {
        if run.target.try_subset_equivalent().is_some() {
            run.builder.ensure(CertifiedApiDocs { target: run.target });
        } else if run.builder.was_invoked_explicitly::<CertifiedApiDocs>(crate::Kind::Doc) {
            panic!(
                "Could not build certified API docs, no certified target exists for {}",
                run.target.to_string()
            )
        } else {
            run.builder.info(&format!("No certified target for {}", run.target.to_string()));
        }
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let certified_crates = vec!["core".into()];
        let certified_target = self.target.certified_equivalent();

        // Build the docs for the certified target
        let certified_target_doc_out = builder.ensure(
            doc::Std::from_build_compiler(
                builder.compiler(builder.top_stage, builder.host_target),
                certified_target,
                doc::DocumentationFormat::Html,
            )
            .with_crates(certified_crates),
        );

        // Remove unwanted files/dirs
        builder.remove(&certified_target_doc_out.join("index.html"));

        // Copy the files from the certified target to the host target
        let host_target_doc_out = builder.doc_out(self.target).join("certification/api-docs");
        builder.create_dir(&host_target_doc_out);
        builder.cp_link_r(&certified_target_doc_out, &host_target_doc_out);

        host_target_doc_out
    }
}
