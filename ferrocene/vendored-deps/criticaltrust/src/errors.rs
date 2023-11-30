use crate::keys::KeyRole;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to sign data")]
    SignatureFailed,
    #[error("failed to verify signed data")]
    VerificationFailed,
    #[error("failed to generate a local key")]
    LocalKeyGenerationFailed,
    #[error("wrong key role for the trust root key (expected Root, found {0:?})")]
    WrongKeyRoleForTrustRoot(KeyRole),
    #[error("failed to serialize the contents of the signed payload")]
    SignedPayloadSerializationFailed(#[source] serde_json::Error),
    #[error("failed to deserialize verified data")]
    DeserializationFailed(#[source] serde_json::Error),
    #[error("failed to load key pair")]
    InvalidKey(String),
    #[error("unsupported key")]
    UnsupportedKey,
    #[cfg(feature = "aws-kms")]
    #[error("failed to retrieve the public key from AWS KMS")]
    AwsKmsFailedToGetPublicKey(
        #[from] aws_sdk_kms::types::SdkError<aws_sdk_kms::error::GetPublicKeyError>,
    ),
    #[cfg(feature = "aws-kms")]
    #[error("failed to sign data with AWS KMS")]
    AwsKmsFailedToSign(#[from] aws_sdk_kms::types::SdkError<aws_sdk_kms::error::SignError>),
}
