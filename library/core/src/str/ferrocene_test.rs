pub fn test_private_fn_adapters() {
    use crate::str::{UnsafeBytesToStr, LinesMap, IsWhitespace, IsNotEmpty, IsAsciiWhitespace};

    assert!(IsAsciiWhitespace.call_once((&b' ',)));
    assert!(!IsAsciiWhitespace.call_once((&b'a',)));

    assert!(IsNotEmpty.call_once((&"x",)));
    assert!(!IsNotEmpty.call_once((&"",)));

    assert!(IsWhitespace.call_once((' ',)));
    assert!(!IsWhitespace.call_once(('x',)));

    assert_eq!(LinesMap.call_once(("line\r\n",)), "line");

    let bytes = b"hello";
    assert_eq!(UnsafeBytesToStr.call_once((bytes,)), "hello");
}
