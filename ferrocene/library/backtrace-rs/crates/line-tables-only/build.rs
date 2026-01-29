fn main() {
    println!("cargo:rerun-if-changed=src/callback.c");

    cc::Build::new()
        .opt_level(0)
        // Don't use -gline-tables-only: it breaks on platforms that don't use clang (Linux with gcc, etc.)
        .flag("-g1")
        .file("src/callback.c")
        .compile("libcallback.a");
}
