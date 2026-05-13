use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::build_steps::compile::Std;
use crate::core::config::TargetSelection;
use crate::ferrocene::test::{SourceType, tool};
use crate::ferrocene::tool::flip_link::PATH as FLIP_LINK_PATH;
use crate::{Kind, Mode};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FlipLink {}

impl Step for FlipLink {
    type Output = ();
    const IS_HOST: bool = true;

    fn is_default_step(builder: &Builder<'_>) -> bool {
        // The flip link tests require a thumbv7em-none-eabi target to exist
        builder.targets.iter().any(|target| target.triple == "thumbv7em-none-eabi")
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(FLIP_LINK_PATH)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(FlipLink {});
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let thumb = TargetSelection::from_user("thumbv7em-none-eabi");
        if !builder.targets.contains(&thumb) {
            eprintln!("can't run flip-link tests without thumbv7 built!");
            crate::exit!(1);
        }
        if !builder.config.lld_enabled {
            eprintln!("can't run flip-link tests without LLD built!");
            crate::exit!(1);
        }

        let host = builder.config.host_target;
        let compiler = builder.compiler(builder.top_stage, host);
        builder.ensure(Std::new(compiler, host));
        builder.ensure(Std::new(compiler, thumb));

        builder.info("Testing ferrocene/tools/flip-link");
        builder.require_submodule("ferrocene/tools/flip-link", None);
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
