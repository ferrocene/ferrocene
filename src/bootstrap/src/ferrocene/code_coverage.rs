use crate::builder::{Builder, RunConfig, ShouldRun, Step};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ProfilerBuiltinsNoCore;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct ProfilerBuiltins;

impl Step for ProfilerBuiltinsNoCore {
    type Output = ();
    const ONLY_HOSTS: bool = true;
    const DEFAULT: bool = true;

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info("Building profiler_builtins with no_core");
        dbg!("Building profiler_builtins here");
        
        // let compiler = builder.compiler(0, self.target);
        // let cargo = tool::prepare_tool_cargo(
        //     builder,
        //     compiler,
        //     Mode::ToolBootstrap,
        //     self.target,
        //     "test",
        //     "ferrocene/tools/generate-tarball",
        //     SourceType::InTree,
        //     &[],
        // );
        // crate::core::build_steps::test::run_cargo_test(
        //     cargo,
        //     &[],
        //     &[],
        //     "generate-tarball",
        //     "generate-tarball",
        //     compiler,
        //     self.target,
        //     builder,
        // );
    }

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Self {});
    }
}
