// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use similar::{ChangeTag, TextDiff};

/// Print a diff `expected` and `actual`
pub fn diff_text(expected: &str, actual: &str) {
    for change in TextDiff::from_lines(expected, actual).iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => '-',
            ChangeTag::Insert => '+',
            ChangeTag::Equal => ' ',
        };
        print!("{}{}", sign, change);
    }
}
