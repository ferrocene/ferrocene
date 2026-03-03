use core::bstr::ByteStr;
use std::ops::DerefMut;

// Covers `<core::bstr::ByteStr as core::ops::deref::DerefMut>::deref_mut`.
#[test]
fn test_bstr_deref_mut() {
    let mut buf = b"hello".to_vec();
    let slice = buf.as_mut_slice();
    let bstr = unsafe { core::mem::transmute::<&mut [u8], &mut ByteStr>(slice) };
    assert_eq!(bstr.deref_mut(), b"hello");
}
