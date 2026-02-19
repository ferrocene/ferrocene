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

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-certified-api-docs")
    }

    fn make_run(run: RunConfig<'_>) {
        if run.target.has_certified_subset() {
            run.builder.ensure(CertifiedApiDocs { target: run.target });
        } else if run.builder.was_invoked_explicitly::<CertifiedApiDocs>(crate::Kind::Doc) {
            panic!(
                "Could not build certified API docs, no certified target exists for {}",
                run.target.to_string()
            )
        } else {
            run.builder.info(&format!("libcore for `{}` is not certified", run.target.to_string()));
        }
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let certified_crates = vec!["core".into()];
        // Build the docs, noting which items are certified and which aren't.
        builder.ensure(
            doc::Std::from_build_compiler(
                builder.compiler(builder.top_stage, builder.host_target),
                self.target,
                doc::DocumentationFormat::Html,
                true,
            )
            .with_crates(certified_crates),
        )
    }
}
