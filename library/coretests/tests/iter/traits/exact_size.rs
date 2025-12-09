#[test]
fn mut_ref_is_empty() {
    assert!((&mut (0..0)).is_empty());
    assert!(!(&mut (0..1)).is_empty());
}

#[test]
fn chunks_exact_is_empty() {
    assert!([0; 10].chunks_exact(11).is_empty());
    assert!(![0; 10].chunks_exact(2).is_empty());

    assert!([0; 10].chunks_exact_mut(11).is_empty());
    assert!(![0; 10].chunks_exact_mut(2).is_empty());
}
