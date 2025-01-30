//@ ignore-cross-compile

use run_make_support::{build_native_static_lib, run, rustc};

fn main() {
    build_native_static_lib("floatlib");
    rustc().input("main.rs").arg("-lstatic=floatlib").run();
    run("main");
}
