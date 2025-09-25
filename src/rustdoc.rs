use anyhow::{Context as _, Result};
use llvm_profparser::CoverageReport;
use rustdoc_types::{Crate, Id, Impl, ItemEnum, Module, Span, Trait, Type};
use std::collections::HashMap;
use std::fs::{self, File};

use crate::{CoverageStatus, FunctionCoverage, FunctionType, LineCoverage, ShowCommand};

fn type_name(t: &Type) -> &str {
    match t {
        Type::ResolvedPath(p) => &p.path,
        _ => todo!("{t:?}"),
    }
}

fn is_provided_default(item: Id, provided_defaults: &[String], rustdoc: &Crate) -> bool {
    let name = rustdoc.index[&item].name.as_ref().expect("no name for impl function");
    provided_defaults.contains(&name)
}

fn all_functions<'c>(parent_name: &str, items: &[Id], funcs: &mut HashMap<Id, FunctionType>, rustdoc: &'c Crate) {
    for id in items {
        let item = &rustdoc.index[id];
        let item_name = || item.name.as_ref().expect("no name for rustdoc-json module/function?");
        let new_name = |name: &String| {
            if parent_name.is_empty() { name.clone() }
            else { format!("{parent_name}::{}", name) }
        };
        println!("{}", new_name(item.name.as_ref().unwrap_or(&"<unknown>".into())));
        match &item.inner {
            | ItemEnum::Module(Module { items, .. })
            | ItemEnum::Trait(Trait { items, .. })
            => {
                all_functions(&new_name(item_name()), &items, funcs, rustdoc)
            }
            ItemEnum::Impl(Impl { items, trait_: None, for_, .. }) => {
                all_functions(type_name(for_), items, funcs, rustdoc)
            }
            ItemEnum::Impl(Impl { items, trait_: Some(t), for_, provided_trait_methods, .. }) => {
                let name = format!("<{} as {}>", type_name(for_), t.path);
                let (provided, real): (Vec<_>, _) = items.iter().copied()
                                       .partition(|&i| is_provided_default(i, provided_trait_methods, rustdoc));
                println!("provided methods: {provided_trait_methods:?}");
                for id in provided {
                    println!("provided default for {}", rustdoc.index[&id].name.as_ref().unwrap());
                    funcs.insert(id, FunctionType::ProvidedDefault);
                }
                all_functions(&name, &real, funcs, rustdoc)
            }
            ItemEnum::Function(_) => drop(funcs.insert(*id, FunctionType::Canonical(new_name(item_name())))),
            _ => {}
        }
    }
}

fn get_coverage(report: &CoverageReport, span: Span, ferrocene: &std::path::Path, source_name: String) -> Result<FunctionCoverage> {
    let Span { mut filename, begin: (start_line, _), end: (end_line, _), .. } = span;
    if filename.is_relative() {
        filename = ferrocene.join(filename);
    }
    let filename = if filename.is_absolute() {
        fs::canonicalize(&filename).context(format!("failed to canonicalize {filename:?}"))
    } else {
        panic!("--ferrocene-src was not absolute")
    }?;
    let source_lines = start_line..=end_line;
    let no_coverage = FunctionCoverage {
        source_name,
        // symbol_name: "TODO sorry".into(),
        filename: filename.clone(),
        // we didn't get any hits from the tool, so we don't know which lines shouldn't be
        // considered. report them all as considered and missing coverage.
        lines: LineCoverage {
            lines: source_lines.clone()
                .map(|i| (i, CoverageStatus::Untested)).collect(),
        },
    };
    let Some(func_coverage) = report.files.get(&filename) else {
        println!("warning: couldn't find source file {} in coverage report", filename.display());
        return Ok(no_coverage);
    };
    let mut covered = vec![];
    for line in source_lines {
        // one more thing to do: within a function, some lines will always be uncovered (e.g. }
        // closing braces). so we do have to trust the coverage tool to report those accurately.
        let status = match func_coverage.hits_for_line(line) {
            None => CoverageStatus::Ignored,
            Some(0) => CoverageStatus::Untested,
            Some(_) => CoverageStatus::Tested,
        };
        covered.push((line, status));
    }
    Ok(FunctionCoverage { lines: LineCoverage { lines: covered }, ..no_coverage })
}

pub fn coverage(cmd: &ShowCommand, report: &CoverageReport) -> Result<Vec<FunctionCoverage>> {
    let rustdoc: rustdoc_types::Crate = serde_json::from_reader(
        File::open(&cmd.rustdoc_json).context(format!("failed to open rustdoc-json file {}", cmd.rustdoc_json.display()))?
    )?;

    let mut funcs = HashMap::new();
    let ItemEnum::Module(root) = &rustdoc.index[&rustdoc.root].inner
        else { panic!("root wasn't a module?") };
    all_functions("core", &root.items, &mut funcs, &rustdoc);

    let mut coverage = vec![];
    for item in rustdoc.index.values() {
        if matches!(item.inner, ItemEnum::Function(_)) {
            let span = item.span.as_ref().expect("TODO: handle no span for rustdoc-json function");
            let name = item.name.clone().expect("no name for rustdoc-json function?");
            // let qualified_name = match rustdoc.paths.get(&item.id) {
            let qualified_name = match funcs.get(&item.id)
                .cloned()
                .or_else(|| rustdoc.paths.get(&item.id).map(|s| FunctionType::Canonical(s.path.join("::")))) {
                    None => {
                        println!("WARNING: no module path for {} @ {}:{}", name, span.filename.display(), span.begin.0);
                        name
                    }
                    // This is an implementation whose function falls back to the default trait
                    // impl. There is no corresponding source code line and nothing to measure.
                    Some(FunctionType::ProvidedDefault) => continue,
                    Some(FunctionType::Canonical(resolved)) => resolved.clone(),
                    // Some(summary) => format!("{}::{name}", summary.path.join("::")),
                };
            coverage.push(get_coverage(report, span.clone(), &cmd.ferrocene, qualified_name)?);
        }
    }
    Ok(coverage)
}
