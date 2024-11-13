// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::process::Command;

use needy::requirements;

#[requirements(REQ_9A6740M)]
#[test]
fn invoke_with_args() {
    let exe = env!("CARGO_BIN_EXE_ferrocene-self-test");
    for arg in ["some-arg", "two args", "--flag", "--option value"] {
        let output = Command::new(exe).arg(arg).output().unwrap();
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains("FST_012"));
    }
}
