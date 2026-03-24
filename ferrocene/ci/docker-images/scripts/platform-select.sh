#!/usr/bin/env sh
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

case "$TARGETPLATFORM" in
  linux/amd64)
    RUST_TARGET=x86_64-unknown-linux-gnu
    MERGIRAF_HASH=91daeb9925d616d19a1784fcb973514ee8ea70ef830baa49aed168ab5ad09b77
    ;;
  linux/arm64)
    RUST_TARGET=aarch64-unknown-linux-gnu
    MERGIRAF_HASH=42a4fa6f05ad07a06f3bdac882c2214991a22e161f154764bda5d985b2afc743
    ;;
  *)
    echo "Unsupported platform '$TARGETPLATFORM'"
    exit 1
    ;;
esac

case "$1" in
  target) echo "$RUST_TARGET" ;;
  mergiraf-hash) echo "$MERGIRAF_HASH" ;;
  *)
    echo "unknown platform config '$1'"
    exit 2
    ;;
esac
