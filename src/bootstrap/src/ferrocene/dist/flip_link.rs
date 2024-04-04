use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct FlipLink {
    pub(crate) target: TargetSelection,
}

impl Step for FlipLink {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("flip-link")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(FlipLink { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let flip_link =
            builder.ensure(crate::ferrocene::tool::flip_link::FlipLink { target: self.target });

        let mut tarball = Tarball::new(builder, "flip-link", &self.target.triple);
        tarball.add_file(flip_link, "bin", 0o755);
        tarball.ferrocene_proxied_binary("bin/flip-link");
        tarball.generate()
    }
}
