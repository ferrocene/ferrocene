// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

mod signatures;
mod util;

mod compression;
mod generator;
mod tarballer;

use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
pub struct CommandLine {
    #[clap(subcommand)]
    command: Subcommand,
}

impl CommandLine {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Subcommand::Combine => anyhow::bail!("the `combine` subcommand is not supported"),
            Subcommand::Generate(generator) => {
                generator.run().context("failed to generate installer")
            }
            Subcommand::Script => anyhow::bail!("the `scripter` subcommand is not supported"),
            Subcommand::Tarball(tarballer) => {
                tarballer.run().context("failed to generate tarballs")
            }
        }
    }
}

#[derive(clap::Subcommand)]
enum Subcommand {
    Generate(generator::Generator),
    Combine,
    Script,
    Tarball(tarballer::Tarballer),
}
