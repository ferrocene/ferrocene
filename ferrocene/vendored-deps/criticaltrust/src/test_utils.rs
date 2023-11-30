use crate::keys::{EphemeralKeyPair, KeyAlgorithm, KeyPair, KeyRole, PublicKey};
use crate::signatures::{Keychain, SignedPayload};
use base64::Engine;
use time::{Duration, OffsetDateTime};

const ALGORITHM: KeyAlgorithm = KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer;

pub(crate) struct TestEnvironment {
    root: EphemeralKeyPair,
    keychain: Keychain,
}

impl TestEnvironment {
    pub(crate) fn prepare() -> Self {
        let root = EphemeralKeyPair::generate(ALGORITHM, KeyRole::Root, None).unwrap();
        let keychain = Keychain::new(root.public()).unwrap();

        Self { root, keychain }
    }

    pub(crate) fn keychain(&self) -> &Keychain {
        &self.keychain
    }

    pub(crate) fn create_untrusted_key(&self, role: KeyRole) -> EphemeralKeyPair {
        EphemeralKeyPair::generate(ALGORITHM, role, None).unwrap()
    }

    pub(crate) fn create_key(&mut self, role: KeyRole) -> EphemeralKeyPair {
        let key = self.create_untrusted_key(role);
        self.sign_and_add_key(key.public());
        key
    }

    pub(crate) fn create_key_with_expiry(
        &mut self,
        role: KeyRole,
        expiry_diff_hours: i64,
    ) -> EphemeralKeyPair {
        let expiry = OffsetDateTime::now_utc() + Duration::hours(expiry_diff_hours);
        let key = EphemeralKeyPair::generate(ALGORITHM, role, Some(expiry)).unwrap();
        self.sign_and_add_key(key.public());
        key
    }

    fn sign_and_add_key(&mut self, key: &PublicKey) {
        let mut payload = SignedPayload::new(key).unwrap();
        payload.add_signature(&self.root).unwrap();

        self.keychain.load(&payload).unwrap();
    }
}

pub(crate) fn base64_encode(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub(crate) fn base64_decode(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::engine::general_purpose::STANDARD.decode(encoded)
}
