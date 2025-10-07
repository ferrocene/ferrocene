// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::Path;
use std::process::Command;

use crate::error::Error;
use crate::linkers::Linker;
use crate::report::Reporter;
use crate::utils::run_command;

static SUPPORTED_TARGETS: &[TargetSpec] = &[
    // Targets with architecture specific tuning
    #[cfg(target_arch = "x86_64")]
    TargetSpec {
        tuple: "aarch64-unknown-linux-gnu",
        std: true,
        linker: Linker::CrossCc(&["aarch64-linux-gnu-"]),
    },
    #[cfg(target_arch = "aarch64")]
    TargetSpec { tuple: "aarch64-unknown-linux-gnu", std: true, linker: Linker::HostCc },
    #[cfg(target_arch = "x86_64")]
    TargetSpec {
        tuple: "aarch64-unknown-linux-musl",
        std: true,
        linker: Linker::CrossCc(&["aarch64-linux-musl-"]),
    },
    #[cfg(target_arch = "aarch64")]
    TargetSpec { tuple: "aarch64-unknown-linux-musl", std: true, linker: Linker::BundledLld },
    #[cfg(target_arch = "aarch64")]
    TargetSpec {
        tuple: "x86_64-unknown-linux-gnu",
        std: true,
        linker: Linker::CrossCc(&["x86_64-linux-gnu-"]),
    },
    #[cfg(target_arch = "x86_64")]
    TargetSpec { tuple: "x86_64-unknown-linux-gnu", std: true, linker: Linker::HostCc },
    #[cfg(target_arch = "aarch64")]
    TargetSpec {
        tuple: "x86_64-unknown-linux-musl",
        std: true,
        linker: Linker::CrossCc(&["x86_64-linux-musl-"]),
    },
    #[cfg(target_arch = "x86_64")]
    TargetSpec { tuple: "x86_64-unknown-linux-musl", std: true, linker: Linker::BundledLld },
    // Targets without architecture specific tuning
    TargetSpec { tuple: "aarch64-apple-darwin", std: true, linker: Linker::BundledLld },
    TargetSpec { tuple: "aarch64-unknown-none", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "aarch64-unknown-ferrocene.facade", std: true, linker: Linker::BundledLld },
    TargetSpec { tuple: "aarch64-unknown-nto-qnx710", std: true, linker: Linker::BundledLld },
    TargetSpec { tuple: "armebv7r-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "armv7r-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "armv8r-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec {
        tuple: "riscv64gc-unknown-linux-gnu",
        std: true,
        // https://www.embecosm.com/resources/tool-chain-downloads/#riscv-linux's toolchains use
        // the `riscv64-unknown-linux-gnu-` prefix instead of `riscv64-linux-gnu-`
        linker: Linker::CrossCc(&["riscv64-unknown-linux-gnu-"]),
    },
    TargetSpec { tuple: "thumbv6m-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "thumbv7em-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "thumbv7em-ferrocene.facade-eabi", std: true, linker: Linker::BundledLld },
    TargetSpec { tuple: "thumbv7em-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec {
        tuple: "thumbv7em-ferrocene.facade-eabihf",
        std: true,
        linker: Linker::BundledLld,
    },
    TargetSpec { tuple: "thumbv8m.base-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "thumbv8m.main-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "thumbv8m.main-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { tuple: "x86_64-pc-nto-qnx710", std: true, linker: Linker::BundledLld },
    TargetSpec { tuple: "x86_64-pc-windows-msvc", std: true, linker: Linker::BundledLld },
];

#[derive(Debug)]
pub(crate) struct TargetSpec {
    /// The rustc tuple for the target
    pub(crate) tuple: &'static str,
    /// Indicates if the target provides libstd.
    pub(crate) std: bool,
    /// Indicates if the target requires a system C compiler as a linker driver
    /// to ensure system libs are on the library path, and if so, what possible
    /// filename prefixes should be checked to find a suitable compiler.
    pub(crate) linker: Linker,
}

#[derive(Debug)]
pub(crate) struct Target {
    pub(crate) spec: &'static TargetSpec,
    pub(crate) rustflags: Vec<String>,
}

impl Deref for Target {
    type Target = TargetSpec;

    fn deref(&self) -> &Self::Target {
        self.spec
    }
}

/// Check which of the supported targets are installed.
pub(crate) fn check(reporter: &dyn Reporter, sysroot: &Path) -> Result<Vec<Target>, Error> {
    SUPPORTED_TARGETS.iter().try_fold(Vec::new(), |mut found, target| {
        match check_target(reporter, sysroot, target)? {
            CheckTargetOutcome::Missing => {}
            CheckTargetOutcome::Found => found.push(Target { spec: target, rustflags: Vec::new() }),
        }
        Ok(found)
    })
}

fn check_target(
    reporter: &dyn Reporter,
    sysroot: &Path,
    target: &TargetSpec,
) -> Result<CheckTargetOutcome, Error> {
    let target_dir = sysroot.join("lib").join("rustlib").join(target.tuple);
    if !target_dir.is_dir() {
        // Target not present, ignore it.
        return Ok(CheckTargetOutcome::Missing);
    }

    let expected_libs = match target.std {
        true => ["core", "alloc", "std", "test", "proc_macro"].as_slice(),
        false => ["core", "alloc"].as_slice(),
    };
    check_libraries(target, &target_dir, expected_libs)?;
    check_default_link_args(sysroot, target)?;

    reporter.success(&format!("target installed correctly: {}", target.tuple));
    Ok(CheckTargetOutcome::Found)
}

/// Check if the default link args for the target are what is expected
fn check_default_link_args(sysroot: &Path, target: &TargetSpec) -> Result<(), Error> {
    match target.linker {
        Linker::HostCc => (),
        Linker::BundledLld | Linker::CrossCc(_) => return Ok(()), // No default link args expected
    }

    let rustc = sysroot.join("bin").join("rustc");
    let temp_dir = tempfile::tempdir()
        .map_err(|error| Error::TemporaryCompilationDirectoryCreationFailed { error })?;
    let temp_main = temp_dir.path().join("main.rs");
    std::fs::write(&temp_main, "fn main() {}").map_err(|error| {
        Error::WritingSampleProgramFailed {
            name: "link_arg_example".into(),
            dest: temp_main.clone(),
            error,
        }
    })?;

    let mut command = Command::new(rustc);
    command.arg("--target");
    command.arg(target.tuple);
    command.arg("--print");
    command.arg("link-args");
    command.arg(temp_main);
    let output = run_command(&mut command)
        .map_err(|error| Error::sample_program_compilation_failed("link-args", error))?;

    // All our `HostCc` targets require `-fuse-ld=lld` to be passed
    let fuse_ld_arg = "-fuse-ld=lld";
    if !output.stdout.contains(fuse_ld_arg) {
        Err(Error::TargetDefaultLinkArgMissing {
            target: target.tuple.into(),
            link_arg: fuse_ld_arg.into(),
        })
    } else {
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CheckTargetOutcome {
    Missing,
    Found,
}

fn check_libraries(target: &TargetSpec, target_dir: &Path, expected: &[&str]) -> Result<(), Error> {
    let lib_dir = target_dir.join("lib");

    let mut expected_to_find = expected.iter().cloned().collect::<HashSet<_>>();
    for (library, count) in find_libraries_in(&lib_dir)?.into_iter() {
        if count > 1 {
            return Err(Error::DuplicateTargetLibrary { target: target.tuple.into(), library });
        }
        expected_to_find.remove(library.as_str());
    }

    if let Some(library) = expected_to_find.iter().next() {
        Err(Error::TargetLibraryMissing {
            target: target.tuple.into(),
            library: library.to_string(),
        })
    } else {
        Ok(())
    }
}

fn find_libraries_in(path: &Path) -> Result<HashMap<String, usize>, Error> {
    let map_err = |e| Error::TargetLibraryDiscoveryFailed { path: path.into(), error: e };

    let mut found = HashMap::new();
    for entry in path.read_dir().map_err(map_err)? {
        let path = entry.map_err(map_err)?.path();
        if !path.is_file() {
            continue;
        }
        let Some(library) = extract_library_name(&path) else { continue };

        *found.entry(library.into()).or_insert(0) += 1;
    }

    Ok(found)
}

/// Extract `name` from `libname-hash.rlib`.
fn extract_library_name(file_path: &Path) -> Option<&str> {
    let (library_name, hash) = file_path
        .file_name()?
        .to_str()?
        .strip_prefix("lib")?
        .strip_suffix(".rlib")?
        .rsplit_once('-')?;

    (hash.len() == 16 && hash.chars().all(|c| c.is_ascii_hexdigit()) && !library_name.is_empty())
        .then_some(library_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestUtils;

    #[test]
    fn test_check_target_std() {
        let tuple = "x86_64-unknown-linux-gnu";
        let target = TargetSpec { tuple, std: true, linker: Linker::HostCc };

        let utils = TestUtils::new();
        utils
            .target(tuple)
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("std", "0123456789abcdef")
            .lib("test", "0123456789abcdef")
            .lib("proc_macro", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        let _bin = utils
            .bin("rustc")
            .expected_args(&["--target", tuple, "--print", "link-args"])
            .expected_args_strict(false)
            .stdout("-fuse-ld=lld")
            .create();

        assert_eq!(
            CheckTargetOutcome::Found,
            check_target(utils.reporter(), utils.sysroot(), &target).unwrap()
        );
        utils.assert_report_success("target installed correctly: x86_64-unknown-linux-gnu");
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))] // Only on x86_64 Linux since the test is specific to that.
    #[test]
    #[should_panic]
    fn test_check_target_std_fails_if_lld_not_used() {
        let tuple = "x86_64-unknown-linux-gnu";
        let target = TargetSpec { tuple, std: true, linker: Linker::HostCc };

        let utils = TestUtils::new();
        utils
            .target(tuple)
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("std", "0123456789abcdef")
            .lib("test", "0123456789abcdef")
            .lib("proc_macro", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        let _bin = utils
            .bin("rustc")
            .expected_args(&["--target", tuple, "--print", "link-args"])
            .expected_args_strict(false)
            .stdout("-fuse-ld=not-lld-this-should-fail")
            .create();

        check_target(utils.reporter(), utils.sysroot(), &target).unwrap(); // Panic!
    }

    #[test]
    fn test_check_target_no_std() {
        let tuple = "x86_64-unknown-none";
        let target = TargetSpec { tuple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils
            .target(tuple)
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        assert_eq!(
            CheckTargetOutcome::Found,
            check_target(utils.reporter(), utils.sysroot(), &target).unwrap()
        );
        utils.assert_report_success(format!("target installed correctly: {tuple}").as_str());
    }

    #[test]
    fn test_check_target_missing_library() {
        let tuple = "x86_64-unknown-none";
        let target = TargetSpec { tuple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils.target(tuple).lib("core", "0123456789abcdef").create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::TargetLibraryMissing { target, library }) => {
                assert_eq!(target, tuple);
                assert_eq!(library, "alloc");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_target_duplicate_required_library() {
        let tuple = "x86_64-unknown-none";
        let target = TargetSpec { tuple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils
            .target(tuple)
            .lib("core", "0123456789abcdef")
            .lib("core", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::DuplicateTargetLibrary { target, library }) => {
                assert_eq!(target, tuple);
                assert_eq!(library, "core");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_target_duplicate_other_library() {
        let tuple = "x86_64-unknown-none";
        let target = TargetSpec { tuple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils
            .target(tuple)
            .lib("core", "0123456789abcdef")
            .lib("other", "0123456789abcdef")
            .lib("other", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::DuplicateTargetLibrary { target, library }) => {
                assert_eq!(target, tuple);
                assert_eq!(library, "other");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_find_libraries_in() {
        let tuple = "x86_64-unknown-linux-gnu";

        let utils = TestUtils::new();
        utils
            .target(tuple)
            .lib("core", "0123456789abcdef")
            .lib("core", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .lib("proc_macro", "0123456789abcdef")
            .create();

        let lib_dir = utils.target_dir(tuple).join("lib");
        std::fs::write(lib_dir.join("foo-0123456789abcdef.so"), b"").unwrap(); // Invalid files are not counted.

        let output = find_libraries_in(&lib_dir).unwrap();
        assert_eq!(output.len(), 3);
        assert_eq!(output.get("core"), Some(&2));
        assert_eq!(output.get("alloc"), Some(&1));
        assert_eq!(output.get("proc_macro"), Some(&1));
        assert_eq!(output.get("foo"), None)
    }

    #[test]
    fn test_find_libraries_in_missing_directory() {
        let temp = tempfile::tempdir().unwrap();
        let missing = temp.path().join("missing");

        let err = find_libraries_in(&missing).unwrap_err();
        if let Error::TargetLibraryDiscoveryFailed { path, error } = err {
            assert_eq!(missing, path);
            assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
        } else {
            panic!("wrong error type");
        }
    }

    #[test]
    fn test_extract_library_name() {
        assert_eq!(Some("core"), extract_library_name(Path::new("libcore-0123456789abcdef.rlib")));
        assert_eq!(
            Some("proc_macro"),
            extract_library_name(Path::new("libproc_macro-0123456789abcdef.rlib"))
        );

        let assert_fail = |name: &str| {
            assert!(
                extract_library_name(Path::new(name)).is_none(),
                "{name} is treated as valid but should be wrong"
            )
        };
        assert_fail("libcore-0123456789abcde.rlib"); // Hash too short
        assert_fail("libcore-0123456789abcdef0.rlib"); // Hash too long
        assert_fail("libcore-0123456789abcdeg.rlib"); // Non-hexdigit in hash
        assert_fail("core-0123456789abcdef.rlib"); // No "lib" prefix
        assert_fail("lib-0123456789abcdef.rlib"); // No library name
        assert_fail("libcore-0123456789abcdef.so"); // Different extension
        assert_fail("libcore-0123456789abcdef"); // No extension
    }

    fn check_target_default_link_args(tuple: &'static str) -> Result<(), Error> {
        let target = TargetSpec { tuple, std: true, linker: Linker::HostCc };

        let utils = TestUtils::new();
        let _bin = utils
            .bin("rustc")
            .expected_args(&["--target", tuple, "--print", "link-args"])
            .expected_args_strict(false)
            .stdout("-fuse-ld=lld")
            .create();
        check_default_link_args(utils.sysroot(), &target)
    }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    #[test]
    fn test_check_target_default_link_args_x86_64() -> Result<(), Error> {
        check_target_default_link_args("x86_64-unknown-linux-gnu")
    }

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    #[test]
    fn test_check_target_default_link_args_x86_64() -> Result<(), Error> {
        check_target_default_link_args("aarch64-unknown-linux-gnu")
    }
}
