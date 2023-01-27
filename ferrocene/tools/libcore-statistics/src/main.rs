mod loc;
mod render;
mod stability;
mod stats;
mod visitor;

use crate::loc::LOC;
use crate::stats::StatsCollector;
use crate::visitor::Visitor;
use anyhow::Error;
use rustdoc_types::Crate;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, path, out_dir, source_dir] = args.as_slice() else {
        eprintln!("two arguments required: path to json, output directory, and source directory");
        std::process::exit(1);
    };

    let loc = LOC::new(Path::new(source_dir));

    let contents = std::fs::read(&path)?;
    let root: Crate = serde_json::from_slice(&contents)?;

    let mut collector = StatsCollector::new(loc);
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
            "Trait name",
            "Lines of code",
            "File",
            "Impl",
        ],
    )?;
    for function in &collector.functions {
        functions.add([
            &function.module,
            &function.name,
            &function.kind.to_string(),
            function.public_str(),
            function.stable_str(),
            function.feature_str(),
            match &function.trait_id {
                Some(id) => &collector.traits[id].name,
                None => "",
            },
            &function.lines_of_code.to_string(),
            &function.file,
            function.impl_.as_deref().unwrap_or(""),
        ])?;
    }

    let mut types = TSV::new(
        &out_dir.join("types.tsv"),
        [
            "Module",
            "Path",
            "Kind",
            "Visibility",
            "Stability",
            "Feature Gate",
            "Traits implemented",
            "Auto traits implemented",
            "Blanket traits implemented",
            "Methods",
            "Trait methods",
        ],
    )?;
    for type_ in &collector.types {
        let counters = &collector
            .type_counters
            .remove(&type_.id)
            .unwrap_or_default();

        types.add([
            &type_.module,
            &type_.name,
            &type_.kind.to_string(),
            type_.public_str(),
            type_.stable_str(),
            type_.feature_str(),
            &counters.trait_impls.to_string(),
            &counters.auto_impls.to_string(),
            &counters.blanket_impls.to_string(),
            &counters.methods.to_string(),
            &counters.trait_methods.to_string(),
        ])?;
    }

    let mut traits = TSV::new(
        &out_dir.join("traits.tsv"),
        [
            "Module",
            "Path",
            "Visibility",
            "Stability",
            "Feature Gate",
            "Required methods",
            "Default methods",
            "Implementations",
        ],
    )?;
    for trait_ in collector.traits.values() {
        let counters = &collector
            .trait_counters
            .remove(&trait_.id)
            .unwrap_or_default();

        traits.add([
            &trait_.module,
            &trait_.name,
            trait_.public_str(),
            trait_.stable_str(),
            trait_.feature_str(),
            &counters.required_methods.to_string(),
            &counters.default_methods.to_string(),
            &trait_.implementations.to_string(),
        ])?;
    }

    let mut items = TSV::new(
        &out_dir.join("items.tsv"),
        [
            "Module",
            "Path",
            "Kind",
            "Visibility",
            "Stability",
            "Feature Gate",
            "Type",
            "Value",
        ],
    )?;
    for item in &collector.items {
        items.add([
            &item.module,
            &item.name,
            &item.kind.to_string(),
            item.public_str(),
            item.stable_str(),
            item.feature_str(),
            &item.type_,
            &item.value,
        ])?;
    }

    let mut macros = TSV::new(
        &out_dir.join("macros.tsv"),
        [
            "Module",
            "Path",
            "Kind",
            "Visibility",
            "Stability",
            "Feature Gate",
        ],
    )?;
    for macro_ in &collector.macros {
        macros.add([
            &macro_.module,
            &macro_.name,
            &macro_.kind.to_string(),
            macro_.public_str(),
            macro_.stable_str(),
            macro_.feature_str(),
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
