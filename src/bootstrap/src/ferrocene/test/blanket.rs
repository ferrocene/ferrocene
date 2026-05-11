use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::compile::Std;
use crate::ferrocene::test::{SourceType, tool};
use crate::ferrocene::tool::blanket::PATH as BLANKET_PATH;
use crate::{Kind, Mode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Blanket {}

impl Step for Blanket {
    type Output = ();
    const IS_HOST: bool = true;

    fn is_default_step(builder: &Builder<'_>) -> bool {
        builder.targets.iter().any(|target| target.contains("thumbv7em"))
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(BLANKET_PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self {});
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let host = builder.config.host_target;
        let compiler = builder.compiler(builder.top_stage, host);
        builder.ensure(Std::new(compiler, host));

        builder.info(&format!("Testing {:?}", BLANKET_PATH));
        let mut cmd = tool::prepare_tool_cargo(
            builder,
            compiler,
            Mode::ToolTarget,
            host,
            Kind::Test,
            BLANKET_PATH,
            SourceType::Submodule,
            &[],
        )
        .into_cmd();
        // Since flip-link tests the target, unsetting RUSTFLAGS prevents us from passing invalid args into the build.
        cmd.env_remove("RUSTFLAGS");
        cmd.run(builder);
    }
}
