use crate::io as std_io;

// SAFETY: must be called only once during runtime cleanup.
// NOTE: this is not guaranteed to run, for example when the program aborts.
pub unsafe fn cleanup() {}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::UNSUPPORTED_PLATFORM
}

// pub fn is_interrupted(_code: i32) -> bool {
//     false
// }

// pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
//     crate::io::ErrorKind::Uncategorized
// }

// pub fn abort_internal() -> ! {
//     core::intrinsics::abort();
// }

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}
