// Covers `core::alloc::layout::Layout::is_size_align_valid`'s None
#[test]
fn test_layout_is_size_align_valid_none() {
    use std::alloc::Layout;
    assert!(Layout::from_size_align(0, 0).is_err());
    assert!(Layout::from_size_align(0, 3).is_err());
}
