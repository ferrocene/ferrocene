pub fn test_escape_byte_call_once(a: u8, b: u8) {
    use super::EscapeByte;
    let result = iter_to_array(EscapeByte::call_once(EscapeByte, (&a,)));
    let expected = iter_to_array(core::ascii::Char::from_u8(b).unwrap().escape_ascii());

    assert_eq!(result, expected);
}

fn iter_to_array<I>(iter: I) -> [u8; 8]
where
    I: IntoIterator<Item = u8>,
{
    let mut array = [0; _];
    let mut i = 0;
    for byte in iter {
        array[i] = byte;
        i += 1;
    }
    array
}
