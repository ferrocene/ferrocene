mod ecdsa_p256_sha256_asn1_spki_der;

use crate::keys::algorithms::ecdsa_p256_sha256_asn1_spki_der::EcdsaP256Sha256Asn1SpkiDer;
use crate::keys::newtypes::{PayloadBytes, PrivateKeyBytes, PublicKeyBytes, SignatureBytes};
use crate::Error;
use serde::{Deserialize, Serialize};

/// Cryptographic algorithm used for signature verification.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyAlgorithm {
    /// ECDSA using P256 and SHA256, encoded as ASN.1, with the public key encoded as SPKI with
    /// DER.
    #[serde(rename = "ecdsa-p256-sha256-asn1-spki-der")]
    EcdsaP256Sha256Asn1SpkiDer,
    #[serde(other)]
    #[doc(hidden)]
    Unknown,
}

impl KeyAlgorithm {
    pub(crate) fn methods(&self) -> &'static dyn Algorithm {
        match self {
            KeyAlgorithm::EcdsaP256Sha256Asn1SpkiDer => &EcdsaP256Sha256Asn1SpkiDer,
            KeyAlgorithm::Unknown => &UnknownAlgorithm,
        }
    }
}

pub(crate) trait Algorithm {
    fn sign(
        &self,
        private_key: &PrivateKeyBytes<'_>,
        payload: &PayloadBytes<'_>,
    ) -> Result<SignatureBytes<'static>, Error>;
    fn verify(
        &self,
        public_key: &PublicKeyBytes<'_>,
        payload: &PayloadBytes<'_>,
        signature: &SignatureBytes<'_>,
    ) -> Result<(), Error>;
    fn generate_private_key(&self) -> Result<PrivateKeyBytes<'static>, Error>;
    fn derive_public_key_from_private_key(
        &self,
        private_key: &PrivateKeyBytes<'_>,
    ) -> Result<PublicKeyBytes<'static>, Error>;
}

struct UnknownAlgorithm;

impl Algorithm for UnknownAlgorithm {
    fn sign(
        &self,
        _: &PrivateKeyBytes<'_>,
        _: &PayloadBytes<'_>,
    ) -> Result<SignatureBytes<'static>, Error> {
        Err(Error::UnsupportedKey)
    }

    fn verify(
        &self,
        _: &PublicKeyBytes<'_>,
        _: &PayloadBytes<'_>,
        _: &SignatureBytes<'_>,
    ) -> Result<(), Error> {
        Err(Error::UnsupportedKey)
    }

    fn generate_private_key(&self) -> Result<PrivateKeyBytes<'static>, Error> {
        Err(Error::UnsupportedKey)
    }

    fn derive_public_key_from_private_key(
        &self,
        _: &PrivateKeyBytes<'_>,
    ) -> Result<PublicKeyBytes<'static>, Error> {
        Err(Error::UnsupportedKey)
    }
}
