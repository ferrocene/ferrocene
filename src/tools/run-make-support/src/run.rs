use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::command::{Command, CompletedProcess};
use crate::{cwd, env_var, is_windows, set_host_rpath};

use super::handle_failed_output;

fn run_common(name: &str) -> (Command, CompletedProcess) {
    let mut bin_path = PathBuf::new();
    bin_path.push(cwd());
    bin_path.push(name);
    let ld_lib_path_envvar = env_var("LD_LIB_PATH_ENVVAR");
    let mut cmd = Command::new(bin_path);
    cmd.env(&ld_lib_path_envvar, {
        let mut paths = vec![];
        paths.push(cwd());
        for p in env::split_paths(&env_var("TARGET_RPATH_ENV")) {
            paths.push(p.to_path_buf());
        }
        for p in env::split_paths(&env_var(&ld_lib_path_envvar)) {
            paths.push(p.to_path_buf());
        }
        env::join_paths(paths.iter()).unwrap()
    });

    if is_windows() {
        let mut paths = vec![];
        for p in env::split_paths(&std::env::var("PATH").unwrap_or(String::new())) {
            paths.push(p.to_path_buf());
        }
        paths.push(Path::new(&env_var("TARGET_RPATH_DIR")).to_path_buf());
        cmd.env("PATH", env::join_paths(paths.iter()).unwrap());
    }

    let output = cmd.command_output();
    (cmd, output)
}

/// Run a built binary and make sure it succeeds.
#[track_caller]
pub fn run(name: &str) -> CompletedProcess {
    let caller_location = std::panic::Location::caller();
    let caller_line_number = caller_location.line();

    let (cmd, output) = run_common(name);
    if !output.status().success() {
        handle_failed_output(&cmd, output, caller_line_number);
    }
    output
}

/// Run a built binary and make sure it fails.
#[track_caller]
pub fn run_fail(name: &str) -> CompletedProcess {
    let caller_location = std::panic::Location::caller();
    let caller_line_number = caller_location.line();

    let (cmd, output) = run_common(name);
    if output.status().success() {
        handle_failed_output(&cmd, output, caller_line_number);
    }
    output
}

/// Create a new custom Command.
/// This should be preferred to creating `std::process::Command` directly.
pub fn cmd<S: AsRef<OsStr>>(program: S) -> Command {
    let mut command = Command::new(program);
    set_host_rpath(&mut command);
    command
}
