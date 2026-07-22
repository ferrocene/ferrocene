#!/bin/bash

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
    tarfile=$1
    prefix=$(echo $tarfile | cut -d- -f1)
    src=s3://ferrocene-ci-mirrors/manual/${prefix}/${tarfile}
    dst=ferrocene/ci/mirrors/${tarfile}
    if ! [ -e "$dst" ]; then
        echo "copying $src -> $dst"
        aws s3 cp "$src" "$dst"
    fi
}

while read -r LINE || [ -n "${LINE}" ]; do
    hash=$(echo "${LINE}" | cut -f 1 -d " ")
    package=$(echo "${LINE}" | cut -f 2 -d " ")
    # we know that the host arch of the package is always the first occurence, the
    # target arch is optional and only occurs for the musl-cross package
    host_arch=$(echo $LINE | awk '{ match($0, /(aarch64|x86_64)/); print substr($0, RSTART, RLENGTH);}')

    if [ "$host" = "$host_arch" ]; then
        echo "Downloading package ${package}"
        ensure $package
    else
        echo "Skipping package ${package}, wrong host arch (${host_arch})"
    fi
done < ferrocene/ci/mirrors/hashes.txt