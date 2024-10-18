// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers
// SPDX-FileCopyrightText: The Rust Project Developers (see https://thanks.rust-lang.org)

use std::fs::{read_link, symlink_metadata};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result, bail};
use tar::{Builder, Header};
use walkdir::WalkDir;

use crate::compression::{CombinedEncoder, CompressionFormats, CompressionProfile};
use crate::util::{open_file, path_to_str};

#[derive(Debug)]
#[derive(clap::Args)]
pub struct Tarballer {
    /// The input folder to be compressed.
    #[clap(value_name = "NAME")]
    #[clap(long, default_value = "package")]
    pub input: String,
    /// The prefix of the tarballs.
    #[clap(value_name = "PATH")]
    #[clap(long, default_value = "./dist")]
    pub output: String,
    /// The folder in which the input is to be found.
    #[clap(value_name = "DIR")]
    #[clap(long, default_value = "./workdir")]
    pub work_dir: String,
    /// The profile used to compress the tarball.
    #[clap(value_name = "FORMAT", default_value_t)]
    #[clap(long)]
    pub compression_profile: CompressionProfile,
    /// The formats used to compress the tarball.
    #[clap(value_name = "FORMAT", default_value_t)]
    #[clap(long)]
    pub compression_formats: CompressionFormats,
    /// Modification time that will be set for all files added to the archive.
    /// The default is the date of the first Rust commit from 2006.
    /// This serves for better reproducibility of the archives.
    #[arg(long, value_name = "FILE_MTIME", default_value_t = 1153704088)]
    pub override_file_mtime: u64,
}

impl Tarballer {
    /// Generates the actual tarballs
    pub fn run(self) -> Result<()> {
        if let CompressionProfile::NoOp = self.compression_profile {
            return Ok(());
        }

        let Self {
            input,
            output,
            work_dir,
            compression_profile,
            compression_formats,
            override_file_mtime,
        } = self;
        let tarball_name = output + ".tar";
        let encoder = CombinedEncoder::new(
            compression_formats
                .iter()
                .map(|f| f.encode(&tarball_name, compression_profile))
                .collect::<Result<Vec<_>>>()?,
        );

        let (dirs, mut files) =
            get_recursive_paths(&work_dir, &input).context("failed to collect file paths")?;
        // Sort files by their suffix, to group files with the same name from
        // different locations (likely identical) and files with the same
        // extension (likely containing similar data).
        files.sort_by(|a, b| a.bytes().rev().cmp(b.bytes().rev()));

        // Write the tar into both encoded files. We write all directories
        // first, so files may be directly created. (See rust-lang/rustup.rs#1092.)
        let buf = BufWriter::with_capacity(1024 * 1024, encoder);
        let mut builder = Builder::new(buf);

        let pool = rayon::ThreadPoolBuilder::new().num_threads(2).build().unwrap();
        let base = &Path::new(&work_dir).join(&input);
        pool.install(move || {
            for path in dirs {
                let src = base.join(&path);
                builder
                    .append_dir(&path, &src)
                    .with_context(|| format!("failed to tar dir '{}'", src.display()))?;
            }
            for path in files {
                let src = base.join(&path);
                append_path(&mut builder, &src, &path, override_file_mtime)
                    .with_context(|| format!("failed to tar file '{}'", src.display()))?;
            }
            builder
                .into_inner()
                .context("failed to finish writing .tar stream")?
                .into_inner()
                .ok()
                .unwrap()
                .finish()?;

            Ok(())
        })
    }
}

fn append_path<W: Write>(
    builder: &mut Builder<W>,
    src: &Path,
    path: &String,
    override_file_mtime: u64,
) -> Result<()> {
    let stat = symlink_metadata(src)?;
    let mut header = Header::new_gnu();
    header.set_metadata(&stat);
    header.set_mtime(override_file_mtime);

    if stat.file_type().is_symlink() {
        let link = read_link(src)?;
        builder.append_link(&mut header, path, &link)?;
    } else {
        if cfg!(windows) {
            // Windows doesn't really have a mode, so `tar` never marks files executable.
            // Use an extension whitelist to update files that usually should be so.
            const EXECUTABLES: [&'static str; 4] = ["exe", "dll", "py", "sh"];
            if let Some(ext) = src.extension().and_then(|s| s.to_str()) {
                if EXECUTABLES.contains(&ext) {
                    let mode = header.mode()?;
                    header.set_mode(mode | 0o111);
                }
            }
        }
        let file = open_file(src)?;
        builder.append_data(&mut header, path, &file)?;
    }
    Ok(())
}

/// Returns all `(directories, files)` under the source path.
fn get_recursive_paths<P, Q>(root: P, name: Q) -> Result<(Vec<String>, Vec<String>)>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let root = root.as_ref();
    let name = name.as_ref();

    if !name.is_relative() && !name.starts_with(root) {
        bail!("input '{}' is not in work dir '{}'", name.display(), root.display());
    }

    let root = root.join(name);

    let mut dirs = vec![];
    let mut files = vec![];
    for entry in WalkDir::new(&root) {
        let entry = entry?;
        let path = entry.path().strip_prefix(&root)?;
        let path = path_to_str(&path)?;
        if path.is_empty() {
            // WalkDir will report the root path as well, but we strip it from the entries
            // so skip that empty path here
            continue;
        }

        if entry.file_type().is_dir() { &mut dirs } else { &mut files }.push(path.to_owned());
    }
    Ok((dirs, files))
}
