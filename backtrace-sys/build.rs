use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let src = env::current_dir().unwrap();
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    // libbacktrace doesn't currently support Mach-O files
    if target.contains("darwin") { return }
    // libbacktrace isn't used on windows
    if target.contains("windows") { return }
    // libbacktrace is already included in the linux libstd for rust
    if target.contains("linux") { return }

    run(Command::new(src.join("src/libbacktrace/configure"))
                .current_dir(&dst)
                .arg("--with-pic")
                .arg("--disable-multilib")
                .arg("--disable-shared")
                .arg("--disable-host-shared"));
    run(Command::new("make")
                .current_dir(&dst)
                .arg(format!("INCDIR={}",
                             src.join("src/libbacktrace").display())));
    println!("cargo:rustc-link-search=native={}/.libs", dst.display());
    println!("cargo:rustc-link-lib=static=backtrace");
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => panic!("failed to get status: {}", e),
    };
    if !status.success() {
        panic!("failed with: {}", status);
    }
}
