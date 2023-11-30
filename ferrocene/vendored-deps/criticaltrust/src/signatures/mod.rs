//! High-level abstraction for key management and digital signature verification.
//!
//! This module provides [`SignedPayload`], a wrapper around digitally signed payloads that
//! enforces signatures are properly verified before the inner contents are accessible.
//! [`Keychain`] is also provided to establish a root of trust.

mod keychain;
mod payload;

pub use keychain::Keychain;
pub use payload::{PublicKeysRepository, Signable, SignedPayload};
