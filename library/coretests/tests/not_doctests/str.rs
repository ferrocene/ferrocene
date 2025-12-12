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

#[test]
fn str_slice_index_range() {
    use slice::SliceIndex;

    let str_ref = "Hello, World!";
    let str_ptr = str_ref as *const str;

    let mut str_mut_bytes = STR_MUT_BYTES;
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
    let str_mut_ptr = str_mut as *mut str;

    assert_eq!(Some("Hell"), SliceIndex::get(0..4, str_ref));
    assert_eq!(Some("Hell"), SliceIndex::get_mut(0..4, str_mut).as_deref());

    assert_eq!(None, SliceIndex::get(100..100, str_ref));
    assert_eq!(None, SliceIndex::get_mut(100..100, str_mut));

    assert_eq!("Hell", SliceIndex::index(0..4, str_ref));
    assert_eq!("Hell", SliceIndex::index_mut(0..4, str_mut));

    assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked(0..4, str_ptr) }));
    assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked(0..4, str_mut_ptr) }));
}

#[test]
fn str_slice_index_range_from() {
    use slice::SliceIndex;

    let str_ref = "Hello, World!";
    let str_ptr = str_ref as *const str;

    let mut str_mut_bytes = STR_MUT_BYTES;
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
    let str_mut_ptr = str_mut as *mut str;

    assert_eq!(Some("Hello, World!"), SliceIndex::get(0.., str_ref));
    assert_eq!(Some("Hello, World!"), SliceIndex::get_mut(0.., str_mut).as_deref());

    assert_eq!(None, SliceIndex::get(100.., str_ref));
    assert_eq!(None, SliceIndex::get_mut(100.., str_mut));

    assert_eq!("Hello, World!", SliceIndex::index(0.., str_ref));
    assert_eq!("Hello, World!", SliceIndex::index_mut(0.., str_mut));

    assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked(0.., str_ptr) }));
    assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked(0.., str_mut_ptr) }));
}

#[test]
fn str_slice_index_range_to() {
    use slice::SliceIndex;

    let str_ref = "Hello, World!";
    let str_ptr = str_ref as *const str;

    let mut str_mut_bytes = STR_MUT_BYTES;
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
    let str_mut_ptr = str_mut as *mut str;

    assert_eq!(Some("Hell"), SliceIndex::get(..4, str_ref));
    assert_eq!(Some("Hell"), SliceIndex::get_mut(..4, str_mut).as_deref());

    assert_eq!(None, SliceIndex::get(..100, str_ref));
    assert_eq!(None, SliceIndex::get_mut(..100, str_mut));

    assert_eq!("Hell", SliceIndex::index(..4, str_ref));
    assert_eq!("Hell", SliceIndex::index_mut(..4, str_mut));

    assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked(..4, str_ptr) }));
    assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked(..4, str_mut_ptr) }));
}

#[test]
#[should_panic]
fn str_slice_index_panic_range() {
    let str_ref = "Hello, World!";

    assert_eq!("Hell", slice::SliceIndex::index(100..100, str_ref));
}

#[test]
#[should_panic]
fn str_slice_index_mut_panic_range() {
    let mut str_mut_bytes = [b'H', b'e', b'l', b'l', b'o'];
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();

    assert_eq!("Hell", slice::SliceIndex::index_mut(100..100, str_mut));
}

#[test]
#[should_panic]
fn str_slice_index_panic_range_to() {
    let str_ref = "Hello, World!";

    assert_eq!("Hell", slice::SliceIndex::index(..100, str_ref));
}

#[test]
#[should_panic]
fn str_slice_index_mut_panic_range_to() {
    let mut str_mut_bytes = [b'H', b'e', b'l', b'l', b'o'];
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();

    assert_eq!("Hell", slice::SliceIndex::index_mut(..100, str_mut));
}

#[test]
#[should_panic]
fn str_slice_index_panic_range_from() {
    let str_ref = "Hello, World!";

    assert_eq!("Hell", slice::SliceIndex::index(100.., str_ref));
}

#[test]
#[should_panic]
fn str_slice_index_mut_panic_range_from() {
    let mut str_mut_bytes = [b'H', b'e', b'l', b'l', b'o'];
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();

    assert_eq!("Hell", slice::SliceIndex::index_mut(100.., str_mut));
}
