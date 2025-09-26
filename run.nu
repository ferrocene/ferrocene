#!/usr/bin/env nu
use std/dirs

def main [
  --debug # Toggle debug
  --ferrocene-src: string # The path to ferrocene
] {
  let ferrocene = $ferrocene_src | default "/home/ci/project/ferrocene"
  let host = "x86_64-unknown-linux-gnu"
  let certified_host = "x86_64-unknown-ferrocene.certified"
  let prof = $"($ferrocene)/build/tmp/ferrocene-library-($host).profdata"
  let std_build = $"($ferrocene)/build/host/stage1-std/($host)/release/deps/"
  let symbols = $"($ferrocene)/build/host/stage1-std/($certified_host)/release/symbol-report.json"
  
  let std_obj = ls $std_build | where name =~ "libstd.*so" | get name | first
  let corebenches_bin = ls $std_build | where name =~ "corebenches" | get name | first
  let coretests_bin = ls $std_build | where name =~ "coretests" | get name | first
  let stdbenches_bin = ls $std_build | where name =~ "stdbenches" | get name | first
  
  dirs add $ferrocene
  let rev = (git log --pretty=format:'%H' -n 1)
  dirs drop

  mut extra_arg = []
  if $debug { $extra_arg = $extra_arg | append "--debug" }
  
  
  (cargo run --release -- show
    --instr-profile $prof
    --object $std_obj
    --object $corebenches_bin
    --object $coretests_bin
    --object $stdbenches_bin
    --report $symbols
    --path-equivalence $"/rustc/($rev),/home/ci/project/ferrocene"
    --ferrocene-src $ferrocene
    ...$extra_arg
  )
}
