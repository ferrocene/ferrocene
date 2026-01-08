use core::hash::Hasher;

#[test]
fn hasher_default_write_str() {
    struct MyHasher {
        hash: u64,
    }

    impl Hasher for MyHasher {
        fn write(&mut self, buf: &[u8]) {
            for byte in buf {
                self.hash += *byte as u64;
            }
        }
        fn finish(&self) -> u64 {
            self.hash
        }
    }

    let mut hasher = MyHasher { hash: 0 };
    hasher.write_str("Hello, world!");

    assert_eq!(1416, hasher.finish());
}
