//! Wrapper functions to encode/decode a string as base64 during serialization and deserialization.
//! The module is supposed to be used by adding `#[serde(with = "crate::serde_base64")]` to each
//! field you want to encode/decode as base64.

use base64::engine::GeneralPurpose;
use base64::Engine;
use serde::de::Visitor;
use serde::{Deserializer, Serializer};
use std::marker::PhantomData;

const ENGINE: &GeneralPurpose = &base64::engine::general_purpose::STANDARD;

pub(crate) fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: SerdeBase64,
    S: Serializer,
{
    serializer.serialize_str(&ENGINE.encode(value.to_bytes()))
}

pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: SerdeBase64,
    D: Deserializer<'de>,
{
    struct Base64Visitor<T: SerdeBase64>(PhantomData<T>);

    impl<'de, T: SerdeBase64> Visitor<'de> for Base64Visitor<T> {
        type Value = T;

        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "a base64-encoded string")
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let decoded = ENGINE.decode(v).map_err(E::custom)?;
            T::from_bytes(decoded).map_err(E::custom)
        }
    }

    deserializer.deserialize_str(Base64Visitor(PhantomData))
}

pub(crate) trait SerdeBase64: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, String>;
    fn to_bytes(&self) -> &[u8];
}

impl SerdeBase64 for Vec<u8> {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
        Ok(bytes)
    }

    fn to_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}

impl SerdeBase64 for String {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
        String::from_utf8(bytes).map_err(|e| e.to_string())
    }

    fn to_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Foo<T: SerdeBase64> {
        plain: T,
        #[serde(with = "crate::serde_base64")]
        encoded: T,
    }

    #[test]
    fn test_serialize_deserialize_string() {
        let initial = Foo {
            plain: "Foo bar".into(),
            encoded: "Hello world".into(),
        };

        let encoded = serde_json::to_string(&initial).unwrap();
        assert_eq!(
            "{\"plain\":\"Foo bar\",\"encoded\":\"SGVsbG8gd29ybGQ=\"}",
            encoded
        );

        let decoded: Foo<String> = serde_json::from_str(&encoded).unwrap();
        assert_eq!(initial, decoded);
    }

    #[test]
    fn test_serialize_deserialize_vec_u8() {
        let initial = Foo {
            plain: vec![1, 2, 3, 4],
            encoded: vec![5, 6, 7, 8],
        };

        let encoded = serde_json::to_string(&initial).unwrap();
        assert_eq!("{\"plain\":[1,2,3,4],\"encoded\":\"BQYHCA==\"}", encoded);

        let decoded: Foo<Vec<u8>> = serde_json::from_str(&encoded).unwrap();
        assert_eq!(initial, decoded);
    }
}
