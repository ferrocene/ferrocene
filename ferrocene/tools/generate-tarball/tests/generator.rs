// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::ffi::OsStr;

use clap::Parser;
use xz2::read::XzDecoder;

#[test]
fn works() -> Result<(), anyhow::Error> {
    let temp_dir = tempfile::tempdir()?;
    let temp_dir = temp_dir.path().to_str().unwrap();

    let test_data = format!("{}/test_data", env!("CARGO_MANIFEST_DIR"));
    let data_dir = format!("{test_data}/foo-package/x86_64-unknown-linux-gnu",);

    let args = [
        "this",
        "generate",
        "--image-dir",
        &format!("{data_dir}/image"),
        "--component-name=ferrocene-gen-tarball-test",
        "--rel-manifest-dir=rustlib",
        "--legacy-manifest-dirs=rustlib,cargo",
        "--product-name=Rust",
        "--success-message=ferrocene-self-test installed.",
        "--package-name=ferrocene-gen-tarball-test-1.72.0-dev-x86_64-unknown-linux-gnu",
        "--non-installed-overlay",
        &format!("{data_dir}/overlay"),
        "--output-dir",
        &format!("{temp_dir}/dist"),
        "--work-dir",
        &format!("{temp_dir}/work"),
        "--compression-profile",
        "fast",
        "--ferrocene-component",
        "foo",
    ];
    generate_tarball::CommandLine::parse_from(args.into_iter().map(OsStr::new)).run()?;

    let file = std::fs::File::open(&format!(
        "{temp_dir}/dist/ferrocene-gen-tarball-test-1.72.0-dev-x86_64-unknown-linux-gnu.tar.xz"
    ))?;

    let mut res = tar::Archive::new(XzDecoder::new(file))
        .entries()?
        .map(|it| anyhow::Ok(it?.path()?.to_string_lossy().to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    res.sort();

    let expected = vec!["bar".to_string(), "bar/bar".to_string(), "foo".to_string()];
    assert_eq!(res, expected);

    Ok(())
}
