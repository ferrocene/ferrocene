// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;
use std::path::Path;
use std::sync::LazyLock;

/// Uncertified code is marked with this attribute
const MARKER: &str = "#[cfg(not(feature = \"ferrocene_certified\"))]";
/// Mark code with this to stop rustc from instrumenting it
const TO_BE_INSERTED: &str = "#[coverage(off)]";
// Note: Has to be a lock to be used in a static. Will only ever be called from
// one thread, so it should not be blocking in reality.
static MARKER_PLUS_TO_BE_INSERTED: LazyLock<String> =
    LazyLock::new(|| format!("{MARKER} {TO_BE_INSERTED}"));

fn main() {
    // The path to inject the coverage(off) attributes.
    // Note: hardcoded because it is only used for libcore atm. Can be made
    // into a CLI argument if necessary.
    let path = "library/core";
    handle_dir(path);
}

fn handle_dir(dir_path: impl AsRef<Path>) {
    let dir = fs::read_dir(&dir_path).expect(&format!("{:?}", dir_path.as_ref()));

    for file in dir {
        let path = file.unwrap().path();

        if path.is_dir() {
            handle_dir(&path); // recurse into subdirectories
        } else if !is_rust_file(&path) {
            continue; // skip non Rust files
        } else {
            handle_file(&path);
        }
    }
}

fn handle_file(file_path: &Path) {
    let mut file_content = fs::read_to_string(&file_path).unwrap();
    file_content = inject_coverage_attribute(&file_content);
    fs::write(file_path, file_content).unwrap();
}

fn inject_coverage_attribute(file_content: &str) -> String {
    file_content.replace(MARKER, &MARKER_PLUS_TO_BE_INSERTED)
}

fn is_rust_file(file_path: &Path) -> bool {
    file_path.extension().unwrap() == "rs"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_coverage_attribute() {
        // Arrange
        let before = TEST_FILE_BEFORE;
        let correct = TEST_FILE_CORRECT;

        // Act
        let after = inject_coverage_attribute(before);

        // Assert
        assert_eq!(after, correct);
    }

    #[test]
    fn test_is_rust_file() {
        // Arrange
        for (file, correct) in [
            ("src/lib.rs", true),
            ("src/module.rs", true),
            ("src/module/module2.rs", true),
            ("README.md", false),
            ("rs.md", false),
        ] {
            // Act
            let b = is_rust_file(Path::new(file));

            // Assert
            assert_eq!(b, correct);
        }
    }

    const TEST_FILE_BEFORE: &str = r#"
#[cfg(not(feature = "ferrocene_certified"))]
fn a{} {}

#[cfg(not(feature = "ferrocene_certified"))]
fn b() {}

#[cfg(feature = "ferrocene_certified")]
fn b() {}

#[cfg(not(feature = "ferrocene_certified"))]
mod c;

#[cfg(not(feature = "ferrocene_certified"))]
impl D {
    fn e() {}
}

impl F {
    fn g() {}

    #[cfg(not(feature = "ferrocene_certified"))]
    fn h() {}
}
"#;

    const TEST_FILE_CORRECT: &str = r#"
#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
fn a{} {}

#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
fn b() {}

#[cfg(feature = "ferrocene_certified")]
fn b() {}

#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
mod c;

#[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
impl D {
    fn e() {}
}

impl F {
    fn g() {}

    #[cfg(not(feature = "ferrocene_certified"))] #[coverage(off)]
    fn h() {}
}
"#;
}
