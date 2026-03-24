#!/usr/bin/env sh
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

case "$TARGETPLATFORM" in
  linux/amd64)
    RUST_TARGET=x86_64-unknown-linux-gnu
    ;;
  linux/arm64)
    RUST_TARGET=aarch64-unknown-linux-gnu
    ;;
  *)
    echo "Unsupported platform '$TARGETPLATFORM'"
    exit 1
    ;;
esac

case "$1" in
  target) echo "$RUST_TARGET" ;;
  *)
    echo "unknown platform config '$1'"
    exit 2
    ;;
esac
