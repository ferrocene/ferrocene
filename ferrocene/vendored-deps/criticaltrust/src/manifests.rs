//! Serializable and deserializable representation of criticaltrust manifests.

use crate::keys::{KeyRole, PublicKey};
use crate::signatures::{Signable, SignedPayload};
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Typed representation of a manifest version number.
///
/// The version number is stored as a const generic rather than as a field of the struct. This is
/// done to:
///
/// * Verify that the version number is correct as part of the deserialization process.
/// * Simplify constructing manifests: you don't have to specify the version number, type
///   inference will figure out the right one.
#[derive(PartialEq, Eq)]
pub struct ManifestVersion<const V: u32>;

impl<const V: u32> std::fmt::Debug for ManifestVersion<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ManifestVersion").field(&V).finish()
    }
}

impl<const V: u32> Serialize for ManifestVersion<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(V)
    }
}

impl<'de, const V: u32> Deserialize<'de> for ManifestVersion<V> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = u32::deserialize(deserializer)?;
        if raw != V {
            Err(D::Error::custom(format!(
                "expected version {V}, found version {raw}"
            )))
        } else {
            Ok(ManifestVersion)
        }
    }
}

// Redirects

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectManifest {
    pub version: ManifestVersion<1>,
    #[serde(flatten)]
    pub payload: SignedPayload<Redirect>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirect {
    pub nonce: String,
    pub to: String,
}

impl Signable for Redirect {
    const SIGNED_BY_ROLE: KeyRole = KeyRole::Redirects;
}

// Releases

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseManifest {
    pub version: ManifestVersion<1>,
    #[serde(flatten)]
    pub signed: SignedPayload<Release>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Release {
    pub product: String,
    pub release: String,
    pub commit: String,
    pub packages: Vec<ReleasePackage>,
}

impl Signable for Release {
    const SIGNED_BY_ROLE: KeyRole = KeyRole::Releases;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleasePackage {
    pub package: String,
    pub artifacts: Vec<ReleaseArtifact>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseArtifact {
    pub format: ReleaseArtifactFormat,
    pub size: usize,
    #[serde(with = "crate::serde_base64")]
    pub sha256: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ReleaseArtifactFormat {
    #[serde(rename = "tar.zst")]
    TarZst,
    #[serde(rename = "tar.xz")]
    TarXz,
    #[serde(other)]
    #[doc(hidden)]
    Unknown,
}

// Packages

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    pub version: ManifestVersion<1>,
    #[serde(flatten)]
    pub signed: SignedPayload<Package>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    pub product: String,
    pub package: String,
    pub commit: String,
    pub files: Vec<PackageFile>,
    pub managed_prefixes: Vec<String>,
}

impl Signable for Package {
    const SIGNED_BY_ROLE: KeyRole = KeyRole::Packages;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PackageFile {
    pub path: String,
    pub posix_mode: u32,
    #[serde(with = "crate::serde_base64")]
    pub sha256: Vec<u8>,
    pub needs_proxy: bool,
}

// Keys

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysManifest {
    pub version: ManifestVersion<1>,
    pub keys: Vec<SignedPayload<PublicKey>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_version_debug() {
        assert_eq!("ManifestVersion(1)", format!("{:?}", ManifestVersion::<1>));
        assert_eq!(
            "ManifestVersion(42)",
            format!("{:?}", ManifestVersion::<42>)
        );
    }

    #[test]
    fn test_manifest_version_serialize() {
        assert_eq!("1", serde_json::to_string(&ManifestVersion::<1>).unwrap());
        assert_eq!("42", serde_json::to_string(&ManifestVersion::<42>).unwrap());
    }

    #[test]
    fn test_manifest_version_deserialize() {
        assert_eq!(
            ManifestVersion,
            serde_json::from_str::<ManifestVersion<1>>("1").unwrap()
        );
        assert_eq!(
            ManifestVersion,
            serde_json::from_str::<ManifestVersion<42>>("42").unwrap()
        );

        assert!(serde_json::from_str::<ManifestVersion<1>>("42").is_err());
        assert!(serde_json::from_str::<ManifestVersion<42>>("1").is_err());
    }
}
