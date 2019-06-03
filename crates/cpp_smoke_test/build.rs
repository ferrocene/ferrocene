extern crate gcc;

fn main() {
    compile_cpp();
}

fn compile_cpp() {
    println!("cargo:rerun-if-changed=cpp/trampoline.cpp");

    gcc::Config::new()
        .cpp(true)
        .debug(true)
        .opt_level(0)
        .file("cpp/trampoline.cpp")
        .compile("libcpptrampoline.a");
}
