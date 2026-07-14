use std::fs;
use std::path::Path;

use crate::diagnostics::{CheckId, TidyCtx};

pub fn check(root_path: &Path, tidy_ctx: TidyCtx) {
    let manuals = ["qnx7-manual", "qnx8-manual", "qualification-report", "rhivos-manual"];
    let basedir = root_path.join("ferrocene/doc");
    let mut check = tidy_ctx.start_check(CheckId::new("ferrocene_test_outcomes").path(&basedir));
    for manual in manuals {
        let dir_path = basedir.join(manual).join("src/rustc");

        for entry in fs::read_dir(&dir_path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_name() == "index.rst" {
                continue;
            }

            let target_page_path = entry.path();
            let contents = fs::read_to_string(&target_page_path).unwrap();
            let option_names = ["host", "target", "tested_target_with_std"];
            for name in option_names {
                let option = format!(":{name}:");
                if !contents.contains(&option) {
                    check.error(format!(
                        "the following target page is missing the {option} option: {}",
                        target_page_path.display()
                    ))
                }
            }
        }
    }
}
