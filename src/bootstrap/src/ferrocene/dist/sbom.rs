// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::FileType;
use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::utils::exec::BootstrapCommand;
use crate::utils::tarball::{GeneratedTarball, Tarball};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Sbom {
    pub(crate) target: TargetSelection,
    root_dir: PathBuf,
}

const SBOM_CMD_NAME: &str = "ferrocene-sbom";

impl Step for Sbom {
    type Output = GeneratedTarball;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias(SBOM_CMD_NAME)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Sbom { target: run.target, root_dir: run.builder.src.clone() });
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        false
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        const MAIN_FERROCENE_SPDX_SBOM_FILE: &str = "ferrocene_spdx_sbom.json";

        let base_sbom_dir = builder.out.join(SBOM_CMD_NAME);
        let main_spdx_sbom_filepath = base_sbom_dir.join(MAIN_FERROCENE_SPDX_SBOM_FILE);

        std::fs::create_dir_all(&base_sbom_dir)
            .expect("Creating the SBOM output dir should never fail.");

        if !BootstrapCommand::new("syft")
            .current_dir(&self.root_dir)
            .env("SYFT_JAVASCRIPT_SEARCH_REMOTE_LICENSES", "true")
            .env("SYFT_PYTHON_SEARCH_REMOTE_LICENSES", "true")
            .env("SYFT_FORMAT_SPDX_JSON_PRETTY", "true")
            .arg("--source-name=Ferrocene")
            .arg("--source-version")
            .arg(&builder.ferrocene_version)
            .arg("--enrich=all")
            .arg("--base-path")
            .arg(&self.root_dir)
            .arg("-o")
            .arg(format!("spdx-json={}", main_spdx_sbom_filepath.display()))
            .arg(&self.root_dir)
            .run(builder.exec_ctx())
        {
            panic!("Building the Ferrocene SBOM using syft failed.");
        }

        let tarball = Tarball::new(builder, SBOM_CMD_NAME, &self.target.triple);
        tarball.add_file(main_spdx_sbom_filepath, ".", FileType::Regular);
        tarball.generate()
    }
}
