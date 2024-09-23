//@ run-pass
// these tests come from `/library/std/src/error/tests.rs`

#![feature(error_generic_member_access)]
#![feature(error_reporter)]

use std::backtrace::Backtrace;
use std::error::{Error, Report, Request};
use std::fmt;

fn main() {
    error_with_backtrace_outputs_correctly_with_one_source();
    error_with_backtrace_outputs_correctly_with_two_sources();
}

fn error_with_backtrace_outputs_correctly_with_one_source() {
    let trace = Backtrace::force_capture();
    let expected = format!(
        "\
The source of the error

Caused by:
      Error with backtrace

Stack backtrace:
{}",
        trace
    );
    let error = GenericError::new("Error with backtrace");
    let mut error = GenericError::new_with_source("The source of the error", error);
    error.backtrace = Some(trace);
    let report = Report::new(error).pretty(true).show_backtrace(true);

    println!("Error: {report}");
    assert_eq!(expected.trim_end(), report.to_string());
}

fn error_with_backtrace_outputs_correctly_with_two_sources() {
    let trace = Backtrace::force_capture();
    let expected = format!(
        "\
Error with two sources

Caused by:
   0: The source of the error
   1: Error with backtrace

Stack backtrace:
{}",
        trace
    );
    let mut error = GenericError::new("Error with backtrace");
    error.backtrace = Some(trace);
    let error = GenericError::new_with_source("The source of the error", error);
    let error = GenericError::new_with_source("Error with two sources", error);
    let report = Report::new(error).pretty(true).show_backtrace(true);

    println!("Error: {report}");
    assert_eq!(expected.trim_end(), report.to_string());
}

#[derive(Debug)]
struct GenericError<D> {
    message: D,
    backtrace: Option<Backtrace>,
    source: Option<Box<dyn Error + 'static>>,
}

impl<D> GenericError<D> {
    fn new(message: D) -> GenericError<D> {
        Self { message, backtrace: None, source: None }
    }

    fn new_with_source<E>(message: D, source: E) -> GenericError<D>
    where
        E: Error + 'static,
    {
        let source: Box<dyn Error + 'static> = Box::new(source);
        let source = Some(source);
        GenericError { message, backtrace: None, source }
    }
}

impl<D> fmt::Display for GenericError<D>
where
    D: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.message, f)
    }
}

impl<D> Error for GenericError<D>
where
    D: fmt::Debug + fmt::Display,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }

    fn provide<'a>(&'a self, req: &mut Request<'a>) {
        self.backtrace.as_ref().map(|bt| req.provide_ref::<Backtrace>(bt));
    }
}
