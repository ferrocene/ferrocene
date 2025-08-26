// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::fs;
use std::path::Path;

/// Uncertified code is marked with this attribute
const MARKER: &str = "#[cfg(not(feature = \"ferrocene_certified\"))]";
const APPLICABLE: [&str; 20] = [
    "fn",
    "unsafe fn",
    "const fn",
    "const unsafe fn",
    "pub fn",
    "pub unsafe fn",
    "pub const fn",
    "pub const unsafe fn",
    "pub(crate) fn",
    "pub(crate) unsafe fn",
    "pub(crate) const fn",
    "pub(crate) const unsafe fn",
    "impl",
    "unsafe impl",
    "impl const",
    "pub impl",
    "pub(crate) impl",
    "mod",
    "pub mod",
    "pub(crate) mod",
];
const MAX_INDENTATION: usize = 60;
/// Mark code with this to stop rustc from instrumenting it
const TO_BE_INSERTED: &str = "#[coverage(off)]";

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
    file_content = inject_coverage_attribute(file_content);
    fs::write(file_path, file_content).unwrap();
}

fn inject_coverage_attribute(mut file_content: String) -> String {
    // This is, frankly, a very slow hack with a heavy hand, but it works for our purposes.
    for applicable in APPLICABLE {
        for i in 0..MAX_INDENTATION {
            let valid_marker = format!("{MARKER}\n{}{}", " ".repeat(i), applicable);
            let to_be_inserted =
                format!("{MARKER} {TO_BE_INSERTED}\n{}{}", " ".repeat(i), applicable);
            file_content = file_content.replace(&valid_marker, &to_be_inserted);
        }
    }
    file_content
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
        let after = inject_coverage_attribute(before.into());

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
    fn g() {
        #[cfg(not(feature = "ferrocene_certified"))]
        ::core::panicking::panic_nounwind_fmt();
    }

    #[cfg(not(feature = "ferrocene_certified"))]
    fn h() {}
}

#[cfg(not(feature = "ferrocene_certified"))]
macro_rules! marco_polo {}

#[cfg(not(feature = "ferrocene_certified"))]
pub struct Foo;

#[cfg(not(feature = "ferrocene_certified"))]
pub(crate) struct Foo;
"#;

    const TEST_FILE_CORRECT: &str = r#"
#[cfg(not(feature = "ferrocene_certified"))] #[cfg_attr(not(bootstrap), coverage(off))]
fn a{} {}

#[cfg(not(feature = "ferrocene_certified"))] #[cfg_attr(not(bootstrap), coverage(off))]
fn b() {}

#[cfg(feature = "ferrocene_certified")]
fn b() {}

#[cfg(not(feature = "ferrocene_certified"))] #[cfg_attr(not(bootstrap), coverage(off))]
mod c;

#[cfg(not(feature = "ferrocene_certified"))] #[cfg_attr(not(bootstrap), coverage(off))]
impl D {
    fn e() {}
}

impl F {
    fn g() {
        #[cfg(not(feature = "ferrocene_certified"))]
        ::core::panicking::panic_nounwind_fmt();
    }

    #[cfg(not(feature = "ferrocene_certified"))] #[cfg_attr(not(bootstrap), coverage(off))]
    fn h() {}
}

#[cfg(not(feature = "ferrocene_certified"))]
macro_rules! marco_polo {}

#[cfg(not(feature = "ferrocene_certified"))]
pub struct Foo;

#[cfg(not(feature = "ferrocene_certified"))]
pub(crate) struct Foo;
"#;
}
