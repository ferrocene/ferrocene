// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use anyhow::Error;
use serde::Deserialize;
use std::path::Path;

pub(crate) struct Documentaton {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) ids: TraceabilityIds,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TraceabilityIds {
    pub(crate) documents: Vec<Document>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Document {
    pub(crate) title: String,
    pub(crate) link: String,
    #[serde(default)]
    pub(crate) sections: Vec<Section>,
    #[serde(default)]
    pub(crate) options: Vec<CliOption>,
    pub(crate) informational: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Section {
    pub(crate) id: String,
    pub(crate) number: String,
    pub(crate) title: String,
    pub(crate) link: String,
    pub(crate) paragraphs: Vec<Paragraph>,
    pub(crate) informational: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Paragraph {
    pub(crate) id: String,
    pub(crate) number: String,
    pub(crate) link: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CliOption {
    pub(crate) id: String,
    pub(crate) program: String,
    pub(crate) option: String,
    pub(crate) link: String,
}

pub(crate) fn load(name: &str, path: &Path, url: &str) -> Result<Documentaton, Error> {
    Ok(Documentaton {
        name: name.into(),
        url: url.into(),
        ids: serde_json::from_slice(&std::fs::read(path)?)?,
    })
}
