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

impl Step for Sbom {
    type Output = GeneratedTarball;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.alias("ferrocene-sbom")
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(Sbom { target: run.target, root_dir: run.builder.src.clone() });
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        false
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        const SBOM_OUTPUT_DIR: &str = "ferrocene-sboms";
        const MAIN_FERROCENE_SBOM_FILE: &str = "ferrocene_cdx_sbom.json";
        const SBOM_SUBPACKAGE_DIR: &str = "subpackages";

        let base_sbom_dir = builder.out.join(SBOM_OUTPUT_DIR);
        let partial_sbom_dir = base_sbom_dir.join(SBOM_SUBPACKAGE_DIR);
        let main_sbom_file = base_sbom_dir.join(MAIN_FERROCENE_SBOM_FILE);

        let yarn_sbom_created = BootstrapCommand::new("yarn")
            .current_dir(&self.root_dir)
            .arg("dlx")
            .arg("-q")
            .arg("@cyclonedx/yarn-plugin-cyclonedx")
            .arg("-o")
            .arg(partial_sbom_dir.join("yarn_root_cdx_sbom.json"))
            .run(builder.exec_ctx());
        if !yarn_sbom_created {
            println!("Failed to create yarn SBOM");
        }

        let mut merge_cmd = BootstrapCommand::new("cyclonedx");

        merge_cmd
            .current_dir(&partial_sbom_dir)
            .arg("merge")
            .arg("--name='Ferrocene'")
            .arg(format!("--version={}", builder.ferrocene_version))
            .arg("--hierarchical");

        for dir_entry in partial_sbom_dir
            .read_dir()
            .expect("Partial SBOM directory created through commands above")
            .filter_map(|entry| match entry {
                Ok(good_entry) => {
                    if good_entry.file_name().to_string_lossy().ends_with("_sbom.json") {
                        Some(good_entry)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
        {
            merge_cmd.arg(format!("--input-files={}", dir_entry.path().display()));
        }

        merge_cmd
            .arg("--output-version=v1_6")
            .arg(format!("--output-file={}", main_sbom_file.display()))
            .run(builder.exec_ctx());

        let tarball = Tarball::new(builder, "ferrocene-sbom", &self.target.triple);
        tarball.add_dir(partial_sbom_dir, SBOM_SUBPACKAGE_DIR);
        tarball.add_file(main_sbom_file, "main", FileType::Regular);
        tarball.generate()
    }
}
