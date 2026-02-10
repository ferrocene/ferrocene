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

        let base_sbom_dir = builder.out.join("ferrocene").join("sbom");
        let main_spdx_sbom_filepath = base_sbom_dir.join(MAIN_FERROCENE_SPDX_SBOM_FILE);

        std::fs::create_dir_all(&base_sbom_dir)
            .expect("Creating the SBOM output dir should never fail.");

        let mut sbom_cmd = BootstrapCommand::new("syft");
        sbom_cmd
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
            .arg("--exclude=./build")
            .arg("--exclude=./build-rust-analyzer")
            // llvm specific excludes
            .arg("--exclude=./src/llvm-project/**/*/benchmarks")
            .arg("--exclude=./src/llvm-project/**/*/docs")
            .arg("--exclude=./src/llvm-project/**/*/examples")
            // Input folders in the llvm-project seem to contain .exe files that are used for testing llvm
            .arg("--exclude=./src/llvm-project/**/*/Inputs")
            // utils folders in the llvm-project seem to be helpers for developing llvm
            .arg("--exclude=./src/llvm-project/**/*/utils");

        // Exclude CI folders of subpackages, because those are not used by Ferrocene
        let sub_folders = ["compiler", "ferrocene", "library", "src", "tests"];
        let ci_folders = [".github", ".ci", ".circleci", ".forgejo"];
        let exclude_external_ci = sub_folders.iter().flat_map(|sub_folder| {
            ci_folders
                .iter()
                .map(move |ci_folder| format!("--exclude=./{}/**/*/{}", sub_folder, ci_folder))
        });

        for exclude in exclude_external_ci {
            sbom_cmd.arg(exclude);
        }

        if !sbom_cmd
            // Note: Using `self.root.dir` instead of `./` somehow breaks path exclusions
            .arg("./")
            .run(builder.exec_ctx())
        {
            panic!("Building the Ferrocene SBOM using syft failed.");
        }

        builder.info(&format!("Saving SBOM to {}", main_spdx_sbom_filepath.display()));

        let tarball = Tarball::new(builder, SBOM_CMD_NAME, &self.target.triple);
        tarball.add_file(main_spdx_sbom_filepath, ".", FileType::Regular);
        tarball.generate()
    }
}
