// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

mod annotations;
mod documentations;
mod matrix;
mod report;
mod test_outcomes;
mod utils;

use crate::annotations::Annotations;
use crate::matrix::TraceabilityMatrix;
use crate::report::Urls;
use crate::test_outcomes::TestOutcomes;
use crate::utils::capitalize;
use anyhow::Error;
use std::path::PathBuf;

fn main() -> Result<(), Error> {
    let annotations_path = env_path("ANNOTATIONS");
    let html_out = env_path("HTML_OUT");
    let src_base = env_path("SRC_BASE");
    let test_outcomes_dir = maybe_env_path("TEST_OUTCOMES_DIR");

    let urls = Urls { src: env_str("SRC_URL") };

    let documentations = [
        documentations::load("FLS", &env_path("FLS_IDS"), &env_str("FLS_URL"))?,
        documentations::load("UM", &env_path("UM_IDS"), &env_str("UM_URL"))?,
    ];

    let test_outcomes =
        if let Some(dir) = test_outcomes_dir { Some(TestOutcomes::load(&dir)?) } else { None };

    let mut annotations = Annotations::new();
    annotations.load_directory(&annotations_path, &src_base, test_outcomes.as_ref())?;

    let matrix = matrix::prepare(&documentations, &annotations)?;
    cli_summary(&matrix);

    let report = report::generate(&annotations, &matrix, urls)?;
    std::fs::write(&html_out, report.as_bytes())?;

    // The file:// link is hopefully clickable in terminals.
    eprintln!("Full report: file://{}", std::fs::canonicalize(&html_out)?.display());
    eprintln!();

    if !matrix.unknown_annotations.is_empty() {
        anyhow::bail!("some tests have unknown annotations");
    }

    Ok(())
}

fn cli_summary(matrix: &TraceabilityMatrix) {
    eprintln!("=====================================");
    eprintln!("==   Traceability matrix summary   ==");
    eprintln!("=====================================");
    eprintln!();
    for analysis in matrix.analyses_by_kind() {
        let linked_count = analysis.linked.len();
        let unlinked_count = analysis.unlinked.len();
        let total_items = linked_count + unlinked_count;

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
}

fn env_str(var: &str) -> String {
    let var = format!("TRACEABILITY_MATRIX_{var}");
    if let Ok(content) = std::env::var(&var) {
        content
    } else {
        panic!("missing enviroment variable {var}");
    }
}

fn env_path(var: &str) -> PathBuf {
    let var = format!("TRACEABILITY_MATRIX_{var}");
    if let Some(content) = std::env::var_os(&var) {
        content.into()
    } else {
        panic!("missing enviroment variable {var}");
    }
}

fn maybe_env_path(var: &str) -> Option<PathBuf> {
    let var = format!("TRACEABILITY_MATRIX_{var}");
    if let Some(content) = std::env::var_os(&var) { Some(content.into()) } else { None }
}
