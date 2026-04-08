#!/bin/sh

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -eu

case $(uname -m) in
    arm64|aarch64) host=aarch64;;
    x86_64) host=x86_64;;
    *)  echo "unknown platform $(uname -m)"
        exit 1
        ;;
esac

ensure() {
    prefix=$1
    tarfile=$2
    src=s3://ferrocene-ci-mirrors/manual/${prefix}/${tarfile}.tar.xz
    dst=ferrocene/ci/mirrors/${tarfile}.tar.xz
    if ! [ -e "$dst" ]; then
        echo "copying $src -> $dst"
        aws s3 cp "$src" "$dst"
    fi
}

for package in cmake-3.21.1 python-3.12.3 gdb-12.1; do
    ensure "$(echo $package | cut -d- -f1)" "${package}-${host}"
done
ensure musl "musl-cross-make-${host}-to-aarch64"
ensure musl "musl-cross-make-${host}-to-x86_64"
