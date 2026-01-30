// Cover `core::unicode::printable::is_printable`
#[test]
fn test_unicode_is_printable() {
    fn string(c: char) -> String {
        let iter: String = c.escape_debug().collect();
        let disp: String = c.escape_debug().to_string();
        assert_eq!(iter, disp);
        iter
    }

    assert_eq!(string('\u{2a6e0}'), "\\u{2a6e0}");
    assert_eq!(string('\u{2b81e}'), "\\u{2b81e}");
    assert_eq!(string('\u{2ceae}'), "\\u{2ceae}");
    assert_eq!(string('\u{2ebe1}'), "\\u{2ebe1}");
    assert_eq!(string('\u{2ee5e}'), "\\u{2ee5e}");
    assert_eq!(string('\u{2fa1e}'), "\\u{2fa1e}");
    assert_eq!(string('\u{3134b}'), "\\u{3134b}");
    assert_eq!(string('\u{3347a}'), "\\u{3347a}");
    assert_eq!(string('\u{e01f0}'), "\\u{e01f0}");
    assert_eq!(string('\u{20000}'), "𠀀");
}

// Cover `core::unicode::unicode_data::skip_search`
#[test]
fn test_unicode_skip_search() {
    fn string(c: char) -> String {
        let iter: String = c.escape_debug().collect();
        let disp: String = c.escape_debug().to_string();
        assert_eq!(iter, disp);
        iter
    }

    // 0x300 makes `binary_search_by_key` return `Ok`
    assert_eq!(string('\u{300}'), "\\u{300}");
}
