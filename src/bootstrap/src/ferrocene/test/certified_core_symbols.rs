// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::path::Path;
use std::process;

use build_helper::diff::diff_text;

use crate::builder::{Builder, RunConfig, ShouldRun, Step};
use crate::core::config::TargetSelection;
use crate::ferrocene::run::{self, CERTIFIED_CORE_SYMBOLS_ALIAS, update_certified_core_symbols};
use crate::ferrocene::tool::SYMBOL_PATH;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CertifiedCoreSymbols {
    host: TargetSelection,
}

impl Step for CertifiedCoreSymbols {
    type Output = ();
    const DEFAULT: bool = true;
    const IS_HOST: bool = true;

    fn should_run(run: ShouldRun<'_>) -> ShouldRun<'_> {
        run.path(SYMBOL_PATH).alias(CERTIFIED_CORE_SYMBOLS_ALIAS)
    }

    fn make_run(run: RunConfig<'_>) {
        run.builder.ensure(CertifiedCoreSymbols { host: run.target });
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        // load the expected symbol report
        let expected_path = Path::new(update_certified_core_symbols::TRACKED_FILE);
        let expected = builder.read(expected_path);

        if builder.config.dry_run() {
            return;
        }

        // generate the actual symbol report
        let host = self.host;
        let actual_path = builder.ensure(run::CertifiedCoreSymbols { host, target: host });
        let actual = builder.read(&actual_path);

        // compare the two
        if actual == expected {
            eprintln!("The certified core symbol report is up to date.")
        } else {
            eprintln!("Diff of {} and {}:", expected_path.display(), actual_path.display());
            diff_text(&expected, &actual);
            eprintln!(
                "The certified core symbol report is out of date. \
                Run `./x run update-certified-core-symbols` to update it."
            );
            process::exit(-1);
        }
    }
}
