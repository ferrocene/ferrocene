use crate::keys::newtypes::{PayloadBytes, SignatureBytes};
use crate::keys::PublicKey;
use crate::Error;

/// Pair of public and private keys capable of signing.
///
/// A key pair is required whenever signing operations are performed, and the trait allows swapping
/// implementations depending on the use case. The private portion of the pair doesn't have to be
/// stored on the local system. For example, it's possible and recommended for implementations of
/// [`KeyPair`] to rely on a Hardware Security Module.
pub trait KeyPair {
    /// Retrieve the [`PublicKey`] associated with this key pair.
    fn public(&self) -> &PublicKey;

    /// Sign the provided data with this key pair, returning the bytes of the signature.
    fn sign(&self, data: &PayloadBytes<'_>) -> Result<SignatureBytes<'static>, Error>;
}
