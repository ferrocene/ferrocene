// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::error::Error;
use crate::report::Reporter;
use crate::utils::run_command;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

/// Check that the executables exist in the expected path, and that they have the correct permissions.
///
/// Some binaries are optional and only warn when not present.
pub(crate) fn check(reporter: &dyn Reporter, sysroot: &Path) -> Result<(), Error> {
    check_binary(reporter, sysroot, "rustc", CommitHashOf::Rust)?;
    check_binary(reporter, sysroot, "rustdoc", CommitHashOf::Rust)?;
    check_optional_binary(reporter, sysroot, "cargo", CommitHashOf::Cargo)?;

    Ok(())
}

fn check_binary(
    reporter: &dyn Reporter,
    sysroot: &Path,
    name: &str,
    hash: CommitHashOf,
) -> Result<(), Error> {
    let bin_dir = sysroot.join("bin");
    let bin = bin_dir.join(name);

    match std::fs::metadata(&bin) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(Error::MissingBinary { directory: bin_dir, name: name.into() });
            }
            if metadata.permissions().mode() & 0o555 != 0o555 {
                return Err(Error::WrongBinaryPermissions { path: bin });
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Err(Error::MissingBinary { directory: bin_dir, name: name.into() });
        }
        Err(err) => return Err(Error::MetadataFetchFailed { path: bin, error: err }),
    }

    let version_command_output = run_command(Command::new(&bin).arg("-vV"))
        .map_err(|error| Error::VersionFetchFailed { binary: name.into(), error })?;
    let version = parse_version_output(&version_command_output.stdout)
        .ok_or_else(|| Error::VersionParseFailed { binary: name.into() })?;

    for (field, expected, found) in [
        ("host", env!("SELFTEST_TARGET"), version.host),
        ("release", env!("CFG_RELEASE"), version.release),
        ("commit hash", hash.fetch().unwrap_or("unknown"), version.commit_hash),
    ] {
        if expected != found {
            return Err(Error::BinaryVersionMismatch {
                binary: name.into(),
                field: field.into(),
                expected: expected.into(),
                found: found.into(),
            });
        }
    }

    reporter.success(&format!("binary {name} is valid"));
    Ok(())
}

fn parse_version_output(output: &str) -> Option<VersionOutput<'_>> {
    let mut release = None;
    let mut commit_hash = None;
    let mut host = None;
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once(": ") else { continue };
        match key {
            "host" => host = Some(value),
            "commit-hash" => commit_hash = Some(value),
            "release" => release = Some(value),
            _ => {}
        }
    }

    Some(VersionOutput { release: release?, commit_hash: commit_hash?, host: host? })
}

#[derive(Debug)]
#[allow(dead_code)]
struct VersionOutput<'a> {
    release: &'a str,
    commit_hash: &'a str,
    host: &'a str,
}

fn check_optional_binary(
    reporter: &dyn Reporter,
    sysroot: &Path,
    name: &str,
    hash: CommitHashOf,
) -> Result<(), Error> {
    match check_binary(reporter, sysroot, name, hash) {
        Err(Error::MissingBinary { .. }) => {
            reporter.skipped(&format!("optional binary {name} (not present)"));
            Ok(())
        }
        other => other,
    }
}

enum CommitHashOf {
    Rust,
    Cargo,
}

impl CommitHashOf {
    fn fetch(&self) -> Option<&'static str> {
        match self {
            CommitHashOf::Rust => option_env!("SELFTEST_RUST_HASH"),
            CommitHashOf::Cargo => option_env!("SELFTEST_CARGO_HASH"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CommandError, CommandErrorKind};
    use crate::test_utils::{CliVersionContent, TestUtils};
    use std::ffi::OsString;

    #[test]
    fn test_check_binary_missing_file() {
        let utils = TestUtils::new();
        std::fs::create_dir_all(utils.sysroot().join("bin")).unwrap();
        assert_not_found(utils);
    }

    #[test]
    fn test_check_optional_binary_missing_file() {
        let utils = TestUtils::new();
        std::fs::create_dir_all(utils.sysroot().join("bin")).unwrap();

        check_optional_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust)
            .unwrap();
        utils.assert_report_skipped("optional binary rustc (not present)");
    }

    #[test]
    fn test_check_binary_missing_file_and_parent_directory() {
        let utils = TestUtils::new();
        assert_not_found(utils);
    }

    #[test]
    fn test_check_binary_file_is_a_directory() {
        let utils = TestUtils::new();
        std::fs::create_dir_all(utils.sysroot().join("bin").join("rustc")).unwrap();
        assert_not_found(utils);
    }

    #[test]
    fn test_check_binary_no_access_to_parent_directory() {
        let utils = TestUtils::new();

        let bin_dir = utils.sysroot().join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();

        let mut perms = bin_dir.metadata().unwrap().permissions();
        perms.set_mode(0);
        std::fs::set_permissions(&bin_dir, perms).unwrap();

        match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
            Ok(()) => panic!("should've failed"),
            Err(Error::MetadataFetchFailed { path, error }) => {
                assert_eq!(utils.sysroot().join("bin").join("rustc"), path);
                assert_eq!(std::io::ErrorKind::PermissionDenied, error.kind());
            }
            Err(err) => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn test_check_binary_cant_invoke_executable() {
        let utils = TestUtils::new();
        let bin = utils.bin("rustc").create();
        std::fs::write(&bin, &[]).unwrap(); // Broken content

        match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
            Ok(()) => panic!("should've failed"),
            Err(Error::VersionFetchFailed {
                binary,
                error: CommandError { path, args, kind: CommandErrorKind::StartupFailed { .. } },
            }) => {
                assert_eq!(bin, path);
                assert_eq!(vec![OsString::from("-vV")], args);
                assert_eq!("rustc", binary);
            }
            Err(err) => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn test_check_failing_binary() {
        let utils = TestUtils::new();
        let bin = utils.bin("rustc").exit(1).create();

        match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
            Ok(()) => panic!("should've failed"),
            Err(Error::VersionFetchFailed {
                binary,
                error: CommandError { path, args, kind: CommandErrorKind::Failure { output } },
            }) => {
                assert_eq!("rustc", binary);
                assert_eq!(vec![OsString::from("-vV")], args);
                assert_eq!(bin, path);
                assert_eq!(Some(1), output.status.code());
            }
            Err(err) => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn test_check_binary_with_invalid_output() {
        let utils = TestUtils::new();
        utils.bin("rustc").expected_args(&["-vV"]).stdout("this is not the output of -vV").create();

        match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
            Ok(()) => panic!("should've failed"),
            Err(Error::VersionParseFailed { binary }) => {
                assert_eq!("rustc", binary);
            }
            Err(err) => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn test_check_binary_wrong_release() {
        test_wrong_version_data(
            CliVersionContent { release: "0.0.0", ..CliVersionContent::default() },
            "release",
            CliVersionContent::default().release,
            "0.0.0",
        );
    }

    #[test]
    fn test_check_binary_wrong_host() {
        test_wrong_version_data(
            CliVersionContent { host: "bad-target-triple", ..CliVersionContent::default() },
            "host",
            CliVersionContent::default().host,
            "bad-target-triple",
        );
    }

    #[test]
    fn test_check_binary_wrong_commit_hash() {
        test_wrong_version_data(
            CliVersionContent { commit_hash: "0000000", ..CliVersionContent::default() },
            "commit hash",
            CliVersionContent::default().commit_hash,
            "0000000",
        );
    }

    fn test_wrong_version_data(
        content: CliVersionContent<'_>,
        expected_field: &str,
        expected_expected: &str,
        expected_found: &str,
    ) {
        let utils = TestUtils::new();
        utils.bin("rustc").expected_args(&["-vV"]).stdout(&content.serialize()).create();

        match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
            Ok(()) => panic!("should've failed"),
            Err(Error::BinaryVersionMismatch { binary, field, expected, found }) => {
                assert_eq!("rustc", binary);
                assert_eq!(expected_field, field);
                assert_eq!(expected_expected, expected);
                assert_eq!(expected_found, found);
            }
            Err(err) => panic!("unexpected error: {err}"),
        }
    }

    #[test]
    fn test_check_binary_wrong_permissions() {
        const PERMISSIONS: &[&[u32]] = &[
            // No permissions whatsoever
            &[0o000],
            // Only executable
            &[0o100, 0o010, 0o001, 0o110, 0o011, 0o101, 0o111],
            // Only writable
            &[0o200, 0o020, 0o002, 0o220, 0o022, 0o202, 0o222],
            // Only readable
            &[0o400, 0o040, 0o004, 0o440, 0o044, 0o404, 0o444],
            // Executable and writable
            &[0o300, 0o030, 0o003, 0o330, 0o033, 0o303, 0o333],
            // Readable and writable
            &[0o600, 0o060, 0o006, 0o660, 0o066, 0o606, 0o666],
            // Readable and executable
            &[0o500, 0o050, 0o005, 0o550, 0o055, 0o505],
            // Readable, writable and executable
            &[0o700, 0o070, 0o007, 0o770, 0o077, 0o707],
        ];

        let utils = TestUtils::new();
        let bin = utils.bin("rustc").create();

        for mode in PERMISSIONS.iter().copied().flatten() {
            eprintln!("checking {mode:o}");

            let mut perms = bin.metadata().unwrap().permissions();
            perms.set_mode(*mode);
            std::fs::set_permissions(&bin, perms).unwrap();

            match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
                Ok(()) => panic!("should've failed"),
                Err(Error::WrongBinaryPermissions { path }) => {
                    assert_eq!(utils.sysroot().join("bin").join("rustc"), path);
                }
                Err(err) => panic!("unexpected error: {err}"),
            }
        }
    }

    #[test]
    fn test_check_binary_good_permissions() {
        let utils = TestUtils::new();
        for mode in [0o777, 0o775, 0o755, 0o555] {
            eprintln!("checking {mode:o}");
            utils.bin("rustc").mode(mode).behaves_like_vV().create();
            check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust).unwrap();
            utils.assert_report_success("binary rustc is valid");
        }
    }

    #[track_caller]
    fn assert_not_found(utils: TestUtils) {
        match check_binary(utils.reporter(), utils.sysroot(), "rustc", CommitHashOf::Rust) {
            Ok(()) => panic!("should've failed"),
            Err(Error::MissingBinary { directory, name }) => {
                assert_eq!(utils.sysroot().join("bin"), directory);
                assert_eq!("rustc", name);
            }
            Err(err) => panic!("unexpected error: {err}"),
        }
    }
}
