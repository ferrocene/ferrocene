// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    <generate_tarball::CommandLine as clap::Parser>::parse().run().await
}
