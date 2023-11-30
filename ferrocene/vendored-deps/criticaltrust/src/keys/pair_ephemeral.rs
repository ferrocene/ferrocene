use crate::keys::newtypes::{PayloadBytes, PrivateKeyBytes, SignatureBytes};
use crate::keys::{KeyAlgorithm, KeyPair, KeyRole, PublicKey};
use crate::Error;
use time::OffsetDateTime;

/// Pair of public and private keys generated at runtime and kept in memory.
///
/// There is intentionally no way to persist the private key of ephemeral key pairs, as that's
/// considerably less secure than storing the key in a Hardware Security Module. Ephemeral key
/// pairs are primarily meant to be used during automated testing.
pub struct EphemeralKeyPair {
    public: PublicKey,
    private: PrivateKeyBytes<'static>,
}

impl EphemeralKeyPair {
    /// Generate a new key pair using the given algorithm and key role.
    pub fn generate(
        algorithm: KeyAlgorithm,
        role: KeyRole,
        expiry: Option<OffsetDateTime>,
    ) -> Result<Self, Error> {
        let private = algorithm.methods().generate_private_key()?;

        Ok(EphemeralKeyPair {
            public: PublicKey {
                role,
                algorithm,
                expiry,
                public: algorithm
                    .methods()
                    .derive_public_key_from_private_key(&private)?,
            },
            private,
        })
    }
}

impl KeyPair for EphemeralKeyPair {
    fn public(&self) -> &PublicKey {
        &self.public
    }

    fn sign(&self, data: &PayloadBytes<'_>) -> Result<SignatureBytes<'static>, Error> {
        self.public.algorithm.methods().sign(&self.private, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALGORITHM: KeyAlgorithm = KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer;

    #[test]
    fn test_unique_keys_are_generated() {
        assert_ne!(
            EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None)
                .unwrap()
                .private,
            EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None)
                .unwrap()
                .private
        );
    }

    #[test]
    fn test_key_with_unknown_algorithm_is_not_generated() {
        assert!(matches!(
            EphemeralKeyPair::generate(KeyAlgorithm::Unknown, KeyRole::Root, None),
            Err(Error::UnsupportedKey),
        ));
    }

    #[test]
    fn test_public_key_is_expected() {
        let key = EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None).unwrap();

        let public = key.public();
        assert_eq!(public.role, KeyRole::Root);
        assert_eq!(public.algorithm, ALGORITHM);
        assert_eq!(
            public.public,
            ALGORITHM
                .methods()
                .derive_public_key_from_private_key(&key.private)
                .unwrap()
        );
    }

    #[test]
    fn test_signatures_are_valid() {
        let key = EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None).unwrap();
        let data = PayloadBytes::borrowed(b"Hello world");

        // We can't verify the exact signature is what we expect, as each signature includes random
        // data in it. Instead, we ensure it's correct.
        let signature = key.sign(&data).unwrap();

        assert!(ALGORITHM
            .methods()
            .verify(&key.public.public, &data, &signature)
            .is_ok());
    }

    #[test]
    fn test_sign_with_unknown_algorithm_fails() {
        let mut key = EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None).unwrap();
        key.public.algorithm = KeyAlgorithm::Unknown;

        assert!(matches!(
            key.sign(&PayloadBytes::borrowed(b"Hello world")),
            Err(Error::UnsupportedKey)
        ));
    }
}
