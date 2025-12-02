// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::Path;

use build_helper::diff::diff_text;
use build_helper::symbol_report::SymbolReport;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::run::update_certified_core_symbols::TRACKED_FILE;
use crate::ferrocene::run::{self, CERTIFIED_CORE_SYMBOLS_ALIAS, update_certified_core_symbols};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CertifiedCoreSymbols;

impl Step for CertifiedCoreSymbols {
    type Output = ();
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(TRACKED_FILE).alias(CERTIFIED_CORE_SYMBOLS_ALIAS)
    }

    fn is_default_step(_: &Builder<'_>) -> bool {
        true
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(CertifiedCoreSymbols);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        builder.info(&format!("Testing {TRACKED_FILE}"));
        let target = TargetSelection::from_user("x86_64-unknown-linux-gnu");
        let actual_symbol_report_path =
            builder.ensure(run::CertifiedCoreSymbols::new(builder, target));

        if builder.config.dry_run() {
            return;
        }

        // load the expected list of qualified functions
        let expected_path = Path::new(update_certified_core_symbols::TRACKED_FILE);
        let mut expected: Vec<String> = Default::default();
        let reader = builder.read(expected_path);
        for qualified_name in reader.lines() {
            expected.push(qualified_name.to_string());
        }

        // generate the actual list of qualified functions
        let actual_symbol_report_content = builder.read(&actual_symbol_report_path);
        let actual_symbol_report =
            serde_json::from_str::<SymbolReport>(&actual_symbol_report_content).unwrap();
        let actual: Vec<String> = actual_symbol_report.to_qualified_fn_list();

        // compare the two
        if actual == expected {
            builder.info(&format!("The certified core symbol report is up to date."));
        } else {
            builder.info(&format!(
                "Diff of {} and {}:",
                expected_path.display(),
                actual_symbol_report_path.display(),
            ));

            let actual_content = actual.join("\n");
            let expected_content = builder.read(expected_path);
            diff_text(&expected_content, &actual_content);

            builder.info(&format!(
                "The certified core symbol report is out of date. \
                Run `./x run update-certified-core-symbols` to update it."
            ));
            crate::exit!(1);
        }
    }
}
