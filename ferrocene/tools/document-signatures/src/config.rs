// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use anyhow::Error;
use std::collections::HashMap;
use std::path::Path;

static IDPS: &[IdP] = &[IdP {
    display_name: "Microsoft",
    url: "https://login.microsoftonline.com",
    email_domains: &["ferrous-systems.com"],
}];

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) roles: HashMap<String, Role>,
}

impl Config {
    pub(crate) fn load(source_dir: &Path) -> Result<Self, Error> {
        let path = source_dir.join("signature").join("config.toml");
        let content = match std::fs::read(&path) {
            Ok(content) => content,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                anyhow::bail!("missing configuration file: {}", path.display());
            }
            Err(err) => return Err(err.into()),
        };
        Ok(toml::from_slice(&content)?)
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Role {
    pub(crate) email: String,
}

impl Role {
    pub(crate) fn idp(&self) -> Result<&'static IdP, Error> {
        let Some((_, domain)) = self.email.split_once('@') else {
            anyhow::bail!("invalid email address: {}", self.email);
        };

        for idp in IDPS {
            if idp.email_domains.contains(&domain) {
                return Ok(idp);
            }
        }
        anyhow::bail!("unsupported email domain: {domain}");
    }
}

pub(crate) struct IdP {
    pub(crate) display_name: &'static str,
    pub(crate) url: &'static str,
    pub(crate) email_domains: &'static [&'static str],
}
