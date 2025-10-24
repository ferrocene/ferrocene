#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::fs::File;
use std::io::{self, Write};

use build_helper::symbol_report::{Function, Symbols};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_middle::ty::print::{
    with_no_trimmed_paths, with_no_visible_paths, with_resolve_crate_name,
};
use rustc_session::EarlyDiagCtxt;
use rustc_session::config::ErrorOutputType;
use rustc_span::FileNameDisplayPreference;

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

        let mut symbols = vec![];
        for def in tcx.hir_crate_items(()).definitions() {
            let kind = tcx.def_kind(def);
            if ![DefKind::Fn, DefKind::AssocFn, DefKind::Closure].contains(&kind) {
                continue;
            }
            // TODO: skip associated default functions inherited from the trait
            // https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/ty/struct.TyCtxt.html#method.provided_trait_methods
            if kind == DefKind::AssocFn {}
            let function_path = with_no_visible_paths!(with_resolve_crate_name!(
                with_no_trimmed_paths!(tcx.def_path_str(def))
            ));
            let span = tcx.hir_span_with_body(tcx.local_def_id_to_hir_id(def));
            let lines = tcx.sess.source_map().span_to_lines(span).expect("failed to look up span");
            let filename = lines.file.name.display(FileNameDisplayPreference::Local).to_string();
            let start_line = lines.lines.first().unwrap().line_index + 1;
            let end_line = lines.lines.last().unwrap().line_index + 1;
            symbols.push(Function { function_path, filename, start_line, end_line });
        }
        serde_json::to_writer_pretty(out, &Symbols(symbols)).expect("failed to serialize symbols");
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
