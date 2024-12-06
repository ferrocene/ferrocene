use std::env;
use std::fs;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let mut command = Command::new(env::args().nth(1).expect("expects an argument"));
    let args = fs::read_to_string("/etc/args.txt")?;
    for line in args.lines() {
        command.arg(line);
    }
    let env = fs::read_to_string("/etc/env.txt")?;
    for line in env.lines() {
        if let Some((key, val)) = line.split_once('=') {
            command.env(key, val);
        }
    }
    let output = command.output()?;
    fs::write("/tmp/stdout.txt", output.stdout)?;
    fs::write("/tmp/stderr.txt", output.stderr)?;
    let status = output.status.code().expect("expects exit status code");
    fs::write("/tmp/exit-code.txt", status.to_string())?;

    Ok(())
}
