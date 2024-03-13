// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::error::{CommandError, CommandErrorKind, FindBinaryInPathError};
use crate::Environment;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub(crate) fn run_command(command: &mut Command) -> Result<CommandOutput, CommandError> {
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let path = command.get_program().into();
    let args = command.get_args().map(|c| c.to_os_string()).collect::<Vec<_>>();
    let child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return Err(CommandError {
                path,
                args,
                kind: CommandErrorKind::StartupFailed { error },
            });
        }
    };
    let output = match child.wait_with_output() {
        Ok(output) => output,
        Err(error) => {
            return Err(CommandError { path, args, kind: CommandErrorKind::WaitFailed { error } });
        }
    };

    if output.status.success() {
        match String::from_utf8(output.stdout) {
            Ok(stdout) => Ok(CommandOutput { stdout }),
            Err(_) => Err(CommandError { path, args, kind: CommandErrorKind::NonUtf8Output }),
        }
    } else {
        Err(CommandError { path, args, kind: CommandErrorKind::Failure { output } })
    }
}

pub(crate) struct CommandOutput {
    pub(crate) stdout: String,
}

pub(crate) fn find_binary_in_path(
    environment: &Environment,
    name: &str,
) -> Result<PathBuf, FindBinaryInPathError> {
    let Some(path) = &environment.path else {
        return Err(FindBinaryInPathError::NoEnvironmentVariable);
    };

    for directory in std::env::split_paths(&path) {
        let binary = directory.join(name);
        if binary.is_file() {
            return Ok(binary);
        }
    }
    Err(FindBinaryInPathError::MissingBinary { name: name.into() })
}

pub(crate) struct DisplayList<'a, T: Display>(pub(crate) &'a [T]);

impl<T: Display> Display for DisplayList<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "<empty>");
        }

        for (idx, item) in self.0.iter().enumerate() {
            Display::fmt(item, f)?;
            if idx == self.0.len() - 1 {
                // Nothing
            } else if idx == self.0.len() - 2 {
                f.write_str(" and ")?;
            } else {
                f.write_str(", ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_find_binary_in_path_missing_path() {
        let env = Environment { path: None, ..Environment::gather() };

        let err = find_binary_in_path(&env, "vim").unwrap_err();
        assert!(matches!(err, FindBinaryInPathError::NoEnvironmentVariable));
    }

    #[test]
    fn test_find_binary_in_path_empty_path() {
        let env = path_env(&[(Path::new(""))]);

        match find_binary_in_path(&env, "vim") {
            Err(FindBinaryInPathError::MissingBinary { name }) if name == "vim" => {}
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn test_find_binary_in_path_one_path_no_binary() {
        let dir = tempfile::tempdir().unwrap();
        let env = path_env(&[dir.path()]);

        match find_binary_in_path(&env, "vim") {
            Err(FindBinaryInPathError::MissingBinary { name }) if name == "vim" => {}
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn test_find_binary_in_path_two_paths_no_binary() {
        let dir1 = tempfile::tempdir().unwrap();
        let dir2 = tempfile::tempdir().unwrap();
        let env = path_env(&[dir1.path(), dir2.path()]);

        match find_binary_in_path(&env, "vim") {
            Err(FindBinaryInPathError::MissingBinary { name }) if name == "vim" => {}
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn test_find_binary_in_path_one_path_one_binary() {
        let dir = tempfile::tempdir().unwrap();
        let bin = dir.path().join("vim");
        std::fs::write(&bin, b"").unwrap();

        let env = path_env(&[dir.path()]);
        assert_eq!(bin, find_binary_in_path(&env, "vim").unwrap());
    }

    #[test]
    fn test_find_binary_in_path_two_paths_one_binary() {
        let dir1 = tempfile::tempdir().unwrap();
        let dir2 = tempfile::tempdir().unwrap();
        let bin = dir1.path().join("vim");
        std::fs::write(&bin, b"").unwrap();

        let env = path_env(&[dir1.path(), dir2.path()]);
        assert_eq!(bin, find_binary_in_path(&env, "vim").unwrap());

        let env = path_env(&[dir2.path(), dir1.path()]);
        assert_eq!(bin, find_binary_in_path(&env, "vim").unwrap());
    }

    #[test]
    fn test_find_binary_in_path_two_paths_two_binaries() {
        let dir1 = tempfile::tempdir().unwrap();
        let dir2 = tempfile::tempdir().unwrap();
        let bin1 = dir1.path().join("vim");
        let bin2 = dir1.path().join("vim");
        std::fs::write(&bin1, b"").unwrap();
        std::fs::write(&bin2, b"").unwrap();

        let env = path_env(&[dir1.path(), dir2.path()]);
        assert_eq!(bin1, find_binary_in_path(&env, "vim").unwrap());

        let env = path_env(&[dir2.path(), dir1.path()]);
        assert_eq!(bin2, find_binary_in_path(&env, "vim").unwrap());
    }

    fn path_env(paths: &[&Path]) -> Environment {
        Environment { path: Some(std::env::join_paths(paths).unwrap()) }
    }

    #[test]
    fn test_display_list() {
        assert_eq!("<empty>", DisplayList::<&str>(&[]).to_string());
        assert_eq!("foo", DisplayList(&["foo"]).to_string());
        assert_eq!("foo and bar", DisplayList(&["foo", "bar"]).to_string());
        assert_eq!("foo, bar and baz", DisplayList(&["foo", "bar", "baz"]).to_string());
        assert_eq!(
            "foo, bar, baz and quux",
            DisplayList(&["foo", "bar", "baz", "quux"]).to_string()
        );
    }
}
