//! Newtypes used by the `keys` module to prevent errors at the type system level.
//!
//! APIs in the `keys` module require multiple byte slices as their input, and it would be easy to
//! accidentally pass a public key when a signature is required (for example). To prevent these
//! kinds of errors at compile time, this module defines newtypes used across criticaltrust.

use crate::serde_base64::SerdeBase64;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

macro_rules! newtypes {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis struct $name:ident(..);
        )*
    ) => {
        $(
            $(#[$meta])*
            #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
            #[serde(transparent)]
            $vis struct $name<'a>(Cow<'a, [u8]>);

            #[allow(unused)]
            impl<'a> $name<'a> {
                /// Create a new instance using borrowed content.
                $vis const fn borrowed(data: &'a [u8]) -> Self{
                    Self(Cow::Borrowed(data))
                }

                /// Create a new instance using owned content.
                $vis fn owned(data: Vec<u8>) -> Self {
                    Self(Cow::Owned(data))
                }

                /// Return the underlying bytes representation.
                $vis fn as_bytes(&self) -> &[u8] {
                    &self.0
                }
            }

            impl SerdeBase64 for $name<'static> {
                fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
                    Ok(Self(Cow::Owned(bytes)))
                }

                fn to_bytes(&self) -> &[u8] {
                    &self.0
                }
            }
        )*
    }
}

newtypes! {
    /// Contains the bytes representing a public key.
    ///
    /// The format of the public key depends on the algorithm used.
    pub struct PublicKeyBytes(..);

    /// Contains the bytes representing a payload to sign or verify.
    pub struct PayloadBytes(..);

    /// Contains the bytes representing a signature.
    ///
    /// The format of the signature depends on the algorithm used.
    pub struct SignatureBytes(..);

    pub(crate) struct PrivateKeyBytes(..);
}
