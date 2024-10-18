// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

use std::collections::HashSet;
use std::fs::File;
#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;
use std::path::Path;

use anyhow::{Error, anyhow};
use criticaltrust::keys::{AwsKmsKeyPair, KeyPair, KeyRole};
use criticaltrust::manifests::{ManifestVersion, Package, PackageFile, PackageManifest};
use criticaltrust::signatures::SignedPayload;
use sha2::{Digest, Sha256};
use tokio::runtime::Runtime;

pub(crate) struct SignatureContext<'a> {
    pub(crate) component: &'a str,
    pub(crate) commit_sha: &'a str,
    pub(crate) package_dir: &'a Path,
    pub(crate) proxied_binaries: HashSet<&'a Path>,
    pub(crate) managed_prefixes: &'a [String],
}

pub(crate) fn sign_manifest_with_aws_kms(
    ctx: &SignatureContext<'_>,
    key_arn: &str,
) -> Result<(), Error> {
    let tokio = Runtime::new()?;
    let aws_config = tokio.block_on(aws_config::load_from_env());
    let kms_client = aws_sdk_kms::Client::new(&aws_config);

    let key = AwsKmsKeyPair::new(key_arn, tokio.handle().clone(), kms_client, KeyRole::Packages)?;
    sign_manifest(ctx, &key)
}

fn sign_manifest(ctx: &SignatureContext<'_>, key_pair: &dyn KeyPair) -> Result<(), Error> {
    let mut package = Package {
        product: "ferrocene".into(),
        package: ctx.component.into(),
        commit: ctx.commit_sha.into(),
        files: Vec::new(),
        managed_prefixes: ctx.managed_prefixes.to_vec(),
    };

    collect_files(&mut package, ctx, ctx.package_dir)?;

    // Ensure the contents of the package manifest are sorted, to prevent differences between
    // manifests generated for the same tarball.
    package.files.sort_by_cached_key(|file| file.path.clone());

    let mut signed = SignedPayload::new(&package)?;
    signed.add_signature(key_pair)?;

    let dest_dir = ctx.package_dir.join("share").join("criticaltrust").join("ferrocene");
    std::fs::create_dir_all(&dest_dir)?;
    std::fs::write(
        &dest_dir.join(format!("{}.json", ctx.component)),
        &serde_json::to_vec_pretty(&PackageManifest { version: ManifestVersion::<1>, signed })?,
    )?;

    Ok(())
}

fn collect_files(
    package: &mut Package,
    ctx: &SignatureContext<'_>,
    dir: &Path,
) -> Result<(), Error> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?.path();
        let relative_path = entry.strip_prefix(ctx.package_dir).unwrap();

        #[cfg(not(windows))]
        let needs_proxy = ctx.proxied_binaries.contains(&relative_path);
        #[cfg(windows)] // Ensure we're not checking for the `.exe` instead of the file name
        let needs_proxy =
            ctx.proxied_binaries.contains(&relative_path.with_extension("").as_path());

        let relative_path =
            relative_path.to_str().ok_or_else(|| anyhow!("path {entry:?} is not utf-8"))?;

        if entry.is_file() {
            package.files.push(PackageFile {
                path: relative_path.into(),
                sha256: hash_file(&entry)?,
                #[cfg(not(windows))]
                posix_mode: entry.metadata()?.mode(),
                #[cfg(windows)]
                posix_mode: 0,
                needs_proxy,
            });
        } else if entry.is_dir() {
            collect_files(package, ctx, &entry)?;
        }
    }
    Ok(())
}

fn hash_file(path: &Path) -> Result<Vec<u8>, Error> {
    let mut sha256 = Sha256::new();
    let mut contents = File::open(path)?;
    std::io::copy(&mut contents, &mut sha256)?;
    Ok(sha256.finalize().to_vec())
}

#[cfg(not(windows))]
#[cfg(test)]
mod tests {
    use std::fs::Permissions;
    use std::io::Write;
    use std::os::unix::prelude::PermissionsExt;

    use criticaltrust::keys::{EphemeralKeyPair, KeyAlgorithm};
    use criticaltrust::signatures::Keychain;
    use tempfile::TempDir;

    use super::*;

    // This insta snapshot is complex and brittle to maintain on Windows
    // Windows does not have file modes, and filters don't provide a reliable way to
    // remap Windows paths to Linux and back.
    #[test]
    fn test_sign_manifest() -> Result<(), Error> {
        let package_dir = TempDir::new()?;

        let create_file = |path, contents, mode| {
            let path = package_dir.path().join(path);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut file = File::create(path)?;
            file.write_all(contents)?;
            #[cfg(unix)]
            file.set_permissions(Permissions::from_mode(mode))?;
            Ok::<_, Error>(())
        };

        create_file("lib/librustc_driver.so", b"not a real library", 0o644)?;
        create_file(
            "lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd.rlib",
            b"not a real standard library",
            0o644,
        )?;
        create_file("bin/rustc", b"hello world", 0o755)?;

        // First generate the signature file
        let (key, keychain) = generate_key_and_keychain(KeyRole::Packages)?;
        sign_manifest(
            &SignatureContext {
                component: "demo-package",
                commit_sha: "000000",
                package_dir: package_dir.path(),
                proxied_binaries: [Path::new("bin/rustc")].into_iter().collect(),
                managed_prefixes: &["lib/rustlib/x86_64-unknown-linux-gnu/lib/".into()],
            },
            &key,
        )?;

        // Then deserialize it and verify the signature is correct.
        let deserialized: PackageManifest = serde_json::from_slice(&std::fs::read(
            package_dir.path().join("share/criticaltrust/ferrocene/demo-package.json"),
        )?)?;
        insta::assert_snapshot!(serde_json::to_string_pretty(
            &deserialized.signed.into_verified(&keychain)?
        )?);

        Ok(())
    }

    fn generate_key_and_keychain(role: KeyRole) -> Result<(EphemeralKeyPair, Keychain), Error> {
        const ALGORITHM: KeyAlgorithm = KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer;

        let root_key = EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None)?;
        let mut keychain = Keychain::new(root_key.public())?;

        let key = EphemeralKeyPair::generate(ALGORITHM, role, None)?;
        let mut signed = SignedPayload::new(key.public())?;
        signed.add_signature(&root_key)?;

        keychain.load(&signed)?;

        Ok((key, keychain))
    }
}
