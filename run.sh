#!/bin/sh
set -e

ferrocene=~/src/ferrocene2
host=aarch64-apple-darwin
certified_host=aarch64-unknown-ferrocene.certified
prof=$ferrocene/build/tmp/ferrocene-library-${host}.profdata
std_build=$ferrocene/build/host/stage1-std/${host}/release/deps/
symbols=$ferrocene/build/host/stage1-std/${certified_host}/release/symbol-report.json
objs=$(find ${std_build} -type f -perm +111 | sed 's/^/--object /')

cargo run --release -- show --ferrocene-src=$ferrocene --instr-profile=$prof --report=$symbols $objs "$@"
