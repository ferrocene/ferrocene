fn main() {
    println!("cargo:rerun-if-changed=src/callback.c");

    cc::Build::new()
        .opt_level(0)
        .flag("-g1")
        .file("src/callback.c")
        .compile("libcallback.a");
}
