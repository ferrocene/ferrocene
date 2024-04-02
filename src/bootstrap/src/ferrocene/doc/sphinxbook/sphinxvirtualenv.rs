// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;
use std::process::Command;

use crate::builder::{Builder, ShouldRun, Step};
use crate::core::config::TargetSelection;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(super) struct SphinxVirtualEnv {
    pub(super) target: TargetSelection,
}

impl Step for SphinxVirtualEnv {
    type Output = VirtualEnv;
    const DEFAULT: bool = false;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        let venv = builder.out.join(self.target.triple).join("ferrocene").join("sphinx-venv");
        let shared_resources =
            builder.src.join("ferrocene").join("doc").join("sphinx-shared-resources");
        let requirements = shared_resources.join("requirements.txt");
        let installed_requirements = venv.join("installed-requirements.txt");

        // Avoid rebuilding the venv if it's up to date.
        if installed_requirements.is_file() {
            if builder.read(&requirements) == builder.read(&installed_requirements) {
                return VirtualEnv { path: venv };
            }
        }

        if venv.is_dir() {
            builder.remove_dir(&venv);
        }
        builder.info("Installing dependencies for building Sphinx documentation");
        builder.run(
            Command::new(
                builder
                    .config
                    .python
                    .as_ref()
                    .expect("Python is required to build Sphinx documentation"),
            )
            .args(&["-m", "venv"])
            .arg(&venv),
        );
        let venv = VirtualEnv { path: venv };
        builder
            .run(venv.cmd("pip").args(&["install", "--require-hashes", "-r"]).arg(&requirements));
        builder.copy_link(&requirements, &installed_requirements);

        venv
    }
}

#[derive(Clone)]
pub(super) struct VirtualEnv {
    path: PathBuf,
}

impl VirtualEnv {
    pub(super) fn cmd(&self, bin: &str) -> Command {
        #[cfg(not(target_os = "windows"))]
        const BIN_DIR: &str = "bin";
        #[cfg(target_os = "windows")]
        const BIN_DIR: &str = "scripts";

        Command::new(self.path.join(BIN_DIR).join(bin))
    }
}
