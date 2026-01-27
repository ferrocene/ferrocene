// Covers: <[core::ascii::ascii_char::AsciiChar]>::as_bytes
#[test]
fn ascii_char_slice_as_bytes() {
    let a = [core::ascii::Char::Null; 5];
    assert_eq!(a.as_bytes(), [0; 5]);
}

// Covers:
// - <core::ascii::EscapeDefault as core::iter::traits::iterator::Iterator>::advance_by
// - <core::ascii::EscapeDefault as core::iter::traits::iterator::Iterator>::count
// - <core::ascii::EscapeDefault as core::iter::traits::iterator::Iterator>::last
// - <core::ascii::EscapeDefault as core::iter::traits::iterator::Iterator>::size_hint
#[test]
fn escape_default_iter() {
    let a = core::ascii::escape_default(0);

    assert_eq!(a.clone().advance_by(5), Err(core::num::NonZero::new(1).unwrap()));
    assert_eq!(a.clone().count(), 4);
    assert_eq!(a.clone().last(), Some(48));
    assert_eq!(a.clone().size_hint(), (4, Some(4)));
}
