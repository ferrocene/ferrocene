#!/usr/bin/env nu
use std/dirs

let host = (sys host)
cd ($env.CURRENT_FILE | path dirname)

def main [
  --debug # Toggle debug
  --std # If std should be tested, too
  --ferrocene-src: string # The path to ferrocene
] {
    let ferrocene = $ferrocene_src | default "/home/ci/project/ferrocene"
    let host_tuple = (rustc --print host-tuple)
    let certified_host = if ($host_tuple | str starts-with "x86_64") {
        "x86_64-unknown-ferrocene.certified"
    } else {
        "aarch64-unknown-ferrocene.certified"
    }
    let prof = $"($ferrocene)/build/tmp/ferrocene-library-($host_tuple).profdata"
    let std_build = $"($ferrocene)/build/host/stage1-std/($host_tuple)/release/deps/"
    let symbols = $"($ferrocene)/build/host/stage1-std/($certified_host)/release/symbol-report.json"

    let std_obj = if ($host | get long_os_version | str starts-with "Linux") {
        ls $std_build | where name =~ "libstd.*so" | get name | first
    } else if ($host | get long_os_version | str starts-with "macOS") {
        ls $std_build | where name =~ "libstd.*dylib" | get name | first
    } else {
        print "Unsupported OS"
        exit 1
    }
    let corebenches_bin = (ls -l $std_build 
        | where name =~ "corebenches"
        | where type == "file"
        | where mode == "rwxr-xr-x"
        | get name
        | first)
    let coretests_bin = (ls -l $std_build
        | where name =~ "coretests"
        | where type == "file"
        | where mode == "rwxr-xr-x"
        | get name
        | first)

    dirs add $ferrocene
    let rev = (git log --pretty=format:'%H' -n 1)
    dirs drop

    mut extra_arg = []
    if $debug { 
        $extra_arg = $extra_arg | append "--debug"
    }
    if $std {
        let stdbenches_bin = ls $std_build | where name =~ "stdbenches" | get name | first
        $extra_arg = $extra_arg ++ ["--object", "--debug"]
    }


    (cargo run --release -- show
        --instr-profile $prof
        --object $std_obj
        --object $corebenches_bin
        --object $coretests_bin
        --report $symbols
        --path-equivalence $"/rustc/($rev),($ferrocene)"
        --ferrocene-src $ferrocene
        --html-out coverage-report.html
        ...$extra_arg
    )
}
