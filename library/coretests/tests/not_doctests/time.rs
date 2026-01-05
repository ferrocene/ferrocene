use core::time::Duration;

// core::time::Duration::checked_add
#[test]
fn duration_checked_add() {
    let one_billion = 1_000_000_000;
    let max_duration = Duration::new(u64::MAX, one_billion - 1);
    let more_duration = Duration::new(0, 2);
    assert_eq!(max_duration.checked_add(more_duration), None);
}

// core::time::Duration::from_secs_f32
#[test]
#[cfg_attr(feature = "ferrocene_certified_runtime", should_panic = "{e}")]
#[cfg_attr(
    not(feature = "ferrocene_certified_runtime"),
    should_panic = "cannot convert float seconds to Duration: value is negative"
)]
fn duration_try_from_secs_f32() {
    let _nope = Duration::from_secs_f32(-1.0);
}
