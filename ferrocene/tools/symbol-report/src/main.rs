#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::io;
use std::process::ExitCode;
use std::sync::LazyLock;

use build_helper::symbol_report::{Function, SymbolReport};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{LOCAL_CRATE, LocalDefId};
use rustc_hir::{AttrId, Attribute, HirId};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::{Instance, TyCtxt};
use rustc_middle::ty::print::{
    with_no_trimmed_paths, with_no_visible_paths, with_resolve_crate_name,
};
use rustc_session::EarlyDiagCtxt;
use rustc_session::config::ErrorOutputType;
use rustc_span::{Span, Symbol};

static FERROCENE_ANNOTATION_PATH: LazyLock<[Symbol; 2]> =
    LazyLock::new(|| ["ferrocene", "annotation"].map(Symbol::intern));

struct Vis<'tcx> {
    tcx: TyCtxt<'tcx>,
    report: SymbolReport,
    visited_attrs: BTreeSet<AttrId>,
}

impl<'tcx> Vis<'tcx> {
    fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self { tcx, report: SymbolReport::new(), visited_attrs: BTreeSet::new() }
    }

    fn convert_span(&mut self, span: Span) -> (String, usize, usize) {
        let lines = self.tcx.sess.source_map().span_to_lines(span).expect("failed to look up span");
        let filename = lines.file.name.prefer_local_unconditionally().to_string();
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

struct LoadCoreSymbols {
    /// Names of the crates we want a symbol report for.
    /// Their dependencies are not listed.
    target_crates: HashSet<String>,
}

impl Callbacks for LoadCoreSymbols {
    fn after_expansion(&mut self, _: &Compiler, tcx: TyCtxt<'_>) -> Compilation {
        let crate_name = tcx.crate_name(LOCAL_CRATE).to_string();
        if !self.target_crates.contains(&crate_name) {
            // Not one of the crates we build a report for, meaning that
            // it's a dependency. We still need to compile it to produce
            // `.rmeta` files for the compiler.
            return Compilation::Continue;
        }

        let vis = Vis::new(tcx);
        let mut vis = extract_all_functions(tcx, vis);

        tcx.hir_visit_all_item_likes_in_crate(&mut vis);
        // It is very important that we walk the attributes *after* we visit the rest of the items.
        // This allows us to detect unused annotations.
        tcx.hir_walk_attributes(&mut vis);

        match std::env::var("SYMBOL_REPORT_OUT") {
            Ok(p) => {
                // When cargo builds multiple crates each gets it's own
                // compiler process. At the end this process would only produce
                // a report for a single crate.
                // So instead of making a brand new file we check if one exists
                // already, load it, merge our report data into it, and store
                // it again.
                let mut report = std::fs::read_to_string(&p)
                    .ok()
                    .and_then(|existing| serde_json::from_str::<SymbolReport>(&existing).ok())
                    .unwrap_or_else(SymbolReport::new);
                report.symbols.extend(vis.report.symbols);
                for (filename, lines) in vis.report.annotations {
                    report.annotations.entry(filename).or_default().extend(lines);
                }
                let out = File::create(&p).expect(&format!("could not create file {p}"));
                serde_json::to_writer(out, &report).expect("failed to serialize report");
            }
            Err(_) => {
                serde_json::to_writer(io::stdout(), &vis.report)
                    .expect("failed to serialize report");
            }
        }

        Compilation::Continue
    }
}

fn main() {
    rustc_driver::install_ice_hook("https://github.com/ferrocene/ferrocene/issues/new", |_| ());
    let handler = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&handler);
    let target_crates: HashSet<String> = std::env::var("SYMBOL_REPORT_CRATES")
        .map(|v| v.split(',').map(str::to_owned).collect())
        .unwrap_or_default();
    let exit_code = rustc_driver::catch_with_exit_code(move || {
        let args: Vec<String> = std::env::args().collect();
        rustc_driver::run_compiler(&args, &mut LoadCoreSymbols { target_crates })
    });
    let exit_code = if exit_code == ExitCode::SUCCESS {
        rustc_driver::EXIT_SUCCESS
    } else {
        rustc_driver::EXIT_FAILURE
    };
    std::process::exit(exit_code);
}

use rustc_middle::middle::codegen_fn_attrs::ferrocene::item_is_validated;

fn extract_all_functions<'tcx>(tcx: TyCtxt<'tcx>, mut vis: Vis<'tcx>) -> Vis<'tcx> {
    for def in tcx.hir_crate_items(()).definitions() {
        match tcx.def_kind(def) {
            DefKind::Mod | DefKind::Struct | DefKind::Enum | DefKind::Union | DefKind::Use => {
                continue;
            }
            _ => {}
        }

        if !item_is_validated(tcx, def.into()).needs_test() {
            continue;
        }

        let qualified_name = get_qualified_name(tcx, def);

        if should_filter_out(&qualified_name) {
            continue;
        }

        let (filename, mut start_line, end_line) = get_span(tcx, &mut vis, def);

        // We don't check for annotations those inside the `Visitor` implementation so we do it
        // here.
        if let Some(attr) = has_ferrocene_annotation(tcx, def) {
            vis.report.add_annotation(filename.clone(), start_line, end_line);
            vis.visited_attrs.insert(attr.id());
            // Set the start line of the function as the minimum between the start line of the
            // function and the start of the annotation so it can be seen in the coverage report.
            let (_, span_start_line, _) = vis.convert_span(attr.span());
            start_line = start_line.min(span_start_line);
        }

        let linkage_name = get_linkage_name(tcx, def);

        vis.report.symbols.push(Function {
            qualified_name,
            filename,
            start_line,
            end_line,
            linkage_name,
        });
    }

    vis
}

/// Returns the mangled linkage (symbol) name for a compiled monomorphic item.
/// For generic items, and for items that aren't a function/closure to begin with (a `use` or a
/// local `macro_rules!` can end up "needing a test" by inheriting validation from their
/// surrounding function, but neither has a linkage name to compute), returns an empty string.
/// Later when a generic item is used by a dependent binary the compiler will
/// produce a new monomorphized item with a new name.
fn get_linkage_name(tcx: TyCtxt<'_>, def: LocalDefId) -> String {
    if !tcx.def_kind(def).is_fn_like() || tcx.generics_of(def.to_def_id()).count() != 0 {
        return String::new();
    }
    let instance = Instance::mono(tcx, def.to_def_id());
    tcx.symbol_name(instance).name.to_string()
}

fn get_qualified_name(tcx: TyCtxt<'_>, def: LocalDefId) -> String {
    with_no_visible_paths!(with_resolve_crate_name!(with_no_trimmed_paths!(tcx.def_path_str(def))))
}

/// Filter out functions that are marked prevalidated for internal use only
fn should_filter_out(qualified_name: &str) -> bool {
    const FILTER_LIST: &[&str] = &["core::ferrocene_test", "core::core_arch"];

    FILTER_LIST
        .iter()
        // Some symbols start with `<`, some without
        .flat_map(|s| [s.to_string(), format!("<{s}")])
        .any(|filter| qualified_name.starts_with(&filter))
}

fn get_span(tcx: TyCtxt<'_>, vis: &mut Vis<'_>, def: LocalDefId) -> (String, usize, usize) {
    let span = tcx.hir_span_with_body(tcx.local_def_id_to_hir_id(def));
    let (filename, start_line, end_line) = vis.convert_span(span);
    (filename, start_line, end_line)
}

fn has_ferrocene_annotation(tcx: TyCtxt<'_>, def: LocalDefId) -> Option<&Attribute> {
    tcx.get_attrs_by_path(def.into(), FERROCENE_ANNOTATION_PATH.as_slice()).next()
}
