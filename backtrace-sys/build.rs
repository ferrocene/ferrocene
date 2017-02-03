extern crate gcc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

fn main() {
    let src = env::current_dir().unwrap();
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    // libbacktrace doesn't currently support Mach-O files
    if target.contains("darwin") {
        return
    }

    // libbacktrace isn't used on windows
    if target.contains("windows") {
        return
    }

    // no way this will ever compile for emscripten
    if target.contains("emscripten") {
        return
    }

    let cfg = gcc::Config::new();
    let compiler = cfg.get_compiler();
    let mut flags = OsString::new();
    for (i, flag) in compiler.args().iter().enumerate() {
        if i > 0 {
            flags.push(" ");
        }
        flags.push(flag);
    }
    run(Command::new(src.join("src/libbacktrace/configure"))
                .current_dir(&dst)
                .env("CC", compiler.path())
                .env("CFLAGS", flags)
                .arg("--with-pic")
                .arg("--disable-multilib")
                .arg("--disable-shared")
                .arg("--disable-host-shared")
                .arg(format!("--target={}", target))
                .arg(format!("--host={}", host)));
    run(Command::new("make")
                .current_dir(&dst)
                .arg(format!("INCDIR={}",
                             src.join("src/libbacktrace").display())));
    println!("cargo:rustc-link-search=native={}/.libs", dst.display());
    println!("cargo:rustc-link-lib=static=backtrace");

    // The standard library currently bundles in libbacktrace, but it's
    // compiled with hidden visibility (naturally) so we can't use it.
    //
    // To prevent conflicts with a second statically linked copy we rename all
    // symbols with a '__rbt_' prefix manually here through `objcopy`.
    let lib = dst.join(".libs/libbacktrace.a");
    let tmpdir = dst.join("__tmp");
    drop(fs::remove_dir_all(&tmpdir));
    t!(fs::create_dir_all(&tmpdir));
    run(Command::new("ar").arg("x").arg(&lib).current_dir(&tmpdir));

    t!(fs::remove_file(&lib));
    let mut objs = Vec::new();
    for obj in t!(tmpdir.read_dir()) {
        let obj = t!(obj);
        run(Command::new("objcopy")
                    .arg("--redefine-syms=symbol-map")
                    .arg(obj.path()));
        objs.push(obj.path());
    }

    run(Command::new("ar").arg("crus").arg(&lib).args(&objs));
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
