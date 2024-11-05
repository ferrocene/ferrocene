// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::Path;

use crate::error::Error;
use crate::linkers::Linker;
use crate::report::Reporter;

static SUPPORTED_TARGETS: &[TargetSpec] = &[
    #[cfg(target_arch = "x86_64")]
    TargetSpec { triple: "x86_64-unknown-linux-gnu", std: true, linker: Linker::HostCc },
    #[cfg(target_arch = "x86_64")]
    TargetSpec {
        triple: "aarch64-unknown-linux-gnu",
        std: true,
        linker: Linker::CrossCc(&["aarch64-linux-gnu-"]),
    },
    #[cfg(target_arch = "aarch64")]
    TargetSpec { triple: "aarch64-unknown-linux-gnu", std: true, linker: Linker::HostCc },
    #[cfg(target_arch = "aarch64")]
    TargetSpec {
        triple: "x86_64-unknown-linux-gnu",
        std: true,
        linker: Linker::CrossCc(&["x86_64-linux-gnu-"]),
    },
    TargetSpec { triple: "aarch64-unknown-none", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "armv8r-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "thumbv8m.base-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "thumbv8m.main-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "thumbv8m.main-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "thumbv7em-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "thumbv7em-none-eabi", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "armv7r-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "armebv7r-none-eabihf", std: false, linker: Linker::BundledLld },
    TargetSpec { triple: "aarch64-apple-darwin", std: true, linker: Linker::BundledLld },
    TargetSpec { triple: "x86_64-apple-darwin", std: true, linker: Linker::BundledLld },
    TargetSpec { triple: "x86_64-pc-windows-msvc", std: true, linker: Linker::BundledLld },
    TargetSpec { triple: "aarch64-unknown-nto-qnx710", std: true, linker: Linker::BundledLld },
    TargetSpec { triple: "x86_64-pc-nto-qnx710", std: true, linker: Linker::BundledLld },
];

#[derive(Debug)]
pub(crate) struct TargetSpec {
    /// The rustc triple for the target
    pub(crate) triple: &'static str,
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
    let target_dir = sysroot.join("lib").join("rustlib").join(target.triple);
    if !target_dir.is_dir() {
        // Target not present, ignore it.
        return Ok(CheckTargetOutcome::Missing);
    }

    let expected_libs = match target.std {
        true => ["core", "alloc", "std", "test", "proc_macro"].as_slice(),
        false => ["core", "alloc"].as_slice(),
    };
    check_libraries(target, &target_dir, expected_libs)?;

    reporter.success(&format!("target installed correctly: {}", target.triple));
    Ok(CheckTargetOutcome::Found)
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
            return Err(Error::DuplicateTargetLibrary { target: target.triple.into(), library });
        }
        expected_to_find.remove(library.as_str());
    }

    if let Some(library) = expected_to_find.iter().next() {
        Err(Error::TargetLibraryMissing {
            target: target.triple.into(),
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
        let triple = "x86_64-unknown-linux-gnu";
        let target = TargetSpec { triple, std: true, linker: Linker::HostCc };

        let utils = TestUtils::new();
        utils
            .target(triple)
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("std", "0123456789abcdef")
            .lib("test", "0123456789abcdef")
            .lib("proc_macro", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        assert_eq!(
            CheckTargetOutcome::Found,
            check_target(utils.reporter(), utils.sysroot(), &target).unwrap()
        );
        utils.assert_report_success("target installed correctly: x86_64-unknown-linux-gnu");
    }

    #[test]
    fn test_check_target_no_std() {
        let triple = "x86_64-unknown-none";
        let target = TargetSpec { triple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils
            .target(triple)
            .lib("core", "0123456789abcdef")
            .lib("alloc", "0123456789abcdef")
            .lib("other", "0123456789abcdef") // Unknown libraries are ignored
            .create();

        assert_eq!(
            CheckTargetOutcome::Found,
            check_target(utils.reporter(), utils.sysroot(), &target).unwrap()
        );
        utils.assert_report_success(format!("target installed correctly: {triple}").as_str());
    }

    #[test]
    fn test_check_target_missing_library() {
        let triple = "x86_64-unknown-none";
        let target = TargetSpec { triple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils.target(triple).lib("core", "0123456789abcdef").create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::TargetLibraryMissing { target, library }) => {
                assert_eq!(target, triple);
                assert_eq!(library, "alloc");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_target_duplicate_required_library() {
        let triple = "x86_64-unknown-none";
        let target = TargetSpec { triple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils
            .target(triple)
            .lib("core", "0123456789abcdef")
            .lib("core", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::DuplicateTargetLibrary { target, library }) => {
                assert_eq!(target, triple);
                assert_eq!(library, "core");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_check_target_duplicate_other_library() {
        let triple = "x86_64-unknown-none";
        let target = TargetSpec { triple, std: false, linker: Linker::BundledLld };

        let utils = TestUtils::new();
        utils
            .target(triple)
            .lib("core", "0123456789abcdef")
            .lib("other", "0123456789abcdef")
            .lib("other", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .create();

        match check_target(utils.reporter(), utils.sysroot(), &target) {
            Err(Error::DuplicateTargetLibrary { target, library }) => {
                assert_eq!(target, triple);
                assert_eq!(library, "other");
            }
            other => panic!("unexpected result: {other:?}"),
        }
        utils.assert_no_reports();
    }

    #[test]
    fn test_find_libraries_in() {
        let triple = "x86_64-unknown-linux-gnu";

        let utils = TestUtils::new();
        utils
            .target(triple)
            .lib("core", "0123456789abcdef")
            .lib("core", "abcdef0123456789")
            .lib("alloc", "0123456789abcdef")
            .lib("proc_macro", "0123456789abcdef")
            .create();

        let lib_dir = utils.target_dir(triple).join("lib");
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
}
