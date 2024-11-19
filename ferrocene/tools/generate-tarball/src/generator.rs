// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::compression::{CompressionFormats, CompressionProfile};
use crate::signatures::{SignatureContext, sign_manifest_with_aws_kms};
use crate::tarballer::Tarballer;
use crate::util::{copy_recursive, create_dir_all, path_to_str, remove_dir_all};

#[derive(Debug, clap::Args)]
pub struct Generator {
    /// The name of the product, for display
    #[clap(value_name = "NAME")]
    #[clap(long, default_value = "Product")]
    product_name: String,
    /// The name of the component, distinct from other installed components
    #[clap(value_name = "NAME")]
    #[clap(long, default_value = "component")]
    component_name: String,
    /// The name of the package, tarball
    #[clap(value_name = "NAME")]
    #[clap(long, default_value = "package")]
    package_name: String,
    /// The directory under lib/ where the manifest lives
    #[clap(value_name = "DIR")]
    #[clap(long, default_value = "packagelib")]
    rel_manifest_dir: String,
    /// The string to print after successful installation
    #[clap(value_name = "MESSAGE")]
    #[clap(long, default_value = "Installed.")]
    success_message: String,
    /// Places to look for legacy manifests to uninstall
    #[clap(value_name = "DIRS")]
    #[clap(long, default_value = "")]
    legacy_manifest_dirs: String,
    /// Directory containing files that should not be installed
    #[clap(value_name = "DIR")]
    #[clap(long, default_value = "")]
    non_installed_overlay: String,
    /// Path prefixes of directories that should be installed/uninstalled in bulk
    #[clap(value_name = "DIRS")]
    #[clap(long, default_value = "")]
    bulk_dirs: String,
    /// The directory containing the installation medium
    #[clap(value_name = "DIR")]
    #[clap(long, default_value = "./install_image")]
    image_dir: String,
    /// The directory to do temporary work
    #[clap(value_name = "DIR")]
    #[clap(long, default_value = "./workdir")]
    work_dir: String,
    /// The location to put the final image and tarball
    #[clap(value_name = "DIR")]
    #[clap(long, default_value = "./dist")]
    output_dir: String,
    /// The profile used to compress the tarball.
    #[clap(value_name = "FORMAT", default_value_t)]
    #[clap(long)]
    compression_profile: CompressionProfile,
    /// The formats used to compress the tarball
    #[clap(value_name = "FORMAT", default_value_t)]
    #[clap(long)]
    compression_formats: CompressionFormats,
    /// Modification time that will be set for all files added to the archive.
    /// The default is the date of the first Rust commit from 2006.
    /// This serves for better reproducibility of the archives.
    #[arg(long, value_name = "FILE_MTIME", default_value_t = 1153704088)]
    override_file_mtime: u64,
    /// The commit SHA of the current build
    #[clap(long, value_name = "SHA")]
    ferrocene_commit_sha: Option<String>,
    /// The ARN of the AWS KMS key used to sign the criticalup manifest
    #[clap(long, value_name = "ARN")]
    ferrocene_signing_kms_key_arn: Option<String>,
    /// Path prefix that should only contain Ferrocene files.
    #[clap(long, value_name = "PATH")]
    ferrocene_managed_prefix: Vec<String>,
    /// Path of a binary that should be proxied by criticalup.
    #[clap(long, value_name = "PATH")]
    ferrocene_proxied_binary: Vec<PathBuf>,
    /// Name of the Ferrocene component.
    #[clap(long, value_name = "NAME")]
    ferrocene_component: String,
}

impl Generator {
    /// Generates the actual installer tarball
    pub fn run(self) -> Result<()> {
        let Self {
            // These arguments are emitted by the build step but they serve no purpose
            // for our tarball creation, so ignore them here as rejecting them will fail
            // the build step otherwise
            product_name: _,
            rel_manifest_dir: _,
            success_message: _,
            legacy_manifest_dirs: _,
            bulk_dirs: _,
            component_name,
            //
            package_name,
            image_dir,
            work_dir,
            output_dir,
            compression_profile,
            compression_formats,
            non_installed_overlay,
            override_file_mtime,
            ferrocene_commit_sha,
            ferrocene_signing_kms_key_arn,
            ferrocene_managed_prefix,
            ferrocene_proxied_binary,
            ferrocene_component,
        } = self;
        // prepare working directory
        create_dir_all(&work_dir)?;
        let package_dir = Path::new(&work_dir).join(&package_name);
        if package_dir.exists() {
            remove_dir_all(&package_dir)?;
        }
        create_dir_all(&package_dir)?;

        // copy over the image to the working directory
        copy_recursive(image_dir.as_ref(), &package_dir)?;

        // Copy the overlay.
        // We only preserve the `non_installed_overlay` on the `rust-dev` component.
        // The `builder-config` file present is required for the `download-ci-llvm` flag.
        if component_name == "rust-dev" {
            if !non_installed_overlay.is_empty() {
                copy_recursive(non_installed_overlay.as_ref(), &package_dir)?;
            }
        }

        if let Some(key_arn) = ferrocene_signing_kms_key_arn {
            let Some(commit_sha) = ferrocene_commit_sha else {
                anyhow::bail!("commit sha not provided, but signing was requested");
            };
            sign_manifest_with_aws_kms(
                &SignatureContext {
                    component: &ferrocene_component,
                    commit_sha: &commit_sha,
                    package_dir: &package_dir,
                    proxied_binaries: ferrocene_proxied_binary
                        .iter()
                        .map(PathBuf::as_path)
                        .collect(),
                    managed_prefixes: &ferrocene_managed_prefix,
                },
                &key_arn,
            )?;
        }

        // Make the tarballs
        create_dir_all(&output_dir)?;
        let output = Path::new(&output_dir).join(&package_name);
        let tarballer = Tarballer {
            work_dir,
            input: package_name,
            output: path_to_str(&output)?.into(),
            compression_profile,
            compression_formats,
            override_file_mtime,
        };
        tarballer.run()?;

        Ok(())
    }
}
