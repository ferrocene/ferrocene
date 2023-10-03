// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::ffi::OsStr;
use std::fs::{copy, create_dir, create_dir_all};
use std::path::Path;

use clap::Parser;
use walkdir::WalkDir;

#[test]
fn works() -> Result<(), anyhow::Error> {
    let temp_dir = tempfile::tempdir()?;
    let temp_dir = temp_dir.path().to_str().unwrap();

    let test_data = format!("{}/test_data", env!("CARGO_MANIFEST_DIR"));
    let data_dir = format!("{test_data}/foo-package/x86_64-unknown-linux-gnu",);

    create_dir_all(&format!("{temp_dir}/work/image"))?;
    copy_recursive(
        format!("{data_dir}/image").as_ref(),
        format!("{temp_dir}/work/image").as_ref(),
    )?;

    let args = [
        "this",
        "tarball",
        "--input",
        &format!("{temp_dir}/work/image"),
        "--output",
        &format!("{temp_dir}/dist"),
        "--work-dir",
        &format!("{temp_dir}/work"),
        "--compression-profile",
        "fast",
    ];
    generate_tarball::CommandLine::parse_from(args.into_iter().map(OsStr::new)).run()
}

fn copy_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    for entry in WalkDir::new(src).min_depth(1) {
        let entry = entry?;
        let file_type = entry.file_type();
        let path = entry.path().strip_prefix(src)?;
        let dst = dst.join(path);

        if file_type.is_dir() {
            create_dir(&dst)?;
        } else {
            copy(entry.path(), dst)?;
        }
    }
    Ok(())
}
