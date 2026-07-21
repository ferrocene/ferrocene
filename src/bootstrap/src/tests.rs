// This entire file is a Ferrocene addition
use crate::Build;
use crate::utils::tests::{ConfigBuilder, TestCtx};

#[test]
fn test_ferrocene_version_string() {
    // Check that the Rust and Ferrocene parts of the version string (displayed by
    // `rustc --version` etc.) remain in sync

    let config = TestCtx::new().config("test").create_config();
    let build = Build::new(config);

    let version_string = build.rust_version();

    let rust_channel = build.config.channel;
    let rust_version = build.version;
    let ferrocene_channel = build.config.ferrocene_raw_channel;
    let ferrocene_version = build.ferrocene_version;

    eprintln!("Rust channel: {rust_channel}");
    eprintln!("Rust version: {rust_version}");
    eprintln!("Ferrocene channel: {ferrocene_channel}");
    eprintln!("Ferrocene version: {ferrocene_version}");
    eprintln!("");

    let (expected_rust_part, expected_ferrocene_part) = match (
        &rust_channel[..],
        &ferrocene_channel[..],
    ) {
        ("dev", "rolling") => (format!("{rust_version}-dev"), "Ferrocene nightly".to_owned()),
        ("nightly", "rolling") => {
            (format!("{rust_version}-nightly"), "Ferrocene nightly".to_owned())
        }
        ("beta", "rolling") => (format!("{rust_version}-beta"), "Ferrocene pre-rolling".to_owned()),
        ("stable", "rolling") => (rust_version.to_owned(), "Ferrocene rolling".to_owned()),
        ("stable", _) => (rust_version.to_owned(), format!("Ferrocene {}", ferrocene_version)),
        _ => panic!(
            "error: unsupported channel configuration: rust '{rust_channel}' and ferrocene '{ferrocene_channel}'"
        ),
    };

    eprintln!("Expected Rust part of version string: {expected_rust_part}");
    eprintln!("Expected Ferrocene part of version string: {expected_ferrocene_part}");
    eprintln!("Actual version string: {version_string}");

    assert!(version_string.starts_with(&expected_rust_part));
    assert!(version_string.contains(&expected_ferrocene_part));
}
