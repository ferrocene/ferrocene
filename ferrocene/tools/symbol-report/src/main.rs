#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

extern crate serde;
extern crate serde_json;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{self, Write};
use std::sync::LazyLock;

use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::print::{
    with_no_trimmed_paths, with_no_visible_paths, with_resolve_crate_name,
};
use rustc_session::EarlyDiagCtxt;
use rustc_session::config::ErrorOutputType;
use rustc_span::{FileNameDisplayPreference, Span, Symbol};

static FERROCENE_ANNOTATION_PATH: LazyLock<[Symbol; 2]> =
    LazyLock::new(|| ["ferrocene", "annotation"].map(Symbol::intern));

#[derive(serde::Serialize)]
struct Function(String, String, usize, usize);

#[derive(serde::Serialize)]
struct Report {
    symbols: Vec<Function>,
    annotations: BTreeMap<String, BTreeSet<(usize, usize)>>,
}

impl Report {
    fn add_annotation(&mut self, filename: String, start: usize, end: usize) {
        self.annotations.entry(filename).or_default().insert((start, end));
    }
}

struct Vis<'v> {
    tcx: TyCtxt<'v>,
    report: Report,
}

impl<'v> Vis<'v> {
    fn convert_span(&mut self, span: Span) -> (String, usize, usize) {
        let lines = self.tcx.sess.source_map().span_to_lines(span).expect("failed to look up span");
        let filename = lines.file.name.display(FileNameDisplayPreference::Local).to_string();
        let start = lines.lines.first().unwrap().line_index;
        let end = lines.lines.last().unwrap().line_index;

        (filename, start + 1, end + 1)
    }

    fn find_hir_id_annotations(&mut self, hir_id: rustc_hir::HirId, span: Span) {
        if self
            .tcx
            .hir_attrs(hir_id)
            .iter()
            .any(|attr| attr.path_matches(FERROCENE_ANNOTATION_PATH.as_slice()))
        {
            let (filename, start, end) = self.convert_span(span);
            self.report.add_annotation(filename, start, end);
        }
    }
}

impl<'v> rustc_hir::intravisit::Visitor<'v> for Vis<'v> {
    type NestedFilter = rustc_middle::hir::nested_filter::All;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_arm(&mut self, arm: &'v rustc_hir::Arm<'v>) -> Self::Result {
        self.find_hir_id_annotations(arm.hir_id, arm.span);
        rustc_hir::intravisit::walk_arm(self, arm)
    }

    fn visit_expr(&mut self, expr: &'v rustc_hir::Expr<'v>) -> Self::Result {
        self.find_hir_id_annotations(expr.hir_id, expr.span);
        rustc_hir::intravisit::walk_expr(self, expr)
    }
}

struct LoadCoreSymbols;

impl Callbacks for LoadCoreSymbols {
    fn after_expansion(&mut self, _: &Compiler, tcx: TyCtxt<'_>) -> Compilation {
        // NOTE: this can't be in main because it shouldn't execute when only running
        // --print=file-names
        let out = match std::env::var("SYMBOL_REPORT_OUT") {
            Ok(p) => Box::new(File::create(&p).expect(&format!("could not create file {p}")))
                as Box<dyn Write + Send>,
            Err(_) => Box::new(io::stdout()) as _,
        };
        let mut vis =
            Vis { tcx, report: Report { symbols: Vec::new(), annotations: BTreeMap::new() } };

        for def in tcx.hir_crate_items(()).definitions() {
            let kind = tcx.def_kind(def);
            if ![DefKind::Fn, DefKind::AssocFn, DefKind::Closure].contains(&kind) {
                continue;
            }
            // TODO: skip associated default functions inherited from the trait
            // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.provided_trait_methods
            if kind == DefKind::AssocFn {}
            let qualified_name = with_no_visible_paths!(with_resolve_crate_name!(
                with_no_trimmed_paths!(tcx.def_path_str(def))
            ));

            let span = tcx.hir_span_with_body(tcx.local_def_id_to_hir_id(def));
            let (filename, start, end) = vis.convert_span(span);

            if tcx.has_attrs_with_path(def, FERROCENE_ANNOTATION_PATH.as_slice()) {
                vis.report.add_annotation(filename.clone(), start, end)
            }

            vis.report.symbols.push(Function(qualified_name, filename, start, end));
        }

        tcx.hir_visit_all_item_likes_in_crate(&mut vis);

        serde_json::to_writer(out, &vis.report).expect("failed to serialize report");
        Compilation::Stop
    }
}

fn main() {
    rustc_driver::install_ice_hook("https://github.com/ferrocene/ferrocene/issues/new", |_| ());
    let handler = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&handler);
    std::process::exit(rustc_driver::catch_with_exit_code(move || {
        let args: Vec<String> = std::env::args().collect();
        rustc_driver::run_compiler(&args, &mut LoadCoreSymbols)
    }))
}
