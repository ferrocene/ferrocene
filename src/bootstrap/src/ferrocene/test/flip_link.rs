use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::compile::Std;
use crate::ferrocene::test::{SourceType, tool};
use crate::ferrocene::tool::flip_link::PATH as FLIP_LINK_PATH;
use crate::{Kind, Mode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlipLink {}

impl Step for FlipLink {
    type Output = ();
    const IS_HOST: bool = true;

    fn is_default_step(builder: &Builder<'_>) -> bool {
        builder.targets.iter().any(|target| target.contains("thumbv7em"))
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(FLIP_LINK_PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(FlipLink {});
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let host = builder.config.host_target;
        let compiler = builder.compiler(builder.top_stage, host);
        builder.ensure(Std::new(compiler, host));
        // The flip link tests require a thumbv7em-none-eabi target to exist
        let thumb = crate::TargetSelection::from_user("thumbv7em-none-eabi");
        builder.ensure(Std::new(compiler, thumb));

        builder.info("Testing ferrocene/tools/flip-link");
        let mut cmd = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolTarget,
            host,
            Kind::Test,
            FLIP_LINK_PATH,
            SourceType::Submodule,
            &[],
        )
        .into_cmd();
        // Since flip-link tests the target, unsetting RUSTFLAGS prevents us from passing invalid args into the build.
        cmd.env_remove("RUSTFLAGS");
        cmd.run(builder);
    }
}
