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

// covers `<core::marker::PhantomData as core::hash::Hash>::hash`.
#[test]
fn hash_phantom_data() {
    let mut hasher = TestHasher { hash: 0 };
    Hash::hash(&core::marker::PhantomData::<()>::default(), &mut hasher);
    assert_eq!(0, hasher.finish());
}

// covers `<core::num::NonZero as core::hash::Hash>::hash`.
#[test]
fn hash_non_zero() {
    let mut hasher = TestHasher { hash: 0 };
    Hash::hash(&core::num::NonZero::<u8>::new(0x45).unwrap(), &mut hasher);
    assert_eq!(0x45, hasher.finish());
}

// covers `<core::ptr::alignment::Alignment as core::hash::Hash>::hash`.
#[test]
fn hash_alignment() {
    let mut hasher = TestHasher { hash: 0 };
    Hash::hash(&core::ptr::Alignment::MIN, &mut hasher);
    assert_eq!(1, hasher.finish());
}
