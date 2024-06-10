// Test linking using `cc` with `rust-lld`, using the unstable CLI described in MCP 510
// see https://github.com/rust-lang/compiler-team/issues/510 for more info

//@ needs-rust-lld
//@ ignore-msvc
//@ ignore-s390x lld does not yet support s390x as target

use run_make_support::regex::Regex;
use run_make_support::rustc;
use std::process::Output;

fn main() {
    // Opt-in to lld and the self-contained linker, to link with rust-lld. We'll check that by
    // asking the linker to display its version number with a link-arg.
    let output = rustc()
        .env("RUSTC_LOG", "rustc_codegen_ssa::back::link=info")
        .arg("-Zlinker-features=+lld")
        .arg("-Clink-self-contained=+linker")
        .arg("-Zunstable-options")
        .link_arg("-Wl,-v")
        .input("main.rs")
        .run();
    assert!(
        find_lld_version_in_logs(output.stderr_utf8()),
        "the LLD version string should be present in the output logs:\n{}",
        output.stderr_utf8()
    );

    // It should not be used when we explictly opt-out of lld.
    let output = rustc()
        .env("RUSTC_LOG", "rustc_codegen_ssa::back::link=info")
        .link_arg("-Wl,-v")
        .arg("-Zlinker-features=-lld")
        .input("main.rs")
        .run();
    assert!(
        !find_lld_version_in_logs(output.stderr_utf8()),
        "the LLD version string should not be present in the output logs:\n{}",
        output.stderr_utf8()
    );

    // While we're here, also check that the last linker feature flag "wins" when passed multiple
    // times to rustc.
    let output = rustc()
        .env("RUSTC_LOG", "rustc_codegen_ssa::back::link=info")
        .link_arg("-Wl,-v")
        .arg("-Clink-self-contained=+linker")
        .arg("-Zunstable-options")
        .arg("-Zlinker-features=-lld")
        .arg("-Zlinker-features=+lld")
        .arg("-Zlinker-features=-lld,+lld")
        .input("main.rs")
        .run();
    assert!(
        find_lld_version_in_logs(output.stderr_utf8()),
        "the LLD version string should be present in the output logs:\n{}",
        output.stderr_utf8()
    );
}

fn find_lld_version_in_logs(stderr: String) -> bool {
    let lld_version_re = Regex::new(r"^LLD [0-9]+\.[0-9]+\.[0-9]+").unwrap();
    stderr.lines().any(|line| lld_version_re.is_match(line.trim()))
}
