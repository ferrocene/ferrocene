//! Low-level keys and digital signature management.
//!
//! This module provides a safe and secure abstraction over the cryptographic primitives used by
//! criticaltrust, with the goal of preventing misuse through the type system. For a higher level
//! abstraction, check out the [`signatures`](crate::signatures) module.

mod algorithms;
pub mod newtypes;
mod pair;
#[cfg(feature = "aws-kms")]
mod pair_aws_kms;
mod pair_ephemeral;
mod public;

pub use algorithms::KeyAlgorithm;
pub use pair::KeyPair;
#[cfg(feature = "aws-kms")]
pub use pair_aws_kms::AwsKmsKeyPair;
pub use pair_ephemeral::EphemeralKeyPair;
pub use public::{KeyId, KeyRole, PublicKey};
