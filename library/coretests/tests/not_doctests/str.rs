use core::str::pattern::Pattern;
use core::{ptr, slice};

#[test]
fn str_from_utf8_ok() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = str::from_utf8(&sparkle_heart);
    assert_eq!(Ok("💖"), sparkle_heart);
}

#[test]
fn str_from_utf8_err() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8(&sparkle_heart_err);
    assert!(sparkle_heart_err.is_err());
}

#[test]
fn str_from_utf8_mut_ok() {
    let mut sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = str::from_utf8_mut(&mut sparkle_heart);
    assert_eq!(Ok("💖"), sparkle_heart.as_deref());
}

#[test]
fn str_from_utf8_mut_err() {
    let mut sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8_mut(&mut sparkle_heart_err);
    assert!(sparkle_heart_err.is_err());
}

// NOTE: Panicking version of this test is missing, since it is impossible to trigger a panic without undefined behaviour.
#[test]
fn str_from_utf8_unchecked() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe { str::from_utf8_unchecked(&sparkle_heart) };
    assert_eq!("💖", sparkle_heart);
}

// NOTE: Panicking version of this test is missing, since it is impossible to trigger a panic without undefined behaviour.
#[test]
fn str_from_utf8_unchecked_mut() {
    let mut sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe { str::from_utf8_unchecked_mut(&mut sparkle_heart) };
    assert_eq!("💖", sparkle_heart);
}

#[test]
fn str_utf8_error() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8(&sparkle_heart_err).unwrap_err();

    assert_eq!(sparkle_heart_err.valid_up_to(), 1);
    assert_eq!(sparkle_heart_err.error_len(), Some(1));
}

const STR_MUT_BYTES: [u8; 13] =
    [b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!'];

macro_rules! test_str_slice_index {
    ($($fn:ident: $range_1:expr => $res:literal, $range_2:expr => None;)*) => { $(
        #[test]
        fn $fn() {
            use slice::SliceIndex;

            let str_ref = "Hello, World!";
            let str_ptr = str_ref as *const str;

            let mut str_mut_bytes = STR_MUT_BYTES;
            let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
            let str_mut_ptr = str_mut as *mut str;

            assert_eq!(Some($res), SliceIndex::get($range_1, str_ref));
            assert_eq!(Some($res), SliceIndex::get_mut($range_1, str_mut).as_deref());

            assert_eq!(None, SliceIndex::get($range_2, str_ref));
            assert_eq!(None, SliceIndex::get_mut($range_2, str_mut));

            assert_eq!($res, SliceIndex::index($range_1, str_ref));
            assert_eq!($res, SliceIndex::index_mut($range_1, str_mut));

            assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked($range_1, str_ptr) }));
            assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked_mut($range_1, str_mut_ptr) }));
        }
    )*};
}
test_str_slice_index!(
    str_slice_index_range: 0..4 => "Hell", 100..100 => None;
    str_slice_index_range_from: 0.. => "Hello, World!", 100.. => None;
    str_slice_index_range_to: ..4 => "Hell", ..100 => None;
);

macro_rules! test_str_slice_index_panic {
    ( $($fn:ident => $range:expr,)*) => { $(
        #[test]
        #[should_panic = "byte index 100 is out of bounds of `Hello, World!`"]
        fn $fn() {
            let str_ref = "Hello, World!";
            slice::SliceIndex::index($range, str_ref);
        }
    )*};

}
test_str_slice_index_panic!(
    str_slice_index_panic_range => 100..100,
    str_slice_index_panic_range_to => ..100,
    str_slice_index_panic_range_from => 100..,
);

macro_rules! test_str_slice_index_mut_panic {
    ( $($fn:ident => $range:expr,)*) => { $(
        #[test]
        #[should_panic = "byte index 100 is out of bounds of `Hello, World!`"]
        fn $fn() {
            let mut str_mut_bytes = STR_MUT_BYTES;
            let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
            slice::SliceIndex::index_mut($range, str_mut);
        }
    )*};

}
test_str_slice_index_mut_panic!(
    str_slice_index_mut_panic_range => 100..100,
    str_slice_index_mut_panic_range_to => ..100,
    str_slice_index_mut_panic_range_from => 100..,
);

#[test]
fn pattern_is_prefix_of() {
    let haystack = "Hello, world!";

    // char
    assert!(Pattern::is_prefix_of('H', haystack));
    assert!(!Pattern::is_prefix_of('w', haystack));

    // &str
    assert!(Pattern::is_prefix_of("Hell", haystack));
    assert!(!Pattern::is_prefix_of("world", haystack));

    // [char; N]
    assert!(Pattern::is_prefix_of(['G', 'H', 'I'], haystack));
    assert!(!Pattern::is_prefix_of(['A', 'B', 'C'], haystack));
}

#[test]
fn pattern_is_suffix_of() {
    let haystack = "Hello, world!";

    // char
    assert!(Pattern::is_suffix_of('!', haystack));
    assert!(!Pattern::is_suffix_of('H', haystack));

    // &str
    assert!(Pattern::is_suffix_of("ld!", haystack));
    assert!(!Pattern::is_suffix_of("llo", haystack));

    // [char; N]
    assert!(Pattern::is_suffix_of(['.', '!', ','], haystack));
    assert!(!Pattern::is_suffix_of(['A', 'B', 'C'], haystack));
}

#[test]
fn str_from_raw_parts_mut() {
    let c = unsafe { core::str::from_raw_parts_mut(STR_MUT_BYTES.cast(), 4) };

    assert_eq!(c, "Hell");
}
