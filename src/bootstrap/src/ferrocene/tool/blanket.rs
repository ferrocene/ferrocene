
use std::path::PathBuf;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::tool::SourceType;
use crate::Mode;
use crate::ferrocene::tool::{ToolArtifactKind, ToolBuild};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Blanket {}
pub(in crate::ferrocene) const PATH: &str = "ferrocene/tools/blanket";

impl Step for Blanket {
    type Output = PathBuf;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(PATH)
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Blanket {});
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let mode = Mode::ToolBootstrap;
        let tool_build = ToolBuild {
            build_compiler: builder.compiler(0, builder.host_target),
            target: builder.host_target,
            tool: "blanket",
            mode,
            path: PATH,
            source_type: SourceType::InTree,
            extra_features: vec![],
            allow_features: "",
            cargo_args: Vec::new(),
            artifact_kind: ToolArtifactKind::Binary,
        };
        builder.ensure(tool_build).tool_path
    }
}
