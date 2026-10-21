use std::env;

fn main() {
    // this is needed because `HOST` is only available to build scripts.
    let host = env::var("HOST").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=BUILD_TRIPLE={host}");

    // Ferrocene addition: Raise the stack limit on windows, otherwise parsing
    // CLI args fails with stack overflow.
    const WINDOWS_STACK_LIMIT: usize = 0x400000; // 4MiB
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    if os == "windows" {
        match env.as_str() {
            "msvc" => println!("cargo:rustc-link-arg-bins=/STACK:{WINDOWS_STACK_LIMIT}"),
            // Currently we don't run x86_64-pc-windows-gnu in CI, may change in the future
            // "gnu" => println!("cargo:rustc-link-arg-bins=-Wl,--stack,{WINDOWS_STACK_LIMIT}"),
            _ => {}
        }
    }
}
