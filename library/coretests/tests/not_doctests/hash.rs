use core::hash::{Hash, Hasher};

struct TestHasher {
    hash: u64,
}

impl Hasher for TestHasher {
    fn write(&mut self, buf: &[u8]) {
        for byte in buf {
            self.hash += *byte as u64;
        }
    }
    fn finish(&self) -> u64 {
        self.hash
    }
}

// Cover: core::hash::Hasher::write_str
#[test]
fn hasher_default_write_str() {
    let mut hasher = TestHasher { hash: 0 };
    hasher.write_str("Hello, world!");
    assert_eq!(1416, hasher.finish());
}

// Cover: core::hash::impls::<impl core::hash::Hash for &T>::hash
#[test]
fn hash_mut_ref() {
    fn hash<T: Hash>(a: &mut T) -> u64 {
        let mut hasher = TestHasher { hash: 0 };
        Hash::hash(&a, &mut hasher);
        hasher.finish()
    }

    assert_eq!(20, hash(&mut [1, 2, 3, 4, 5]));
    assert_eq!(1416, hash(&mut "Hello, world!"));
}
