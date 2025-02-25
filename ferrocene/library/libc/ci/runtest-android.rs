use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args = env::args_os()
        .skip(1)
        .filter(|arg| arg != "--quiet" && arg != "--skip" && arg != "check_style")
        .collect::<Vec<_>>();
    assert_eq!(args.len(), 1);
    let test = PathBuf::from(&args[0]);
    let dst = Path::new("/data/local/tmp").join(test.file_name().unwrap());

    let status = Command::new("adb")
        .arg("wait-for-device")
        .status()
        .expect("failed to run: adb wait-for-device");
    assert!(status.success());

    let status = Command::new("adb")
        .arg("push")
        .arg(&test)
        .arg(&dst)
        .status()
        .expect("failed to run: adb push");
    assert!(status.success());

    let output = Command::new("adb")
        .arg("shell")
        .arg("RUST_BACKTRACE=1")
        .arg(&dst)
        .output()
        .expect("failed to run: adb shell");
    assert!(status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!(
        "status: {}\nstdout ---\n{}\nstderr ---\n{}",
        output.status, stdout, stderr
    );

    if !stderr.lines().any(|l| {
        (l.starts_with("PASSED ") && l.contains(" tests")) || l.starts_with("test result: ok")
    }) && !stdout.lines().any(|l| {
        (l.starts_with("PASSED ") && l.contains(" tests")) || l.starts_with("test result: ok")
    }) {
        panic!("failed to find successful test run");
    };
}
