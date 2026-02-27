use core::time::Duration;

#[test]
fn duration_methods() {
    let secs = Duration::new(1, 0);

    assert_eq!(secs.as_micros(), 1_000_000);

    assert_eq!(secs.as_millis(), 1_000);
    assert_eq!(secs.as_millis_f32(), 1_000.0);
    assert_eq!(secs.as_millis_f64(), 1_000.0);

    assert_eq!(secs.as_secs_f32(), 1.0);

    assert!(!secs.is_zero());
    assert!(Duration::from_secs(0).is_zero());

    assert_eq!(Duration::from_hours(1).as_secs(), 60 * 60);
    assert_eq!(Duration::from_days(1).as_secs(), 60 * 60 * 24);
    assert_eq!(Duration::from_weeks(1).as_secs(), 60 * 60 * 24 * 7);
}

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
#[should_panic = "cannot convert float seconds to Duration: value is negative"]
fn duration_try_from_secs_f32() {
    let _nope = Duration::from_secs_f32(-1.0);
}

// Cover `<core::time::TryFromFloatSecsError as core::fmt::Display>::fmt`
#[test]
fn try_from_floats_secs_error_fmt_nan() {
    assert_eq!(
        format!("{}", Duration::try_from_secs_f32(f32::NAN).unwrap_err()),
        "cannot convert float seconds to Duration: value is either too big or NaN"
    );
    assert_eq!(
        format!("{}", Duration::try_from_secs_f64(f64::NAN).unwrap_err()),
        "cannot convert float seconds to Duration: value is either too big or NaN"
    );
}
