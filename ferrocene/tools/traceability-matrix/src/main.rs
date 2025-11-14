// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod annotations;
mod documentations;
mod matrix;
mod report;
mod test_outcomes;
mod utils;

use std::path::PathBuf;

use crate::annotations::Annotations;
use crate::matrix::TraceabilityMatrix;
use crate::report::Urls;
use crate::test_outcomes::TestOutcomes;
use crate::utils::capitalize;

fn main() -> anyhow::Result<()> {
    let annotations_path = env_path("ANNOTATIONS");
    let html_out = env_path("HTML_OUT");
    let src_base = env_path("SRC_BASE");
    let test_outcomes_dir = maybe_env_path("TEST_OUTCOMES_DIR");

    let urls = Urls { src: env_str("SRC_URL") };

    let documentations = [
        documentations::load("FLS", &env_path("FLS_IDS"), &env_str("FLS_URL"))?,
        documentations::load("UM", &env_path("UM_IDS"), &env_str("UM_URL"))?,
    ];

    let test_outcomes = test_outcomes_dir.map(|dir| TestOutcomes::load(&dir)).transpose()?;

    let mut annotations = Annotations::new();
    annotations.load_directory(&annotations_path, &src_base, test_outcomes.as_ref())?;

    let matrix = matrix::prepare(&documentations, &annotations)?;

    let incomplete = cli_summary(&matrix);
    let report = report::generate(&annotations, &matrix, urls)?;
    std::fs::write(&html_out, report.as_bytes())?;

    // The file:// link is hopefully clickable in terminals.
    eprintln!("Full report: file://{}", std::fs::canonicalize(&html_out)?.display());
    eprintln!();

    if !matrix.unknown_annotations.is_empty() {
        anyhow::bail!("some tests have unknown annotations");
    }

    if incomplete {
        anyhow::bail!("some analyses are incomplete");
    }

    Ok(())
}

fn cli_summary(matrix: &TraceabilityMatrix) -> bool {
    let mut incomplete = false;

    eprintln!("=====================================");
    eprintln!("==   Traceability matrix summary   ==");
    eprintln!("=====================================");
    eprintln!();
    for analysis in matrix.analyses_by_kind() {
        let linked_count = analysis.linked.len();
        let partially_linked_count = analysis.partially_linked.len();
        let unlinked_count = analysis.unlinked.len();
        let total_items = linked_count + unlinked_count + partially_linked_count;
        if linked_count != total_items {
            incomplete = true;
        }
        let percent = linked_count as f32 * 100.0 / total_items as f32;
        eprintln!(
            "{} linked with a test: {linked_count} ({percent:.2}%)",
            capitalize(analysis.kind.plural)
        );
        eprintln!("Total {} in the spec: {total_items}", analysis.kind.plural);
        eprintln!();
    }
    if !matrix.unknown_annotations.is_empty() {
        eprintln!("Tests with unknown annotations:");
        for test in &matrix.unknown_annotations {
            eprintln!("- {}: {}", test.annotation, test.file);
        }
        eprintln!();
    }

    incomplete
}

/// Fetch a [`String`] environment varible.
fn env_str(var: &str) -> String {
    let var = env_var_name(var);
    if let Ok(content) = std::env::var(&var) {
        content
    } else {
        panic!("missing enviroment variable {var}");
    }
}

/// Fetch a [`PathBuf`] environment variable.
fn env_path(var: &str) -> PathBuf {
    let var = env_var_name(var);
    if let Some(content) = std::env::var_os(&var) {
        content.into()
    } else {
        panic!("missing enviroment variable {var}");
    }
}

/// Fetch an optional [`PathBuf`] environment variable.
fn maybe_env_path(var: &str) -> Option<PathBuf> {
    let var = env_var_name(var);
    std::env::var_os(var).map(|content| content.into())
}

/// Prefix the environment variable name with the application name.
fn env_var_name(var: &str) -> String {
    format!("TRACEABILITY_MATRIX_{var}")
}
