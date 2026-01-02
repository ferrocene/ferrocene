// core::time::Duration::checked_add
#[test]
fn duration_checked_add() {
    let one_billion = 1_000_000_000;
    let max_duration = core::time::Duration::new(u64::MAX, one_billion - 1);
    let more_duration = core::time::Duration::new(0, 2);
    assert_eq!(max_duration.checked_add(more_duration), None);
}
