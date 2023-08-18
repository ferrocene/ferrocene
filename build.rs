extern crate cc;

use std::env;
use std::path::Path;

// Must be public so the build script of `std` can call it.
pub fn main() {
    match env::var("CARGO_CFG_TARGET_OS").unwrap_or_default().as_str() {
        "android" => build_android(),
        _ => {}
    }
}

fn build_android() {
    // Resolve `src/android-api.c` relative to this file.
    // Required to support calling this from the `std` build script.
    let android_api_c = Path::new(file!())
        .parent()
        .unwrap()
        .join("src/android-api.c");
    let expansion = match cc::Build::new().file(android_api_c).try_expand() {
        Ok(result) => result,
        Err(e) => {
            println!("failed to run C compiler: {}", e);
            return;
        }
    };
    let expansion = match std::str::from_utf8(&expansion) {
        Ok(s) => s,
        Err(_) => return,
    };
    println!("expanded android version detection:\n{}", expansion);
    let marker = "APIVERSION";
    let i = match expansion.find(marker) {
        Some(i) => i,
        None => return,
    };
    let version = match expansion[i + marker.len() + 1..].split_whitespace().next() {
        Some(s) => s,
        None => return,
    };
    let version = match version.parse::<u32>() {
        Ok(n) => n,
        Err(_) => return,
    };
    if version >= 21 {
        println!("cargo:rustc-cfg=feature=\"dl_iterate_phdr\"");
    }
}
