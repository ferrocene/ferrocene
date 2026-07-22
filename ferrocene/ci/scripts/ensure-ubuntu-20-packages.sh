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

for LINE in $(cat ferrocene/ci/mirrors/hashes.txt | tr -s " " | cut -f 2 -d " "); do
    package_file=$(echo "${LINE}" | cut -f 2)
    # we know that the host arch of the package is always the first occurence, the
    # target arch is optional and only occurs for the musl-cross package
    host_arch=$(echo $package_file | awk '{ match($0, /(aarch64|x86_64)/); print substr($0, RSTART, RLENGTH);}')

    if [ "$host" = "$host_arch" ]; then
        echo "Downloading package ${package_file}"
        ensure $package_file
    else
        echo "Skipping package ${package_file}, wrong host arch (${host_arch})"
    fi
done