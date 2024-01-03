use std::process::Command;
use util::env_path;

mod util;

fn main() {
    let data_dir = env_path("DATA_DIR");
    let out_dir = env_path("OUT_DIR");

    let src_path = env_path("SRC_PATH");
    let binary_path = env_path("BIN_PATH");

    assert!(data_dir.exists());
    assert!(!out_dir.exists());
    assert!(src_path.exists());
    assert!(binary_path.exists());

    Command::new("grcov")
        .arg(data_dir)
        .arg("-s")
        .arg(src_path)
        .arg("--ignore")
        .arg("tests/*")
        .arg("--ignore")
        .arg("benches/*")
        .arg("--binary-path")
        .arg(binary_path)
        .arg("-t")
        .arg("html")
        .arg("--branch")
        .arg("--ignore-not-existing")
        .arg("-o")
        .arg(out_dir)
        .output()
        .expect("Failed to execute grcov");
}
