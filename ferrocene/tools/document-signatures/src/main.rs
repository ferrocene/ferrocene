// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod config;
mod cosign_bundle;
mod pinned;
mod sign;
mod signature_files;
mod verify;

use std::env::VarError;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Error};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    Sign {
        source_dir: PathBuf,
        output_dir: PathBuf,
        #[arg(long)]
        force: bool,
    },
    Verify {
        source_dir: PathBuf,
        output_dir: PathBuf,
    },
}

// \u{2d} replaces "-" to avoid REUSE mistakenly detecting these lines as a license.
const TOML_HEADER_COMMENTS: &str = "\
    # SPDX\u{2d}License\u{2d}Identifier: MIT OR Apache-2.0\n\
    # SPDX\u{2d}FileCopyrightText: The Ferrocene Developers\n\
    \n\
";

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let env = Env::load()?;

    match cli.cmd {
        CliCommand::Sign { source_dir, output_dir, force } => {
            sign::sign(&source_dir, &output_dir, force, &env)?;
        }
        CliCommand::Verify { source_dir, output_dir } => {
            verify::verify(&source_dir, &output_dir, &env)?;
        }
    }

    Ok(())
}

struct Env {
    cosign_binary: PathBuf,
    s3_bucket: Option<String>,
    s3_cache_dir: PathBuf,
}

impl Env {
    fn load() -> Result<Self, Error> {
        Ok(Self {
            cosign_binary: env("COSIGN_BINARY")?,
            s3_bucket: maybe_env("S3_BUCKET")?,
            s3_cache_dir: env("S3_CACHE_DIR")?,
        })
    }
}

fn maybe_env<T>(var: &str) -> Result<Option<T>, Error>
where
    T: FromStr,
    T::Err: Send + Sync + std::error::Error + 'static,
{
    let var = env_name(var);
    match std::env::var(&var) {
        Ok(v) => v
            .parse()
            .map(Some)
            .map_err(Error::from)
            .with_context(|| format!("failed to parse {var}")),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => Err(Error::from(e).context(format!("failed to read {var}"))),
    }
}

fn env<T>(var: &str) -> Result<T, Error>
where
    T: FromStr,
    T::Err: Send + Sync + std::error::Error + 'static,
{
    maybe_env(var)?.ok_or_else(|| anyhow::anyhow!("missing env var {}", env_name(var)))
}

fn env_name(var: &str) -> String {
    format!("DOCUMENT_SIGNATURES_{var}")
}
