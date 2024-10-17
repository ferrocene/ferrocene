// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ops::Deref;

use askama::Template;

use crate::annotations::{AnnotationSource, Annotations};
use crate::matrix::{Element, ElementKind, LinkTest, Page, TraceabilityMatrix};

#[derive(Template)]
#[template(path = "report.html")]
struct Report<'a> {
    considers_ignored_tests: bool,
    matrix: &'a TraceabilityMatrix,
    summary: Vec<SummaryRow<'a>>,
    ignored_tests: BTreeMap<String, BTreeSet<String>>,
    urls: Urls,
}

struct SummaryRow<'a> {
    page: Option<&'a Page>,
    kinds: Vec<SummaryItem>,
}

impl SummaryRow<'_> {
    fn color(&self) -> &'static str {
        let linked = self.kinds.iter().map(|k| k.linked + k.informational).sum::<usize>();
        let total = self.kinds.iter().map(|k| k.total).sum::<usize>();

        if linked == total {
            "green"
        } else if linked == 0 {
            "red"
        } else {
            "orange"
        }
    }
}

#[derive(Clone)]
struct SummaryItem {
    kind: &'static ElementKind,
    linked: usize,
    informational: usize,
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
) -> anyhow::Result<String> {
    Ok(Report {
        matrix,
        summary: build_summary(matrix),
        urls,
        ignored_tests: annotations.ignored_tests.clone(),
        considers_ignored_tests: annotations.considers_ignored_tests,
    }
    .render()?)
}

fn build_summary(matrix: &TraceabilityMatrix) -> Vec<SummaryRow<'_>> {
    let sample_kinds = matrix
        .analyses_by_kind()
        .map(|a| {
            (a.kind, SummaryItem {
                kind: a.kind,
                linked: 0,
                informational: 0,
                total: 0,
                percentage: 0.0,
            })
        })
        .collect::<HashMap<_, _>>();

    // The table is created in an earlier step because we want to make sure all kinds are present,
    // even if a page doesn't contain that kind of data.
    let mut rows = crate::utils::chain(
        matrix.analyses_by_kind().flat_map(|a| &a.linked).map(|l| &l.page),
        matrix.analyses_by_kind().flat_map(|a| &a.unlinked).map(|u| &u.page),
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
            if item.informational() {
                page.informational += 1;
                kind_all.informational += 1;
            } else {
                page.linked += 1;
                kind_all.linked += 1;
            }
            page.total += 1;
            kind_all.total += 1;
        }

        let mut f = |page: &mut SummaryItem, _: &Element| {
            page.total += 1;
            kind_all.total += 1;
        };
        for_each_page(analysis.partially_linked.iter().map(|a| a.deref()), &mut rows, &mut f);
        for_each_page(analysis.unlinked.iter(), &mut rows, &mut f);
    }

    let mut summary = rows
        .into_iter()
        .map(|(page, kinds)| SummaryRow { page: Some(page), kinds: kinds.into_values().collect() })
        .collect::<Vec<_>>();
    summary.sort_by_key(|summary| summary.page);
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
                kind.percentage =
                    (kind.linked + kind.informational) as f32 * 100.0 / kind.total as f32;
            }
        }
    }

    summary
}

fn for_each_page<'a, F, I>(
    items: I,
    rows: &mut HashMap<&Page, HashMap<&ElementKind, SummaryItem>>,
    mut f: F,
) where
    F: FnMut(&mut SummaryItem, &'a Element),
    I: Iterator<Item = &'a Element>,
{
    for item in items {
        let page = rows.get_mut(&item.page).unwrap().get_mut(&item.kind).unwrap();
        f(page, item);
    }
}
