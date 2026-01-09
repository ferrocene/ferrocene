// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;
use std::path::{Path, PathBuf};

use crate::FileType;
use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::uv_command;
use crate::utils::exec::{BootstrapCommand, ExecutionContext};
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
        const SBOM_SUBPACKAGE_DIR: &str = "subpackages";
        const MAIN_FERROCENE_SBOM_FILE: &str = "ferrocene_cdx_sbom.json";

        let base_sbom_dir = builder.out.join(SBOM_OUTPUT_DIR);
        let partial_sbom_dir = base_sbom_dir.join(SBOM_SUBPACKAGE_DIR);
        let main_sbom_file = base_sbom_dir.join(MAIN_FERROCENE_SBOM_FILE);

        std::fs::create_dir_all(&partial_sbom_dir)
            .expect("Creating the SBOM subpackage dir should never fail.");

        // ##### Subpackage SBOM filenames
        // **Note:** Must end with `_sbom.json` to auto-detect them for merging.
        const SBOM_FILE_END: &str = "_sbom.json";

        const YARN_ROOT_SBOM: &str = "yarn_root_cdx_sbom.json";
        const UV_FERROCENE_DOC_SBOM: &str = "uv_ferrocene_doc_cdx_sbom.json";
        const UV_AUTOMATIONS_COMMON_SBOM: &str = "uv_ferrocene_automations-common_cdx_sbom.json";

        let cargo_sbom_cfg = [
            CargoSbomCfg {
                curr_dir: &self.root_dir,
                out_file: &partial_sbom_dir.join("cargo_root_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("ferrocene").join("library").join("backtrace-rs"),
                out_file: &partial_sbom_dir.join("cargo_ferrocene_backtrace-rs_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("ferrocene").join("library").join("libc"),
                out_file: &partial_sbom_dir.join("cargo_ferrocene_libc_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("ferrocene").join("tools"),
                out_file: &partial_sbom_dir.join("cargo_ferrocene_tools_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("library"),
                out_file: &partial_sbom_dir.join("cargo_library_cdx_sbom.json"),
                cargo_state: CargoState::Nightly,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("library").join("stdarch"),
                out_file: &partial_sbom_dir.join("cargo_library_stdarch_cdx_sbom.json"),
                cargo_state: CargoState::Nightly,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("src").join("bootstrap"),
                out_file: &partial_sbom_dir.join("cargo_bootstrap_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("src").join("librustdoc"),
                out_file: &partial_sbom_dir.join("cargo_librustdoc_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
            CargoSbomCfg {
                curr_dir: &self.root_dir.join("src").join("tools").join("rustc-perf"),
                out_file: &partial_sbom_dir.join("cargo_tools_rustc-perf_cdx_sbom.json"),
                cargo_state: CargoState::Stable,
            },
        ];

        // ###########

        for cfg in cargo_sbom_cfg {
            let curr_path = cfg.curr_dir.display();
            cargo_sbom(builder.exec_ctx(), cfg).unwrap_or_else(|err| {
                panic!("Building cargo SBOM for '{}' failed. Cause: {err}", curr_path)
            });
        }

        if !BootstrapCommand::new("yarnpkg")
            .current_dir(&self.root_dir)
            .arg("exec")
            .arg("cyclonedx-yarn")
            .arg("-o")
            .arg(partial_sbom_dir.join(YARN_ROOT_SBOM))
            .run(builder.exec_ctx())
        {
            panic!("Building yarn SBOM for the root package failed.");
        }

        uv_sbom(
            builder,
            &self.root_dir.join("ferrocene").join("doc"),
            &partial_sbom_dir.join(UV_FERROCENE_DOC_SBOM),
        )
        .expect("Building uv SBOM for /ferrocene/doc failed.");

        uv_sbom(
            builder,
            &self.root_dir.join("ferrocene").join("tools").join("automations-common"),
            &partial_sbom_dir.join(UV_AUTOMATIONS_COMMON_SBOM),
        )
        .expect("Building uv SBOM for /ferrocene/tools/automations-common failed.");

        let mut merge_cmd = BootstrapCommand::new("cyclonedx");
        merge_cmd
            .current_dir(&partial_sbom_dir)
            .arg("merge")
            .arg("--name='Ferrocene'")
            .arg(format!("--version={}", builder.ferrocene_version))
            .arg("--hierarchical");

        // Gather all created SBOMs from subpackages
        for dir_entry in partial_sbom_dir
            .read_dir()
            .expect("Partial SBOM directory created through commands above")
            .filter_map(|entry| match entry {
                Ok(good_entry) => {
                    if good_entry.file_name().to_string_lossy().ends_with(SBOM_FILE_END) {
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

        if !merge_cmd
            .arg("--output-version=v1_6")
            .arg(format!("--output-file={}", main_sbom_file.display()))
            .run(builder.exec_ctx())
        {
            panic!("Merging SBOMs should never fail.");
        }

        let tarball = Tarball::new(builder, "ferrocene-sbom", &self.target.triple);
        tarball.add_dir(partial_sbom_dir, SBOM_SUBPACKAGE_DIR);
        tarball.add_file(main_sbom_file, "main", FileType::Regular);
        tarball.generate()
    }
}

struct CargoSbomCfg<'a> {
    curr_dir: &'a Path,
    out_file: &'a Path,
    cargo_state: CargoState,
}

fn cargo_sbom(
    exec_ctx: impl AsRef<ExecutionContext>,
    cfg: CargoSbomCfg<'_>,
) -> Result<(), std::io::Error> {
    let mut cargo_sbom_cmd = BootstrapCommand::new("cargo-sbom");

    if cfg.cargo_state == CargoState::Nightly {
        cargo_sbom_cmd.env("RUSTC_BOOTSTRAP", "1");
    }

    let cargo_sbom_output = cargo_sbom_cmd
        .current_dir(cfg.curr_dir)
        .arg("--output-format=cyclone_dx_json_1_6")
        .run_capture_stdout(exec_ctx)
        .stdout();

    fs::write(cfg.out_file, cargo_sbom_output)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CargoState {
    Stable,
    Nightly,
}

/// Uses uv's built in SBOM export.
///
/// **Note:** Requires uv version >= 0.9.x
fn uv_sbom(builder: &Builder<'_>, curr_dir: &Path, out_file: &Path) -> Result<(), ()> {
    if uv_command(builder)
        .current_dir(curr_dir)
        .arg("export")
        .arg("--format=cyclonedx1.5")
        .arg("--preview-features=sbom-export")
        .arg("--all-packages")
        .arg("-o")
        .arg(out_file)
        .run(builder.exec_ctx())
    {
        Ok(())
    } else {
        Err(())
    }
}
