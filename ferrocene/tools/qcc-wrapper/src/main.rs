//! `qcc` drop in replacement that uses `rust-lld` instead of QNX's `$TARGET-ld`

use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let qcc_args = env::args().skip(1).collect::<Vec<_>>();

    let mut qcc = Command::new("qcc");
    let output = qcc.args(["-n", "-vv"]).args(qcc_args).output()?;

    if !output.status.success() {
        return Err(format!("failed to execute `{qcc:?}`").into());
    }

    let stderr = String::from_utf8(output.stderr)?;
    let mut linker_invocation = None;
    for line in stderr.lines() {
        let line = line.trim();
        if line.starts_with("cc: ") || line.is_empty() {
            // skip log statements
            continue;
        }

        // FIXME arguments may contain whitespace and `qcc -vv` does not escape whitespace
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

        let Some(arg0) = parts.first() else { continue };

        // linker invocation uses the absolute path to the linker
        let linker = Path::new(arg0);
        if linker.is_absolute() {
            if linker_invocation.is_some() {
                return Err("found multiple linker invocations but expected only one".into());
            }

            linker_invocation = Some(parts);
        }
    }

    let Some(cmd_and_args) = linker_invocation else {
        return Err(format!("did not find the linker invocation.\nqcc's stderr:\n{stderr}").into());
    };

    let mut lld = Command::new("rust-lld");
    lld.args(["-flavor", "gnu"]);

    let skip_cmd = 1; // args[0] is the binary name
    let lld_args = process_linker_args(&cmd_and_args[skip_cmd..]);
    lld.args(lld_args);

    let status = lld.status()?;

    if status.success() { Ok(()) } else { Err(format!("failed to execute `{lld:?}`").into()) }
}

fn process_linker_args<T: AsRef<str>>(args: impl IntoIterator<Item = T>) -> Vec<String> {
    let args = args.into_iter();

    let mut processed = vec![];
    for arg in args {
        let arg = arg.as_ref();

        // LLD does not support -Y. -Y is the same as -L and GNU LD supports it for
        // Solaris compatibility
        if let Some(value) = arg.strip_prefix("-Y") {
            processed.push(format!("-L{value}"));
            // FIXME we may see these with whitespace instead of the `=`
        } else if arg.starts_with("-plugin-opt=") || arg.starts_with("-plugin=") {
            // filter out these args that are not allowed by the safety manual
            continue;
        } else {
            processed.push(arg.to_string());
        }
    }

    processed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y_to_l() {
        assert_eq!(vec!["-L/a/b:/c/d"], process_linker_args(["-Y/a/b:/c/d"]));
        assert_eq!(vec!["-L", "/a/b:/c/d"], process_linker_args(["-Y", "/a/b:/c/d"]));
    }

    #[test]
    fn filter_plugin() {
        assert!(process_linker_args(["-plugin=/a/b"]).is_empty());
        assert!(process_linker_args(["-plugin-opt=/a/b"]).is_empty());
    }
}
