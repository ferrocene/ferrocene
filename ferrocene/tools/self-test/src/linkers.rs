// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::error::Error;
use crate::report::Reporter;
use crate::targets::Target;
use crate::utils::{find_binary_in_path, run_command, DisplayList};
use crate::Environment;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Linker {
    #[cfg_attr(not(test), allow(unused))]
    BundledLld,
    GccUbuntu18 { target: &'static str, mode: GccMode },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum GccMode {
    Normal,
    BareMetal,
}

pub(crate) fn check_and_add_rustflags(
    reporter: &dyn Reporter,
    environment: &Environment,
    sysroot: &Path,
    targets: &mut [Target],
) -> Result<(), Error> {
    // Multiple installed targets might use the same linker, deduplicate them.
    let mut linkers = BTreeMap::new();
    for target in targets {
        linkers.entry(target.linker).or_insert_with(Vec::new).push(target);
    }

    for (linker, targets) in linkers {
        let triples = targets.iter().map(|t| t.triple.to_string()).collect::<Vec<_>>();
        let (bin, flavor, extra_flags): (_, _, &[&str]) = match linker {
            Linker::BundledLld => (check_bundled_lld(reporter, sysroot, &triples)?, "ld.lld", &[]),
            Linker::GccUbuntu18 { target: gcc_target, mode } => (
                check_gcc(reporter, environment, &triples, gcc_target, mode, [7, 5])?,
                "gcc",
                match mode {
                    GccMode::Normal => &[],
                    GccMode::BareMetal => &["-ffreestanding", "-nostdlib"],
                },
            ),
        };
        let bin = bin.to_str().ok_or_else(|| Error::NonUtf8Path { path: bin.clone() })?;
        for target in targets {
            target.rustflags.push(format!("-Clinker={bin}"));
            target.rustflags.push(format!("-Clinker-flavor={flavor}"));
            for flag in extra_flags {
                target.rustflags.push(format!("-Clink-arg={flag}"));
            }
        }
    }

    Ok(())
}

fn check_bundled_lld(
    reporter: &dyn Reporter,
    sysroot: &Path,
    targets: &[String],
) -> Result<PathBuf, Error> {
    let path = sysroot
        .join("lib")
        .join("rustlib")
        .join(env!("SELFTEST_TARGET"))
        .join("bin")
        .join("rust-lld");

    if path.is_file() {
        reporter.success(&format!("bundled linker detected, for target {}", DisplayList(targets)));
        Ok(path)
    } else {
        Err(Error::BundledLinkerMissing { targets: targets.into() })
    }
}

fn check_gcc(
    reporter: &dyn Reporter,
    environment: &Environment,
    targets: &[String],
    gcc_target: &str,
    gcc_mode: GccMode,
    expected_version: [u8; 2],
) -> Result<PathBuf, Error> {
    let name = format!("{gcc_target}-gcc");
    let bin = find_binary_in_path(environment, &name).map_err(|error| Error::LinkerNotFound {
        targets: targets.into(),
        name: name.clone(),
        error,
    })?;

    let version_output = run_command(Command::new(&bin).arg("--version")).map_err(|error| {
        Error::LinkerVersionFetchFailed { targets: targets.into(), name: name.clone(), error }
    })?;

    let parsed_version = extract_gcc_version(&name, &version_output.stdout).ok_or_else(|| {
        Error::LinkerVersionParseFailed { targets: targets.into(), name: name.clone() }
    })?;

    if &parsed_version.parsed[..2] == &expected_version {
        let suffix = match gcc_mode {
            GccMode::Normal => "",
            GccMode::BareMetal => " (bare metal)",
        };
        reporter.success(&format!(
            "linker {name} {}{suffix} detected, for target {}",
            parsed_version.raw,
            DisplayList(targets)
        ));
        Ok(bin)
    } else {
        Err(Error::UnsupportedLinkerVersion {
            targets: targets.into(),
            name: name.into(),
            expected: format!(
                "{}.x",
                expected_version.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(".")
            ),
            found: parsed_version.raw,
        })
    }
}

fn extract_gcc_version(binary_name: &str, output: &str) -> Option<GccVersion> {
    let first_line = output.lines().next()?;
    let mut segments = first_line.split(' ');

    // Ensure what we called actually looks like GCC.
    // GCC outputs the name of the binary you called as the first word in the version output. If
    // you renamed a GCC binary to "foo", the first word would be "foo" rather than "gcc".
    let found_binary_name = segments.next()?;
    if found_binary_name != binary_name {
        return None;
    }

    // Parse the version number at the end of the first line
    let raw = segments.next_back()?;
    Some(GccVersion {
        parsed: raw.split('.').map(|n| n.parse().ok()).collect::<Option<_>>()?,
        raw: raw.into(),
    })
}

struct GccVersion {
    raw: String,
    parsed: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{CommandError, CommandErrorKind, FindBinaryInPathError};
    use crate::targets::TargetSpec;
    use crate::test_utils::TestUtils;
    use std::ffi::OsString;

    #[test]
    fn test_check() {
        // Tests both the deduplication of checks, and the specific gcc version we look for.

        let utils = TestUtils::new();
        let lld = utils
            .bin("rust-lld")
            .for_target(env!("SELFTEST_TARGET"))
            .create()
            .to_str()
            .unwrap()
            .to_string();
        let gcc_x86_64 = utils
            .bin("x86_64-linux-gnu-gcc")
            .stdout("x86_64-linux-gnu-gcc 7.5.3")
            .expected_args(&["--version"])
            .external()
            .create()
            .to_str()
            .unwrap()
            .to_string();
        let gcc_aarch64 = utils
            .bin("aarch64-linux-gnu-gcc")
            .stdout("aarch64-linux-gnu-gcc 7.5.8.9")
            .expected_args(&["--version"])
            .external()
            .create()
            .to_str()
            .unwrap()
            .to_string();

        let mut targets = [
            Target {
                spec: &TargetSpec {
                    triple: "x86_64-unknown-linux-gnu",
                    std: true,
                    linker: Linker::GccUbuntu18 {
                        target: "x86_64-linux-gnu",
                        mode: GccMode::Normal,
                    },
                },
                rustflags: Vec::new(),
            },
            Target {
                spec: &TargetSpec {
                    triple: "x86_64-unknown-linux-gnu2",
                    std: true,
                    linker: Linker::GccUbuntu18 {
                        target: "x86_64-linux-gnu",
                        mode: GccMode::Normal,
                    },
                },
                rustflags: Vec::new(),
            },
            Target {
                spec: &TargetSpec {
                    triple: "aarch64-unknown-linux-gnu",
                    std: true,
                    linker: Linker::GccUbuntu18 {
                        target: "aarch64-linux-gnu",
                        mode: GccMode::Normal,
                    },
                },
                rustflags: Vec::new(),
            },
            Target {
                spec: &TargetSpec {
                    triple: "aarch64-unknown-none",
                    std: false,
                    linker: Linker::BundledLld,
                },
                rustflags: Vec::new(),
            },
            Target {
                spec: &TargetSpec {
                    triple: "thumbv7m-none-eabi",
                    std: false,
                    linker: Linker::BundledLld,
                },
                rustflags: Vec::new(),
            },
            Target {
                spec: &TargetSpec {
                    triple: "x86_64-unknown-none",
                    std: false,
                    linker: Linker::GccUbuntu18 {
                        target: "x86_64-linux-gnu",
                        mode: GccMode::BareMetal,
                    },
                },
                rustflags: Vec::new(),
            },
        ];

        check_and_add_rustflags(utils.reporter(), utils.env(), utils.sysroot(), &mut targets)
            .unwrap();

        assert_eq!(
            &[format!("-Clinker={gcc_x86_64}"), format!("-Clinker-flavor=gcc")],
            &targets[0].rustflags[..]
        );
        assert_eq!(
            &[format!("-Clinker={gcc_x86_64}"), format!("-Clinker-flavor=gcc")],
            &targets[1].rustflags[..]
        );
        assert_eq!(
            &[format!("-Clinker={gcc_aarch64}"), format!("-Clinker-flavor=gcc")],
            &targets[2].rustflags[..]
        );
        assert_eq!(
            &[format!("-Clinker={lld}"), format!("-Clinker-flavor=ld.lld")],
            &targets[3].rustflags[..]
        );
        assert_eq!(
            &[format!("-Clinker={lld}"), format!("-Clinker-flavor=ld.lld")],
            &targets[4].rustflags[..]
        );
        assert_eq!(
            &[
                format!("-Clinker={gcc_x86_64}"),
                "-Clinker-flavor=gcc".into(),
                "-Clink-arg=-ffreestanding".into(),
                "-Clink-arg=-nostdlib".into()
            ],
            &targets[5].rustflags[..]
        );

        utils.assert_report_success(
            "linker x86_64-linux-gnu-gcc 7.5.3 (bare metal) detected, for target x86_64-unknown-none",
        );
        utils.assert_report_success(
            "linker x86_64-linux-gnu-gcc 7.5.3 detected, \
             for target x86_64-unknown-linux-gnu and x86_64-unknown-linux-gnu2",
        );
        utils.assert_report_success(
            "linker aarch64-linux-gnu-gcc 7.5.8.9 detected, for target aarch64-unknown-linux-gnu",
        );
        utils.assert_report_success(
            "bundled linker detected, for target aarch64-unknown-none and thumbv7m-none-eabi",
        );
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_bundled_lld() {
        let utils = TestUtils::new();
        utils.bin("rust-lld").for_target(env!("SELFTEST_TARGET")).create();

        check_bundled_lld(
            utils.reporter(),
            utils.sysroot(),
            &["aarch64-unknown-none".into(), "thumbv7m-none-eabi".into()],
        )
        .unwrap();
        utils.assert_report_success(
            "bundled linker detected, for target aarch64-unknown-none and thumbv7m-none-eabi",
        );
    }

    #[test]
    fn test_check_bundled_lld_missing() {
        let utils = TestUtils::new();

        match check_bundled_lld(utils.reporter(), utils.sysroot(), &["thumbv7-none-eabi".into()]) {
            Err(Error::BundledLinkerMissing { targets }) => {
                assert_eq!(&["thumbv7-none-eabi".to_string()], &targets[..]);
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn test_check_gcc() {
        let utils = TestUtils::new();
        utils
            .bin("x86_64-linux-gnu-gcc")
            .stdout("x86_64-linux-gnu-gcc (Ubuntu 11.3.0-1ubuntu1~22.04) 11.3.0")
            .expected_args(&["--version"])
            .external()
            .create();

        check_gcc(
            utils.reporter(),
            utils.env(),
            &["x86_64-unknown-linux-gnu".into(), "other-target".into()],
            "x86_64-linux-gnu",
            GccMode::Normal,
            [11, 3],
        )
        .unwrap();
        utils.assert_report_success(
            "linker x86_64-linux-gnu-gcc 11.3.0 detected, \
             for target x86_64-unknown-linux-gnu and other-target",
        );
    }

    #[test]
    fn test_check_gcc_missing_linker_binary() {
        let utils = TestUtils::new();

        match check_gcc(
            utils.reporter(),
            utils.env(),
            &["x86_64-unknown-linux-gnu".into()],
            "x86_64-linux-gnu",
            GccMode::Normal,
            [11, 3],
        ) {
            Err(Error::LinkerNotFound {
                targets,
                name,
                error: FindBinaryInPathError::MissingBinary { name: binary_name },
            }) => {
                assert_eq!(&["x86_64-unknown-linux-gnu".to_string()], &targets[..]);
                assert_eq!("x86_64-linux-gnu-gcc", name);
                assert_eq!("x86_64-linux-gnu-gcc", binary_name);
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_gcc_invocation_failure() {
        let utils = TestUtils::new();
        let bin = utils.bin("x86_64-linux-gnu-gcc").external().exit(1).create();

        match check_gcc(
            utils.reporter(),
            utils.env(),
            &["x86_64-unknown-linux-gnu".into()],
            "x86_64-linux-gnu",
            GccMode::Normal,
            [11, 3],
        ) {
            Err(Error::LinkerVersionFetchFailed {
                targets,
                name,
                error: CommandError { path, args, kind: CommandErrorKind::Failure { output } },
            }) => {
                assert_eq!(&["x86_64-unknown-linux-gnu".to_string()], &targets[..]);
                assert_eq!("x86_64-linux-gnu-gcc", name);
                assert_eq!(Some(1), output.status.code());
                assert_eq!(bin, path);
                assert_eq!(vec![OsString::from("--version")], args);
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_gcc_invalid_output() {
        let utils = TestUtils::new();
        utils
            .bin("x86_64-linux-gnu-gcc")
            .external()
            .expected_args(&["--version"])
            .stdout("I'm not a version")
            .create();

        match check_gcc(
            utils.reporter(),
            utils.env(),
            &["x86_64-unknown-linux-gnu".into()],
            "x86_64-linux-gnu",
            GccMode::Normal,
            [11, 3],
        ) {
            Err(Error::LinkerVersionParseFailed { targets, name }) => {
                assert_eq!(&["x86_64-unknown-linux-gnu".to_string()], &targets[..]);
                assert_eq!("x86_64-linux-gnu-gcc", name);
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_gcc_unsupported_version() {
        let utils = TestUtils::new();
        utils
            .bin("x86_64-linux-gnu-gcc")
            .external()
            .expected_args(&["--version"])
            .stdout("x86_64-linux-gnu-gcc 1.2.3")
            .create();

        match check_gcc(
            utils.reporter(),
            utils.env(),
            &["x86_64-unknown-linux-gnu".into()],
            "x86_64-linux-gnu",
            GccMode::Normal,
            [11, 3],
        ) {
            Err(Error::UnsupportedLinkerVersion { targets, name, expected, found }) => {
                assert_eq!(&["x86_64-unknown-linux-gnu".to_string()], &targets[..]);
                assert_eq!("x86_64-linux-gnu-gcc", name);
                assert_eq!("11.3.x", expected);
                assert_eq!("1.2.3", found);
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_extract_gcc_version() {
        let valid_outputs = [
            "gcc 11.3.0",
            "gcc (Ubuntu 11.3.0-1ubuntu1~22.04) 11.3.0",
            "gcc (Ubuntu 11.3.0-1ubuntu1~22.04) 11.3.0\n\
             This is free software; see the source for copying conditions.  There is NO\n\
             warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.\n\n",
        ];
        for output in valid_outputs {
            let result = extract_gcc_version("gcc", output).unwrap();
            assert_eq!("11.3.0", result.raw);
            assert_eq!([11, 3, 0], result.parsed[..]);
        }

        // Invalid outputs
        assert!(extract_gcc_version("gcc", "").is_none());
        assert!(extract_gcc_version("gcc", "x86_64-linux-gnu-gcc 1.0.0").is_none());
        assert!(extract_gcc_version("gcc", "gcc 1.foo.0").is_none());
    }
}
