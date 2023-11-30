use sha2::{Digest, Sha256};

/// Helper function to hash bytes with SHA256. It's a simple wrapper on top of the sha2 crate,
/// turning the three method calls into a function call.
pub(crate) fn hash_sha256(contents: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(contents);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_sha256() {
        const PLAINTEXT: &str = "Hello world";
        const HASHED: &str = "64ec88ca00b268e5ba1a35678a1b5316d212f4f366b2477232534a8aeca37f3c";

        let hashed = hash_sha256(PLAINTEXT.as_bytes());

        let mut hashed_hex = String::new();
        for byte in hashed {
            hashed_hex.push_str(&format!("{byte:0>2x}"));
        }
        assert_eq!(HASHED, hashed_hex);
    }
}
