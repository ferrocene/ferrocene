// covers `core::mem::conjure_zst`
#[test]
fn test_conjure_zst() {
    assert_eq!((), unsafe { core::mem::conjure_zst::<()>() });
}
