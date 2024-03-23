// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

use anyhow::{Context, Error};
use rayon::prelude::*;
use std::{fmt, io::Write, path::Path, str::FromStr};
use xz2::write::XzEncoder;

#[derive(Default, Debug, Copy, Clone)]
pub enum CompressionProfile {
    Fast,
    #[default]
    Balanced,
    Best,
}

impl FromStr for CompressionProfile {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        Ok(match input {
            "fast" => Self::Fast,
            "balanced" => Self::Balanced,
            "best" => Self::Best,
            other => anyhow::bail!("invalid compression profile: {other}"),
        })
    }
}

impl fmt::Display for CompressionProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionProfile::Fast => f.write_str("fast"),
            CompressionProfile::Balanced => f.write_str("balanced"),
            CompressionProfile::Best => f.write_str("best"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CompressionFormat {
    Xz,
}

impl CompressionFormat {
    pub(crate) fn extension(&self) -> &'static str {
        match self {
            CompressionFormat::Xz => "xz",
        }
    }

    pub(crate) fn encode(
        &self,
        path: impl AsRef<Path>,
        profile: CompressionProfile,
    ) -> Result<Box<dyn Encoder>, Error> {
        let mut os = path.as_ref().as_os_str().to_os_string();
        os.push(format!(".{}", self.extension()));
        let path = Path::new(&os);

        if path.exists() {
            crate::util::remove_file(path)?;
        }
        let file = crate::util::create_new_file(path)?;

        Ok(match self {
            CompressionFormat::Xz => {
                let encoder = match profile {
                    CompressionProfile::Fast => {
                        xz2::stream::MtStreamBuilder::new().threads(6).preset(1).encoder().unwrap()
                    }
                    CompressionProfile::Balanced => {
                        xz2::stream::MtStreamBuilder::new().threads(6).preset(6).encoder().unwrap()
                    }
                    CompressionProfile::Best => {
                        // Note that this isn't actually the best compression settings for the
                        // produced artifacts, the production artifacts on static.rust-lang.org are
                        // produced by rust-lang/promote-release which hosts recompression logic
                        // and is tuned for optimal compression.
                        xz2::stream::MtStreamBuilder::new().threads(6).preset(9).encoder().unwrap()
                    }
                };

                let compressor = XzEncoder::new_stream(std::io::BufWriter::new(file), encoder);
                Box::new(compressor)
            }
        })
    }
}

/// This struct wraps Vec<CompressionFormat> in order to parse the value from the command line.
#[derive(Debug, Clone)]
pub struct CompressionFormats(Vec<CompressionFormat>);

impl TryFrom<&'_ str> for CompressionFormats {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parsed = Vec::new();
        for format in value.split(',') {
            match format.trim() {
                "xz" => parsed.push(CompressionFormat::Xz),
                other => anyhow::bail!("unknown compression format: {}", other),
            }
        }
        Ok(CompressionFormats(parsed))
    }
}

impl FromStr for CompressionFormats {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl fmt::Display for CompressionFormats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, format) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ",")?;
            }
            fmt::Display::fmt(
                match format {
                    CompressionFormat::Xz => "xz",
                },
                f,
            )?;
        }
        Ok(())
    }
}

impl Default for CompressionFormats {
    fn default() -> Self {
        Self(vec![CompressionFormat::Xz])
    }
}

impl CompressionFormats {
    pub(crate) fn iter(&self) -> impl Iterator<Item = CompressionFormat> + '_ {
        self.0.iter().map(|i| *i)
    }
}

pub(crate) trait Encoder: Send + Write {
    fn finish(self: Box<Self>) -> Result<(), Error>;
}

impl<W: Send + Write> Encoder for XzEncoder<W> {
    fn finish(self: Box<Self>) -> Result<(), Error> {
        XzEncoder::finish(*self).context("failed to finish .xz file")?;
        Ok(())
    }
}

pub(crate) struct CombinedEncoder {
    encoders: Vec<Box<dyn Encoder>>,
}

impl CombinedEncoder {
    pub(crate) fn new(encoders: Vec<Box<dyn Encoder>>) -> Box<dyn Encoder> {
        Box::new(Self { encoders })
    }
}

impl Write for CombinedEncoder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_all(buf)?;
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.encoders
            .par_iter_mut()
            .map(|w| w.write_all(buf))
            .collect::<std::io::Result<Vec<()>>>()?;
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.encoders.par_iter_mut().map(|w| w.flush()).collect::<std::io::Result<Vec<()>>>()?;
        Ok(())
    }
}

impl Encoder for CombinedEncoder {
    fn finish(self: Box<Self>) -> Result<(), Error> {
        self.encoders.into_par_iter().map(|e| e.finish()).collect::<Result<Vec<()>, Error>>()?;
        Ok(())
    }
}
