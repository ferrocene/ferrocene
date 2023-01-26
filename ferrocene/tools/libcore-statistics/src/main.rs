mod render;
mod stats;
mod visitor;

use crate::stats::FunctionsCollector;
use crate::visitor::Visitor;
use anyhow::Error;
use rustdoc_types::Crate;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, path, out_dir] = args.as_slice() else {
        eprintln!("two arguments required: path to json, and output directory");
        std::process::exit(1);
    };

    let contents = std::fs::read(&path)?;
    let root: Crate = serde_json::from_slice(&contents)?;

    let mut collector = FunctionsCollector::new();
    collector.visit_crate(&root);

    let out_dir = PathBuf::from(out_dir);
    if !out_dir.is_dir() {
        std::fs::create_dir_all(&out_dir)?;
    }

    let mut functions = TSV::new(
        &out_dir.join("functions.tsv"),
        [
            "Module",
            "Path",
            "Kind",
            "Visibility",
            "Stability",
            "Feature gate",
            "Impl",
        ],
    )?;
    for item in &collector.found {
        functions.add([
            &item.module,
            &item.name,
            &item.kind.to_string(),
            if item.public { "public" } else { "private" },
            "TODO",
            "TODO",
            item.impl_.as_deref().unwrap_or(""),
        ])?;
    }

    Ok(())
}

struct TSV<const N: usize> {
    writer: BufWriter<File>,
}

impl<const N: usize> TSV<N> {
    fn new(path: &Path, header: [&str; N]) -> Result<Self, Error> {
        let mut tsv = Self {
            writer: BufWriter::new(File::create(path)?),
        };
        tsv.add(header)?;

        Ok(tsv)
    }

    fn add(&mut self, line: [&str; N]) -> Result<(), Error> {
        for (i, heading) in line.iter().enumerate() {
            if i != 0 {
                self.writer.write_all(b"\t")?;
            }
            self.writer.write_all(heading.as_bytes())?;
        }
        self.writer.write_all(b"\n")?;
        Ok(())
    }
}
