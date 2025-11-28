#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

extern crate tracing;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, Write};
use std::sync::LazyLock;

use build_helper::symbol_report::{Function, SymbolReport};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::{AttrId, HirId, Item, ItemKind, Node, TraitFn, TraitItem, TraitItemKind};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::print::{
    with_no_trimmed_paths, with_no_visible_paths, with_resolve_crate_name,
};
use rustc_session::EarlyDiagCtxt;
use rustc_session::config::ErrorOutputType;
use rustc_span::{FileNameDisplayPreference, Span, Symbol};
use tracing::info;

static FERROCENE_ANNOTATION_PATH: LazyLock<[Symbol; 2]> =
    LazyLock::new(|| ["ferrocene", "annotation"].map(Symbol::intern));

struct Vis<'tcx> {
    tcx: TyCtxt<'tcx>,
    report: SymbolReport,
    visited_attrs: BTreeSet<AttrId>,
}

impl<'tcx> Vis<'tcx> {
    fn convert_span(&mut self, span: Span) -> (String, usize, usize) {
        let lines = self.tcx.sess.source_map().span_to_lines(span).expect("failed to look up span");
        let filename = lines.file.name.display(FileNameDisplayPreference::Local).to_string();
        let start = lines.lines.first().unwrap().line_index;
        let end = lines.lines.last().unwrap().line_index;

        (filename, start + 1, end + 1)
    }

    fn find_hir_id_annotations(&mut self, hir_id: HirId, span: Span) {
        if let Some(attr) = self
            .tcx
            .hir_attrs(hir_id)
            .iter()
            .find(|attr| attr.path_matches(FERROCENE_ANNOTATION_PATH.as_slice()))
        {
            let (filename, start, end) = self.convert_span(span);
            self.report.add_annotation(filename, start, end);
            self.visited_attrs.insert(attr.id());
        }
    }
}

// This doesn't visit function's definitions as those are iterated in the `Callbacks`
// implementation.
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

    fn visit_attribute(&mut self, attr: &'v rustc_hir::Attribute) -> Self::Result {
        if attr.path_matches(FERROCENE_ANNOTATION_PATH.as_slice()) {
            if !self.visited_attrs.contains(&attr.id()) {
                eprintln!("Unused annotation at {:?}", attr.span());
            }
        }
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
        let mut vis = Vis { tcx, report: SymbolReport::new(), visited_attrs: BTreeSet::new() };

        for def in tcx.hir_crate_items(()).definitions() {
            let kind = tcx.def_kind(def);
            if ![DefKind::Fn, DefKind::AssocFn, DefKind::Closure].contains(&kind) {
                continue;
            }
            // Skip intrinsics, extern functions, and associated default functions provided by the trait.
            match tcx.hir_node_by_def_id(def) {
                Node::Item(Item { kind: ItemKind::Fn { has_body: false, .. }, .. })
                | Node::TraitItem(TraitItem {
                    kind: TraitItemKind::Fn(_, TraitFn::Required(_)),
                    ..
                }) => {
                    info!("skipping item {def:?}");
                    continue;
                }
                _ => {}
            }
            let qualified_name = with_no_visible_paths!(with_resolve_crate_name!(
                with_no_trimmed_paths!(tcx.def_path_str(def))
            ));

            let span = tcx.hir_span_with_body(tcx.local_def_id_to_hir_id(def));
            let (filename, start_line, end_line) = vis.convert_span(span);

            // We don't check for annotations those inside the `Visitor` implementation so we do it
            // here.
            if let Some(attr) =
                tcx.get_attrs_by_path(def.into(), FERROCENE_ANNOTATION_PATH.as_slice()).next()
            {
                vis.report.add_annotation(filename.clone(), start_line, end_line);
                vis.visited_attrs.insert(attr.id());
            }

            vis.report.symbols.push(Function { qualified_name, filename, start_line, end_line });
        }

        tcx.hir_visit_all_item_likes_in_crate(&mut vis);
        // It is very important that we walk the attributes *after* we visit the rest of the items.
        // This allows us to detect unused annotations.
        tcx.hir_walk_attributes(&mut vis);

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
