#!/bin/sh

set -ex

cargo test --target $TARGET
if rustc --version | grep nightly; then
    cargo build --target $TARGET --manifest-path crates/as-if-std/Cargo.toml
fi
