// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod config;
mod cosign_bundle;
mod pinned;
mod sign;
mod signature_files;
mod verify;

use anyhow::{Context, Error};
use std::path::{Path, PathBuf};
use std::str::FromStr;

// \u{2d} replaces "-" to avoid REUSE mistakenly detecting these lines as a license.
const TOML_HEADER_COMMENTS: &str = "\
    # SPDX\u{2d}License\u{2d}Identifier: MIT OR Apache-2.0\n\
    # SPDX\u{2d}FileCopyrightText: The Ferrocene Developers\n\
    \n\
";

fn main() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("required arguments: sign|verify <source-dir> <output-dir>");
        std::process::exit(1);
    }
    let mode = &args[0];
    let source_dir = Path::new(&args[1]);
    let output_dir = Path::new(&args[2]);

    let options = CliOptions::load()?;

    match &mode[..] {
        "sign" => sign::sign(&source_dir, &output_dir, &options)?,
        "verify" => verify::verify(&source_dir, &output_dir, &options)?,
        other => anyhow::bail!("unknown mode: {other}"),
    }

    Ok(())
}

struct CliOptions {
    cosign_binary: PathBuf,
    s3_bucket: String,
    s3_cache_dir: PathBuf,
}

impl CliOptions {
    fn load() -> Result<Self, Error> {
        Ok(Self {
            cosign_binary: env("COSIGN_BINARY")?,
            s3_bucket: env("S3_BUCKET")?,
            s3_cache_dir: env("S3_CACHE_DIR")?,
        })
    }
}

fn env<T>(var: &str) -> Result<T, Error>
where
    T: FromStr,
    T::Err: Send + Sync + std::error::Error + 'static,
{
    let var = format!("DOCUMENT_SIGNATURES_{var}");
    match std::env::var(&var) {
        Ok(v) => v.parse().map_err(Error::from).with_context(|| format!("failed to parse {var}")),
        Err(e) => Err(Error::from(e).context(format!("failed to read {var}"))),
    }
}
