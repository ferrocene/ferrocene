#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::fs::File;
use std::io::{self, Write};

use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;
use rustc_span::FileNameDisplayPreference;
use rustc_span::def_id::LOCAL_CRATE;
use rustc_middle::ty::print::with_no_trimmed_paths;

struct LoadCoreSymbols {
    out: Box<dyn Write + Send>,
}

impl Callbacks for LoadCoreSymbols {
    fn after_expansion(&mut self, _: &Compiler, tcx: TyCtxt<'_>) -> Compilation {
        let krate = tcx.crate_name(LOCAL_CRATE).to_string();
        for def in tcx.hir_crate_items(()).definitions() {
            if ![DefKind::Fn, DefKind::AssocFn, DefKind::Closure].contains(&tcx.def_kind(def)) {
                continue;
            }
            let path = with_no_trimmed_paths!(format!("{}::{}", krate, tcx.def_path_str(def)));
            let span = tcx.def_span(def);
            let lines = tcx.sess.source_map().span_to_lines(span).expect("failed to look up span");
            writeln!(self.out, "{path} @ {}:{}-{}",
                lines.file.name.display(FileNameDisplayPreference::Local),
                lines.lines.first().unwrap().line_index,
                lines.lines.last().unwrap().line_index,
            ).expect("failed to write output");
        }
        Compilation::Stop
    }
}

fn main() {
    rustc_driver::install_ice_hook(
        "https://github.com/ferrocene/comprehensive/issues/new",
        |_| (),
    );
    let handler = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&handler);
    let out = match std::env::var("SYMBOL_REPORT_OUT") {
        Ok(p) => Box::new(File::create(&p).expect(&format!("could not create file {p}")))
            as Box<dyn Write + Send>,
        Err(_) => Box::new(io::stdout()) as _,
    };
    let mut printer = LoadCoreSymbols { out };
    std::process::exit(rustc_driver::catch_with_exit_code(move || {
        let args: Vec<String> = std::env::args().collect();
        rustc_driver::run_compiler(&args, &mut printer)
    }))
}

