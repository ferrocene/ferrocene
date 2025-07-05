mod loc;
mod render;
mod stability;
mod stats;
mod visitor;

use crate::loc::LOC;
use crate::stats::{FunctionKind, StatsCollector};
use crate::visitor::Visitor;
use anyhow::Error;
use rustdoc_types::Crate;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, path, out_dir, source_dir] = args.as_slice() else {
        eprintln!("three arguments required: path to json, output directory, and source directory");
        std::process::exit(1);
    };

    let loc = LOC::new(source_dir);

    let contents = std::fs::read(path)?;
    let root: Crate = serde_json::from_slice(&contents)?;

    let mut collector = StatsCollector::new(loc);
    collector.visit_crate(&root);

    let out_dir = PathBuf::from(out_dir);
    if !out_dir.is_dir() {
        std::fs::create_dir_all(&out_dir)?;
    }

    // TODO: CLI arg to switch between modes
    if false {
        functions_tsv(&collector, &out_dir)?;
        types_tsv(&collector, &out_dir)?;
        traits_tsv(&collector, &out_dir)?;
        items_tsv(&collector, &out_dir)?;
        macros_tsv(&collector, out_dir)?;
    } else {
        // TODO: destructure collector and pass fields to functions
        certified_subset_tsv(&mut collector, &out_dir)?;
    }

    Ok(())
}

fn certified_subset_tsv(collector: &mut StatsCollector, out_dir: &PathBuf) -> Result<(), Error> {
    let mut functions = TSV::new(
        &out_dir.join("certified_subset.tsv"),
        [
            "File",
            "Name",
            "Impl",
            "Kind",
            "Visibility",
            "Safety",
            "doc(hidden)",
            "is_nightly",
            "Safety constraint",
            "Panics section",
            "Examples section",
            "Docs",
        ],
    )?;

    // sort by file and within a file by name
    collector.functions.sort_by_key(|f| f.name.clone());
    collector.functions.sort_by_key(|f| f.file.clone());

    for function in &collector.functions {
        let file = &function.file;
        let name = function
            .name
            .strip_prefix(&format!("{}::", &function.module))
            .unwrap();
        let is_nightly = is_nightly(file, name);

        let safety_constraint = safety_constraint(function);

        let docs = &function.docs;
        let contains_panics = docs.contains("# Panics");
        let contains_examples = ["# Example", "# Examples", "# Basic examples"]
            .iter()
            .any(|a| docs.contains(a));

        // warn about rule violations
        let is_trait_method_implementation = matches!(function.kind, FunctionKind::TraitMethod);
        let is_private = !function.public;
        if !(is_private || function.is_doc_hidden || is_trait_method_implementation || is_nightly) {
            if !contains_examples {
                eprintln!("{file}: {name} has no examples")
            }
            if safety_constraint == "missing" {
                eprintln!("{file}: {name} is missing a safety comment")
            }
        }

        functions.add([
            file,
            name,
            function.impl_.as_deref().unwrap_or(""),
            &function.kind.to_string(),
            function.public_str(),
            &function.safety,
            &function.is_doc_hidden.to_string(),
            &is_nightly.to_string(),
            safety_constraint,
            &contains_panics.to_string(),
            &contains_examples.to_string(),
            docs,
        ])?;
    }

    Ok(())
}

// FIXME: detect nightly from attributes, don't hardcode
fn is_nightly(file: &String, name: &str) -> bool {
    let nightly_items = [
        ("library/core/src/intrinsics/mod.rs", [].as_slice()),
        ("library/core/src/ops/function.rs", &[]),
        (
            "library/core/src/ops/range.rs",
            &["Bound::as_mut", "OneSidedRange::bound"],
        ),
        ("library/core/src/panicking.rs", &[]),
        ("library/core/src/ptr/alignment.rs", &[]),
        ("library/core/src/ptr/metadata.rs", &[]),
    ];
    for (nightly_file, items) in nightly_items {
        if nightly_file == file {
            if items.is_empty() {
                return true; // ignore all items in the modules
            } else if items.contains(&name) {
                return true; // ignore specific items only
            }
        }
    }
    false
}

fn safety_constraint(function: &stats::Function) -> &'static str {
    let contains_safety = function.docs.contains("# Safety");
    let is_unsafe = function.safety == "unsafe";
    match (is_unsafe, contains_safety) {
        (true, true) => "documented",
        (true, false) => "missing",
        (false, _) => "not applicable",
    }
}

fn functions_tsv(collector: &StatsCollector, out_dir: &PathBuf) -> Result<(), Error> {
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
            "Lines of documentation",
            "File",
            "Impl",
            "Safety",
            "Docs",
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
            &function.lines_of_docs.to_string(),
            &function.file,
            function.impl_.as_deref().unwrap_or(""),
            &function.safety,
            &function.docs,
        ])?;
    }

    Ok(())
}

fn types_tsv(collector: &StatsCollector, out_dir: &PathBuf) -> Result<(), Error> {
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
            .get(&type_.id)
            .cloned()
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

    Ok(())
}

fn traits_tsv(collector: &StatsCollector, out_dir: &PathBuf) -> Result<(), Error> {
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
            .get(&trait_.id)
            .cloned()
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

    Ok(())
}

fn items_tsv(collector: &StatsCollector, out_dir: &PathBuf) -> Result<(), Error> {
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

    Ok(())
}

fn macros_tsv(collector: &StatsCollector, out_dir: PathBuf) -> Result<(), Error> {
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

/// Tab-separated values
#[allow(clippy::upper_case_acronyms)]
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
