// Cover `<core::char::EscapeDebug as core::iter::traits::iterator::Iterator>::count`
#[test]
fn test_char_escape_debug_iterator_count() {
    assert_eq!(' '.escape_debug().count(), 1);
    assert_eq!('n'.escape_debug().count(), 1);
    assert_eq!('\n'.escape_debug().count(), 2);
}
