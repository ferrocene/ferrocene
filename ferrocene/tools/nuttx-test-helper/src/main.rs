use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let mut command = Command::new(env::args().nth(1).expect("expects an argument"));
    let args = fs::read_to_string("/etc/args.txt").expect("could not read /etc/args.txt");
    for line in args.lines() {
        command.arg(line);
    }
    let env = fs::read_to_string("/etc/env.txt").expect("could not read /etc/env.txt");
    for line in env.lines() {
        if let Some((key, val)) = line.split_once('=') {
            command.env(key, val);
        }
    }
    let output = command.output().expect("could not execute spawnee");
    fs::write("/tmp/stdout.txt", output.stdout).expect("could not write to /tmp/stdout.txt");
    fs::write("/tmp/stderr.txt", output.stderr).expect("could not write to /tmp/stderr.txt");
    let status = output.status.code().expect("expects exit status code");
    fs::write("/tmp/exit-code.txt", status.to_string())
        .expect("could not write to /tmp/exit-code.txt");
}
