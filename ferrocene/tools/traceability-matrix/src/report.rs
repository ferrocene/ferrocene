// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::annotations::{AnnotationSource, Annotations};
use crate::matrix::{ItemKind, LinkTest, Page, TraceabilityMatrix};
use anyhow::Error;
use askama::Template;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Template)]
#[template(path = "report.html")]
struct Report<'a> {
    considers_ignored_tests: bool,
    matrix: &'a TraceabilityMatrix,
    summary: Vec<SummaryRow<'a>>,
    ignored_tests: Vec<PathBuf>,
    urls: Urls,
}

struct SummaryRow<'a> {
    page: Option<&'a Page>,
    kinds: Vec<SummaryItem>,
}

impl SummaryRow<'_> {
    fn color(&self) -> &'static str {
        let linked = self.kinds.iter().map(|k| k.linked).sum::<usize>();
        let total = self.kinds.iter().map(|k| k.total).sum::<usize>();

        if linked == total {
            "green"
        } else if linked > 0 {
            "orange"
        } else {
            "red"
        }
    }
}

#[derive(Clone)]
struct SummaryItem {
    kind: &'static ItemKind,
    linked: usize,
    total: usize,
    percentage: f32,
}

pub(crate) struct Urls {
    pub(crate) src: String,
}

pub(crate) fn generate(
    annotations: &Annotations,
    matrix: &TraceabilityMatrix,
    urls: Urls,
) -> Result<String, Error> {
    let mut ignored_tests = annotations.ignored_tests.iter().cloned().collect::<Vec<_>>();
    ignored_tests.sort();

    Ok(Report {
        matrix,
        summary: build_summary(matrix),
        urls,
        ignored_tests,
        considers_ignored_tests: annotations.considers_ignored_tests,
    }
    .render()?)
}

fn build_summary(matrix: &TraceabilityMatrix) -> Vec<SummaryRow<'_>> {
    let sample_kinds = matrix
        .analyses_by_kind()
        .map(|a| (a.kind, SummaryItem { kind: a.kind, linked: 0, total: 0, percentage: 0.0 }))
        .collect::<HashMap<_, _>>();

    // The table is created in an earlier step because we want to make sure all kinds are present,
    // even if a page doesn't contain that kind of data.
    let mut rows = crate::utils::chain(
        matrix.analyses_by_kind().flat_map(|a| a.linked.iter()).map(|l| &l.page),
        matrix.analyses_by_kind().flat_map(|a| a.unlinked.iter()).map(|u| &u.page),
    )
    .collect::<HashSet<_>>()
    .into_iter()
    .map(|page| (page, sample_kinds.clone()))
    .collect::<HashMap<_, _>>();

    let mut all = sample_kinds;

    for analysis in matrix.analyses_by_kind() {
        let kind_all = all.get_mut(&analysis.kind).unwrap();

        for item in &analysis.linked {
            let page = rows.get_mut(&item.page).unwrap().get_mut(&item.kind).unwrap();
            page.linked += 1;
            page.total += 1;

            kind_all.linked += 1;
            kind_all.total += 1;
        }

        for item in &analysis.unlinked {
            rows.get_mut(&item.page).unwrap().get_mut(&item.kind).unwrap().total += 1;
            kind_all.total += 1;
        }
    }

    let mut summary = rows
        .into_iter()
        .map(|(page, kinds)| SummaryRow { page: Some(page), kinds: kinds.into_values().collect() })
        .collect::<Vec<_>>();
    summary.sort_by_key(|summary| summary.page.clone());
    summary.push(SummaryRow { page: None, kinds: all.into_values().collect() });

    // We want the summary to be in the same order as everything else, so we get the ordering index
    // for each kind depending on their ordering in analyses_by_kind().
    let kinds_order = matrix
        .analyses_by_kind()
        .enumerate()
        .map(|(i, analysis)| (analysis.kind, i))
        .collect::<HashMap<_, _>>();

    for row in &mut summary {
        row.kinds.sort_by_key(|kind| kinds_order.get(&kind.kind).copied());
        for kind in &mut row.kinds {
            if kind.total == 0 {
                kind.percentage = 100.0;
            } else {
                kind.percentage = kind.linked as f32 * 100.0 / kind.total as f32;
            }
        }
    }

    summary
}
