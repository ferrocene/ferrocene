#[test]
fn slice_methods() {
    let mut arr = [0; 10];
    let slice = arr.as_mut_slice();

    assert_eq!(slice.len(), 10);

    assert!(slice.first_chunk_mut::<11>().is_none());
    assert!(slice.first_chunk_mut::<1>().is_some());

    assert!(slice.split_first_mut().is_some());

    let mut empty = [0; 0];
    assert!(empty.as_mut_slice().split_first_mut().is_none());
}

#[test]
fn slice_partial_eq() {
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    struct Byte(u8);

    let a1 = [Byte(0u8); 100];
    let a2 = [Byte(0u8); 99];
    let mut a3 = [Byte(0u8); 100];

    assert_ne!(a1.as_slice(), a2.as_slice());

    assert_eq!(a1.as_slice(), a3.as_slice());

    a3[a3.len() - 1] = Byte(1);
    assert_ne!(a1.as_slice(), a3.as_slice());
}

#[test]
fn as_mut_array() {
    let mut arr = [0u8; 1];
    let slice = arr.as_mut_slice();

    assert!(slice.as_mut_array::<2>().is_none());
    assert!(slice.as_mut_array::<1>().is_some());
}

#[test]
fn chunks_exact_is_empty() {
    assert!([0; 10].chunks_exact(11).is_empty());
    assert!(![0; 10].chunks_exact(2).is_empty());

    assert!([0; 10].chunks_exact_mut(11).is_empty());
    assert!(![0; 10].chunks_exact_mut(2).is_empty());
}

// covers `core::slice::<impl [T]>::first_mut`
#[test]
fn first_mut() {
    let mut empty = [0u8; 0];
    assert!(empty.first_mut().is_none());

    let mut arr = [0u8; 5];
    *arr.first_mut().unwrap() = 1;

    assert_eq!(arr[0], 1);
}

// covers:
// - `core::slice::<impl [T]>::clone_from_slice`
// - `core::slice::<impl [T]>::split_last`
// - `core::slice::<impl [T]>::split_last_mut`
#[test]
fn split_last() {
    let mut empty = [0u8; 0];

    assert!(empty.split_last().is_none());
    assert!(empty.split_last_mut().is_none());

    let a1 = [1u8; 5];
    let mut a2 = [0u8; 5];

    let (h1, t1) = a1.split_last().unwrap();
    let (h2, t2) = a2.split_last_mut().unwrap();

    *h2 = *h1;
    assert_eq!(*h2, 1);

    t2.clone_from_slice(t1);
    assert_eq!(a1, a2);
}

// covers `core::slice::<impl [T]>::split_at`
#[test]
#[should_panic = "mid > len"]
fn split_at_panics() {
    let _ = [1u8; 5].split_at(6);
}

// covers `core::slice::<impl [T]>::split_at_mut`
#[test]
#[should_panic = "mid > len"]
fn split_at_mut_panics() {
    let _ = [1u8; 5].split_at_mut(6);
}

// covers `core::slice::<impl [T]>::align_to_mut`
#[test]
fn align_to_mut() {
    let mut zst_arr = [(); 5];
    assert_eq!(
        unsafe { zst_arr.align_to_mut::<u8>() },
        ([(); 5].as_mut_slice(), [].as_mut_slice(), [].as_mut_slice())
    );

    #[repr(C, align(256))]
    #[derive(Debug, PartialEq)]
    struct Foo([u8; 256]);

    let mut arr = [0u8; 1];
    assert_eq!(
        unsafe { arr.align_to_mut::<Foo>() },
        ([0u8; 1].as_mut_slice(), [].as_mut_slice(), [].as_mut_slice())
    );
}

// covers `core::slice::rotate::ptr_rotate`
#[test]
fn rotate_zst() {
    let mut zst_arr = [(); 5];

    zst_arr.rotate_left(5);
}

// covers: `<IndexRange as core::slice::index::SliceIndex<[T]>>::{get, get_mut, index, index_mut}`
//
// FIXME: With `#[ferrocene::certified]` this test case will not be necessary, because the tested methods are dead code and are only included due to the cfg-approach.
#[test]
fn index_range_slice_index() {
    core::ferrocene_test::test_index_range_slice_index();
}

// covers: `<IndexRange as core::slice::index::SliceIndex<[T]>>::index`
//
// FIXME: With `#[ferrocene::certified]` this test case will not be necessary, because the tested methods are dead code and are only included due to the cfg-approach.
#[test]
#[should_panic = "range start index 100 out of range for slice of length 5"]
fn index_range_slice_index_panic() {
    core::ferrocene_test::test_index_range_slice_index_panic();
}

// covers: `<IndexRange as core::slice::index::SliceIndex<[T]>>::index_mut`
//
// FIXME: With `#[ferrocene::certified]` this test case will not be necessary, because the tested methods are dead code and are only included due to the cfg-approach.
#[test]
#[should_panic = "range start index 100 out of range for slice of length 5"]
fn index_range_slice_index_panic_mut() {
    core::ferrocene_test::test_index_range_slice_index_panic_mut();
}

// Covers: <[T] as core::slice::specialize::SpecFill<T>>::spec_fill
#[test]
fn test_spec_fill_not_trivially_clone() {
    {
        let mut a = ["0".to_string(), "1".into(), "2".into()];
        a.fill("1".to_string()); // T must not be TriviallyClone
        assert_eq!(a, ["1".to_string(), "1".into(), "1".into()]);
    }

    // Following is necessary to make spec_fill fully covered:
    {
        let mut b: [String; 0] = [];
        b.fill("1".to_string()); // T must not be TriviallyClone
        let c: [String; 0] = [];
        assert_eq!(b, c);
    }
}

// covers: `core::slice::index::into_slice_range`.
#[test]
#[should_panic]
fn test_into_slice_range() {
    let arr = [""; 10];
    let slice = arr.as_slice();

    let _ = slice[(core::ops::Bound::Included(0), core::ops::Bound::Excluded(20))];
}

// covers: `core::slice::ascii::is_ascii_simple`
#[test]
fn slice_ascii_simple() {
    let hello = [b'H', b'e', b'l', b'l', b'o'];
    assert!(core::slice::is_ascii_simple(&hello));

    let sparkle_heart = [240, 159, 146, 150];
    assert!(!core::slice::is_ascii_simple(&sparkle_heart));
}

// covers: `<core::slice::ascii::EscapeAscii<'a> as core::fmt::Display>::fmt`
#[test]
fn slice_escape_ascii_display_fmt_nonempty_front() {
    let slice = b"0\t\r\n'\"\\\x9d";
    let mut escaped = slice.escape_ascii();

    for s in [
        "0\\t\\r\\n\\'\\\"\\\\\\x9d",
        "\\t\\r\\n\\'\\\"\\\\\\x9d",
        "t\\r\\n\\'\\\"\\\\\\x9d",
        "\\r\\n\\'\\\"\\\\\\x9d",
        "r\\n\\'\\\"\\\\\\x9d",
        "\\n\\'\\\"\\\\\\x9d",
        "n\\'\\\"\\\\\\x9d",
        "\\'\\\"\\\\\\x9d",
        "'\\\"\\\\\\x9d",
        "\\\"\\\\\\x9d",
        "\"\\\\\\x9d",
        "\\\\\\x9d",
        "\\\\x9d",
        "\\x9d",
        "x9d",
        "9d",
        "d",
        "",
    ] {
        assert_eq!(format!("{escaped}"), s);
        escaped.next();
    }
}

// covers: `<core::slice::ascii::EscapeAscii<'a> as core::fmt::Display>::fmt`
#[test]
fn slice_escape_ascii_display_fmt_nonempty_back() {
    let slice = b"0\t\r\n'\"\\\x9d";
    let mut escaped = slice.escape_ascii();

    for s in [
        "0\\t\\r\\n\\'\\\"\\\\\\x9d",
        "0\\t\\r\\n\\'\\\"\\\\\\x9",
        "0\\t\\r\\n\\'\\\"\\\\\\x",
        "0\\t\\r\\n\\'\\\"\\\\\\",
        "0\\t\\r\\n\\'\\\"\\\\",
        "0\\t\\r\\n\\'\\\"\\",
        "0\\t\\r\\n\\'\\\"",
        "0\\t\\r\\n\\'\\",
        "0\\t\\r\\n\\'",
        "0\\t\\r\\n\\",
        "0\\t\\r\\n",
        "0\\t\\r\\",
        "0\\t\\r",
        "0\\t\\",
        "0\\t",
        "0\\",
        "0",
        "",
    ] {
        assert_eq!(format!("{escaped}"), s);
        escaped.next_back();
    }
}
