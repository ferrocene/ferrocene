use super::newtypes::SignatureBytes;
use crate::keys::newtypes::{PayloadBytes, PublicKeyBytes};
use crate::keys::KeyAlgorithm;
use crate::sha256::hash_sha256;
use crate::signatures::{PublicKeysRepository, Signable};
use crate::Error;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Public key used for verification of signed payloads.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct PublicKey {
    pub role: KeyRole,
    pub algorithm: KeyAlgorithm,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expiry: Option<OffsetDateTime>,
    #[serde(with = "crate::serde_base64")]
    pub public: PublicKeyBytes<'static>,
}

impl PublicKey {
    /// Verify whether the provided payload matches the provided signature. Signature verification
    /// could fail if:
    ///
    /// * The expected key role is different than the current key role.
    /// * The current key expired.
    /// * The signature doesn't match the payload.
    /// * The signature wasn't performed by the current key.
    pub fn verify(
        &self,
        role: KeyRole,
        payload: &PayloadBytes<'_>,
        signature: &SignatureBytes<'_>,
    ) -> Result<(), Error> {
        if role != self.role || role == KeyRole::Unknown {
            return Err(Error::VerificationFailed);
        }

        if let Some(expiry) = self.expiry {
            if OffsetDateTime::now_utc() > expiry {
                return Err(Error::VerificationFailed);
            }
        }

        self.algorithm
            .methods()
            .verify(&self.public, payload, signature)
    }

    /// Calculate and return the ID of this public key. This is a relatively expensive operation,
    /// so it's better to cache or clone the resulting ID rather than recalculating it on the fly.
    pub fn calculate_id(&self) -> KeyId {
        KeyId(hash_sha256(self.public.as_bytes()))
    }

    /// Checks whether this public key is supported by this version of criticalup.
    pub fn is_supported(&self) -> bool {
        self.role != KeyRole::Unknown && self.algorithm != KeyAlgorithm::Unknown
    }
}

impl PublicKeysRepository for PublicKey {
    fn get<'a>(&'a self, id: &KeyId) -> Option<&'a PublicKey> {
        if *id == self.calculate_id() {
            Some(self)
        } else {
            None
        }
    }
}

impl Signable for PublicKey {
    const SIGNED_BY_ROLE: KeyRole = KeyRole::Root;
}

/// Role of the key, used to determine which kinds of payloads the key is authorized to verify.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum KeyRole {
    /// `releases` key role, used to sign releases.
    Releases,
    /// `packages` key role, used to sign packages.
    Packages,
    /// `redirects` key role, used to sign dynamic server redirects.
    Redirects,
    /// `root` key role, used to sign other keys.
    Root,
    #[serde(other)]
    #[doc(hidden)]
    Unknown,
}

/// Opaque unique identifier for any given key.
///
/// You can obtain it by calling [`PublicKey::calculate_id`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct KeyId(#[serde(with = "crate::serde_base64")] Vec<u8>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::{EphemeralKeyPair, KeyPair};
    use crate::test_utils::base64_decode;
    use time::Duration;

    const SAMPLE_PAYLOAD: PayloadBytes<'static> = PayloadBytes::borrowed(b"Hello world");
    const SAMPLE_KEY_ID: &str = "nvb7o7wel0FvL/hZ/P4yI3JJRfYYjTXZPpdV+xNQqTA=";
    const SAMPLE_KEY: &str = "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEAGDPB8wZg17bAny3c0jPNg8wmnylcKtCLuPnX3GfwEQDf6ydkD1qnOPtMCZBh0P521Q5evvQ1e/rHsjrbBVPMQ==";
    const SAMPLE_SIGNATURE: &str = "MEYCIQC8MN8dk0jkZo1GIY8EZSaLpnDPUqR29E9eerKPjRyeJwIhAOd21m1VqpldE4kagUVZOUL0Pb/EZTQ0ry8ltbC446sh";

    #[test]
    fn test_verify_matches_with_no_expiration() {
        let key = generate(KeyRole::Root, None);
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(key
            .public()
            .verify(KeyRole::Root, &SAMPLE_PAYLOAD, &signature)
            .is_ok())
    }

    #[test]
    fn test_verify_matches_with_valid_expiration() {
        let key = generate(KeyRole::Root, hours_diff(1));
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(key
            .public()
            .verify(KeyRole::Root, &SAMPLE_PAYLOAD, &signature)
            .is_ok());
    }

    #[test]
    fn test_verify_fails_with_different_role() {
        let key = generate(KeyRole::Root, None);
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(matches!(
            key.public()
                .verify(KeyRole::Packages, &SAMPLE_PAYLOAD, &signature),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_the_unknown_role() {
        let key = generate(KeyRole::Unknown, None);
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(matches!(
            key.public()
                .verify(KeyRole::Unknown, &SAMPLE_PAYLOAD, &signature),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_expired_key() {
        let key = generate(KeyRole::Root, hours_diff(-1));
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(matches!(
            key.public()
                .verify(KeyRole::Root, &SAMPLE_PAYLOAD, &signature),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_incorrect_signature() {
        let key = generate(KeyRole::Root, None);
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        let mut bad_signature = signature.as_bytes().to_vec();
        *bad_signature.last_mut().unwrap() = bad_signature.last().unwrap().wrapping_add(1);

        assert!(matches!(
            key.public().verify(
                KeyRole::Root,
                &SAMPLE_PAYLOAD,
                &SignatureBytes::owned(bad_signature)
            ),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_incorrect_payload() {
        let key = generate(KeyRole::Root, None);
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(matches!(
            key.public().verify(
                KeyRole::Root,
                &PayloadBytes::borrowed("Hello world!".as_bytes()),
                &signature
            ),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_empty_signature() {
        let key = generate(KeyRole::Root, None);

        assert!(matches!(
            key.public().verify(
                KeyRole::Root,
                &SAMPLE_PAYLOAD,
                &SignatureBytes::borrowed(&[])
            ),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_wrong_key() {
        let key1 = generate(KeyRole::Root, None);
        let key2 = generate(KeyRole::Root, None);

        let signature = key1.sign(&SAMPLE_PAYLOAD).unwrap();

        assert!(matches!(
            key2.public()
                .verify(KeyRole::Root, &SAMPLE_PAYLOAD, &signature),
            Err(Error::VerificationFailed)
        ));
    }

    #[test]
    fn test_verify_fails_with_unknown_algorithm() {
        let key = generate(KeyRole::Root, None);
        let signature = key.sign(&SAMPLE_PAYLOAD).unwrap();

        let mut public = key.public().clone();
        public.algorithm = KeyAlgorithm::Unknown;

        assert!(matches!(
            public.verify(KeyRole::Root, &SAMPLE_PAYLOAD, &signature),
            Err(Error::UnsupportedKey)
        ));
    }

    #[test]
    fn test_calculate_id() {
        let key = PublicKey {
            role: KeyRole::Root,
            algorithm: KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer,
            expiry: None,
            public: PublicKeyBytes::owned(base64_decode(SAMPLE_KEY).unwrap()),
        };
        assert_eq!(
            key.calculate_id(),
            // base64-encoded sha256 of the key above
            KeyId(base64_decode(SAMPLE_KEY_ID).unwrap())
        );
    }

    #[test]
    fn test_is_key_supported() {
        let key = |role, algorithm| PublicKey {
            role,
            algorithm,
            expiry: None,
            public: PublicKeyBytes::owned(base64_decode(SAMPLE_KEY).unwrap()),
        };

        assert!(!key(KeyRole::Unknown, KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer).is_supported());
        assert!(!key(KeyRole::Root, KeyAlgorithm::Unknown).is_supported());
        assert!(!key(KeyRole::Unknown, KeyAlgorithm::Unknown).is_supported());

        // Test just a few positive combinations
        assert!(key(KeyRole::Root, KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer).is_supported());
        assert!(key(KeyRole::Packages, KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer).is_supported());
    }

    #[test]
    fn test_verify_using_deserialized_key() {
        let key: PublicKey = serde_json::from_str(
            &r#"{
                "role": "root",
                "algorithm": "ecdsa-p256-sha256-asn1-spki-der",
                "expiry": null,
                "public": "$$PUBLICKEY$$"
            }"#
            .replace("$$PUBLICKEY$$", SAMPLE_KEY),
        )
        .unwrap();

        // Ensure the key can verify messages signed with the corresponding private key.
        key.verify(
            KeyRole::Root,
            &SAMPLE_PAYLOAD,
            &SignatureBytes::owned(base64_decode(SAMPLE_SIGNATURE).unwrap()),
        )
        .unwrap();
    }

    #[test]
    fn test_key_deserialization_without_expiry() {
        let key: PublicKey = serde_json::from_str(
            &r#"{
                "role": "root",
                "algorithm": "ecdsa-p256-sha256-asn1-spki-der",
                "expiry": null,
                "public": "$$PUBLICKEY$$"
            }"#
            .replace("$$PUBLICKEY$$", SAMPLE_KEY),
        )
        .unwrap();

        assert_eq!(key.role, KeyRole::Root);
        assert_eq!(key.algorithm, KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer);
        assert_eq!(key.expiry, None);
        assert_eq!(key.public.as_bytes(), base64_decode(SAMPLE_KEY).unwrap());
    }

    #[test]
    fn test_key_deserialization_with_expiry() {
        let key: PublicKey = serde_json::from_str(
            &r#"{
                "role": "packages",
                "algorithm": "ecdsa-p256-sha256-asn1-spki-der",
                "expiry": "2022-03-18T12:04:00+01:00",
                "public": "$$PUBLICKEY$$"
            }"#
            .replace("$$PUBLICKEY$$", SAMPLE_KEY),
        )
        .unwrap();

        assert_eq!(key.role, KeyRole::Packages);
        assert_eq!(key.algorithm, KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer);
        assert_eq!(key.expiry, Some(date("2022-03-18T12:04:00+01:00")));
        assert_eq!(key.public.as_bytes(), base64_decode(SAMPLE_KEY).unwrap());
    }

    #[test]
    fn test_key_deserialization_with_unknown_algorithm() {
        let key: PublicKey = serde_json::from_str(
            &r#"{
                "role": "packages",
                "algorithm": "morse-encoding",
                "expiry": null,
                "public": "$$PUBLICKEY$$"
            }"#
            .replace("$$PUBLICKEY$$", SAMPLE_KEY),
        )
        .unwrap();

        assert_eq!(key.role, KeyRole::Packages);
        assert_eq!(key.algorithm, KeyAlgorithm::Unknown);
        assert_eq!(key.expiry, None);
        assert_eq!(key.public.as_bytes(), base64_decode(SAMPLE_KEY).unwrap());
    }

    #[test]
    fn test_key_serialization_without_expiry() {
        let key = PublicKey {
            role: KeyRole::Root,
            algorithm: KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer,
            expiry: None,
            public: PublicKeyBytes::owned(base64_decode(SAMPLE_KEY).unwrap()),
        };

        assert_eq!(
            r#"{
  "role": "root",
  "algorithm": "ecdsa-p256-sha256-asn1-spki-der",
  "expiry": null,
  "public": "$$PUBLICKEY$$"
}"#
            .replace("$$PUBLICKEY$$", SAMPLE_KEY),
            serde_json::to_string_pretty(&key).unwrap()
        );
    }

    #[test]
    fn test_key_serialization_with_expiry() {
        let key = PublicKey {
            role: KeyRole::Root,
            algorithm: KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer,
            expiry: Some(date("2022-03-18T12:04:00+01:00")),
            public: PublicKeyBytes::owned(base64_decode(SAMPLE_KEY).unwrap()),
        };

        assert_eq!(
            r#"{
  "role": "root",
  "algorithm": "ecdsa-p256-sha256-asn1-spki-der",
  "expiry": "2022-03-18T12:04:00+01:00",
  "public": "$$PUBLICKEY$$"
}"#
            .replace("$$PUBLICKEY$$", SAMPLE_KEY),
            serde_json::to_string_pretty(&key).unwrap()
        );
    }

    fn date(rfc3339: &str) -> OffsetDateTime {
        OffsetDateTime::parse(rfc3339, &time::format_description::well_known::Rfc3339).unwrap()
    }

    fn hours_diff(diff: i64) -> Option<OffsetDateTime> {
        Some(OffsetDateTime::now_utc() + Duration::hours(diff))
    }

    fn generate(role: KeyRole, expiry: Option<OffsetDateTime>) -> EphemeralKeyPair {
        EphemeralKeyPair::generate(KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer, role, expiry).unwrap()
    }
}
