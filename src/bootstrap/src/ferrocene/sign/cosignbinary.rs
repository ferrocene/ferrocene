// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::PathBuf;

use crate::builder::{Builder, ShouldRun, Step};

// Latest version and checksums available at: https://github.com/sigstore/cosign/releases
const COSIGN_VERSION: &str = "2.2.3";
const COSIGN_ARTIFACTS: &[CosignArtifact] = &[
    CosignArtifact {
        target: "x86_64-unknown-linux-gnu",
        name: "cosign-linux-amd64",
        sha256: "f669f41176cb1d58bb6a3fdb06e24861540cfdb5a571b4ec5eb2218b0df5d304",
    },
    CosignArtifact {
        target: "x86_64-pc-windows-msvc",
        name: "cosign-windows-amd64.exe",
        sha256: "f7f272d56c580b0ec96f59bfe9f88ec5f42b6e195df009ce3417428e0e0dead1",
    },
    CosignArtifact {
        target: "x86_64-apple-darwin",
        name: "cosign-darwin-amd64",
        sha256: "2429f4b027fc311a6324e9db6fb3a937d559dc61de906a1c2d0d1e0671685e4c",
    },
    CosignArtifact {
        target: "aarch64-apple-darwin",
        name: "cosign-darwin-arm64",
        sha256: "3d95ab46d4c4cc55e6465758c238dc03f830cc8a1fc38bc7a33bc203e0fb2c3b",
    },
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct CosignBinary;

impl Step for CosignBinary {
    type Output = PathBuf;
    const DEFAULT: bool = false;
    const ONLY_HOSTS: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.never()
    }

    fn run(self, builder: &Builder<'_>) -> PathBuf {
        if builder.config.dry_run() {
            return PathBuf::new();
        }

        let mut artifact = None;
        for candidate in COSIGN_ARTIFACTS {
            if candidate.target == &*builder.config.build.triple {
                artifact = Some(candidate);
                break;
            }
        }
        let Some(artifact) = artifact else {
            eprintln!();
            eprintln!("error: unsupported platform for cosign: {}", builder.config.build);
            eprintln!("note:  add support for it in src/bootstrap/ferrocene/sign.rs");
            eprintln!();
            panic!("could not download cosign");
        };

        let dest = builder
            .out
            .join(builder.config.build.triple)
            .join("ferrocene")
            .join(format!("cosign-{COSIGN_VERSION}"));
        if let Some(parent) = dest.parent() {
            builder.create_dir(parent);
        }

        let url = format!(
            "https://github.com/sigstore/cosign/releases/download/v{}/{}",
            COSIGN_VERSION, artifact.name
        );

        if !dest.exists() {
            builder.config.download_file(&url, &dest, "test");
        }
        if !builder.config.verify(&dest, artifact.sha256) && !builder.config.dry_run() {
            panic!("invalid cosign downloaded");
        }

        #[cfg(unix)]
        if !builder.config.dry_run() {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&dest).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&dest, perms).unwrap();
        }

        dest
    }
}

struct CosignArtifact {
    target: &'static str,
    name: &'static str,
    sha256: &'static str,
}
