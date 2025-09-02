use std::env;
use std::io::IsTerminal;
use std::sync::Arc;

<<<<<<< HEAD
use compiletest::{early_config_check, ferrocene_annotations, log_config, parse_config, run_tests};
=======
use compiletest::{early_config_check, parse_config, run_tests};
>>>>>>> pull-upstream-temp--do-not-use-for-real-code

fn main() {
    tracing_subscriber::fmt::init();

    // colored checks stdout by default, but for some reason only stderr is a terminal.
    // compiletest *does* print many things to stdout, but it doesn't really matter.
    if std::io::stderr().is_terminal()
        && matches!(std::env::var("NO_COLOR").as_deref(), Err(_) | Ok("0"))
    {
        colored::control::set_override(true);
    }

    ferrocene_annotations::maybe_collect_and_exit();

    let config = Arc::new(parse_config(env::args().collect()));

    early_config_check(&config);

    run_tests(config);
}
