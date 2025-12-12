#[test]
fn str_from_utf8_ok() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = str::from_utf8(&sparkle_heart);
    assert_eq!(Ok("💖"), sparkle_heart);
}

#[test]
fn str_from_utf8_err() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8(&sparkle_heart_err);
    assert!(sparkle_heart_err.is_err());
}

#[test]
fn str_from_utf8_mut_ok() {
    let mut sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = str::from_utf8_mut(&mut sparkle_heart);
    assert_eq!(Ok("💖"), sparkle_heart.as_deref());
}

#[test]
fn str_from_utf8_mut_err() {
    let mut sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8_mut(&mut sparkle_heart_err);
    assert!(sparkle_heart_err.is_err());
}

// NOTE: Panicking version of this test is missing, since it is impossible to trigger a panic without undefined behaviour.
#[test]
fn str_from_utf8_unchecked() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe { str::from_utf8_unchecked(&sparkle_heart) };
    assert_eq!("💖", sparkle_heart);
}

// NOTE: Panicking version of this test is missing, since it is impossible to trigger a panic without undefined behaviour.
#[test]
fn str_from_utf8_unchecked_mut() {
    let mut sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe { str::from_utf8_unchecked_mut(&mut sparkle_heart) };
    assert_eq!("💖", sparkle_heart);
}

#[test]
fn str_utf8_error() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8(&sparkle_heart_err).unwrap_err();

    assert_eq!(sparkle_heart_err.valid_up_to(), 1);
    assert_eq!(sparkle_heart_err.error_len(), Some(1));
}
