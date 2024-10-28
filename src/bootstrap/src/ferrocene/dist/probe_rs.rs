use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ProbeRs {
    pub(crate) target: TargetSelection,
}

impl Step for ProbeRs {
    type Output = GeneratedTarball;
    const DEFAULT: bool = true;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("probe-rs")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(ProbeRs { target: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let probe_rs =
            builder.ensure(crate::ferrocene::tool::probe_rs::ProbeRs { target: self.target });

        let mut tarball = Tarball::new(builder, "probe-rs", &self.target.triple);
        tarball.add_file(probe_rs, "bin", 0o755);
        tarball.ferrocene_proxied_binary("bin/probe-rs");
        tarball.generate()
    }
}
