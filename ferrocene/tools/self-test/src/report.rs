// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::error::{CommandError, CommandErrorKind, Error};
use std::error::Error as _;

const TERMINAL_WIDTH: usize = 79;

pub(crate) trait Reporter {
    fn success(&self, message: &str);
    fn skipped(&self, message: &str);
    fn note(&self, message: &str);
    fn info(&self, message: &str);
    fn error(&self, error: &Error);
}

pub(crate) struct StderrReporter {
    color_bold: &'static str,
    color_bold_cyan: &'static str,
    color_bold_green: &'static str,
    color_bold_red: &'static str,
    color_bold_yellow: &'static str,
    color_reset: &'static str,
}

impl StderrReporter {
    pub(crate) fn plain() -> Self {
        Self {
            color_bold: "",
            color_bold_cyan: "",
            color_bold_green: "",
            color_bold_red: "",
            color_bold_yellow: "",
            color_reset: "",
        }
    }

    pub(crate) fn color() -> Self {
        Self {
            color_bold: "\x1b[1m",
            color_bold_cyan: "\x1b[1;36m",
            color_bold_green: "\x1b[1;32m",
            color_bold_red: "\x1b[1;31m",
            color_bold_yellow: "\x1b[1;33m",
            color_reset: "\x1b[0m",
        }
    }

    fn render_command_output(&self, heading: &str, content: &[u8]) {
        let padding_left = (TERMINAL_WIDTH - heading.len() - 2) / 2;
        let padding_right = TERMINAL_WIDTH - heading.len() - 2 - padding_left;

        let heading = std::iter::repeat('=')
            .take(padding_left)
            .chain(std::iter::once(' '))
            .chain(heading.chars())
            .chain(std::iter::once(' '))
            .chain(std::iter::repeat('=').take(padding_right))
            .collect::<String>();

        eprintln!("{}{heading}{}", self.color_bold, self.color_reset);
        eprint!("{}", String::from_utf8_lossy(content));
        if content.last() != Some(&b'\n') {
            eprintln!();
        }
    }
}

impl Reporter for StderrReporter {
    fn success(&self, message: &str) {
        eprintln!("{} Success:{} {message}", self.color_bold_green, self.color_reset);
    }

    fn skipped(&self, message: &str) {
        eprintln!("{} Skipped:{} {message}", self.color_bold_yellow, self.color_reset);
    }

    fn note(&self, message: &str) {
        eprintln!("{}    Note:{} {message}", self.color_bold, self.color_reset);
    }

    fn info(&self, message: &str) {
        eprintln!("{}    Info:{} {message}", self.color_bold_cyan, self.color_reset);
    }

    fn error(&self, error: &Error) {
        eprintln!("{}   Error:{} {error}", self.color_bold_red, self.color_reset);

        let mut source = error.source();
        let mut command_output = None;
        while let Some(s) = source {
            eprintln!("{}   Cause:{} {s}", self.color_bold, self.color_reset);

            if let Some(CommandError { kind: CommandErrorKind::Failure { output }, .. }) =
                s.downcast_ref()
            {
                command_output = Some(output);
            }
            source = s.source();
        }

        self.note(&format!("the error code is FST_{:0>3}", error.code()));

        if let Some(output) = command_output {
            if !output.stdout.is_empty() || !output.stderr.is_empty() {
                eprintln!();
            }
            if !output.stdout.is_empty() {
                self.render_command_output("stdout", &output.stdout);
            }
            if !output.stderr.is_empty() {
                self.render_command_output("stderr", &output.stderr);
            }
            if !output.stdout.is_empty() || !output.stderr.is_empty() {
                let footer = "=".repeat(TERMINAL_WIDTH);
                eprintln!("{}{footer}{}", self.color_bold, self.color_reset);
            }
        }
    }
}
