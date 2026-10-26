// Cover `<core::char::EscapeDebug as core::iter::traits::iterator::Iterator>::count`
#[test]
fn test_char_escape_debug_iterator_count() {
    assert_eq!(' '.escape_debug().count(), 1);
    assert_eq!('n'.escape_debug().count(), 1);
    assert_eq!('\n'.escape_debug().count(), 2);
}

// Cover the "supplementary private-use areas" in `core::char::methods::<impl char>::is_assigned`
// These areas cover codepoints U+E000..U+F8FF, U+F0000..U+FFFFD and U+100000..U+10FFFD (inclusive)
//
// The other cases in this function are covered by the tests in `library/alloctests/tests/str.rs`,
// via `core::char::methods::<impl char>::escape_debug()`. But that function has an early exit for
// the private-use areas, meaning it never calls `is_assigned()` for these characters.
#[test]
fn test_char_private_use_areas() {
    assert!('\u{e000}'.is_assigned());
    assert!('\u{f8ff}'.is_assigned());
    assert!('\u{f0000}'.is_assigned());
    assert!('\u{ffffd}'.is_assigned());
    assert!(!'\u{ffffe}'.is_assigned());
    assert!(!'\u{fffff}'.is_assigned());
    assert!('\u{100000}'.is_assigned());
    assert!('\u{10fffd}'.is_assigned());
    assert!(!'\u{10fffe}'.is_assigned());
    assert!(!'\u{10ffff}'.is_assigned());
}
