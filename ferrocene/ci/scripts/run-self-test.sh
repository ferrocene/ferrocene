#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

BUCKET="ferrocene-ci-artifacts"
PREFIX="ferrocene/dist/$(git rev-parse HEAD)"

root="$(mktemp -d)"
cleanup() {
    rm -rf "${root}"
}
trap cleanup EXIT

case "$(cat ferrocene/ci/channel)" in
    stable)
        version="$(cat ferrocene/version)"
        ;;
    beta)
    rolling)
        version="$(git rev-parse --short=9 HEAD)"
        ;;
    *)
        echo "error: unexpected content of ferrocene/ci/channel"
        exit 1
esac

list_targets() {
    aws s3api list-objects-v2 --bucket "${BUCKET}" --prefix "${PREFIX}/" --delimiter / --query 'Contents[].Key' --output text \
        | xargs basename -a \
        | sed -n "s/^rust-std-\(.*\)-${version}\.tar.xz$/\1/p"
}

download() {
    package="$1"
    target="$2"

    echo "===> downloading ${package} for ${target}"
    aws s3 cp "s3://${BUCKET}/${PREFIX}/${package}-${target}-${version}.tar.xz" "${root}/archives/${package}-${target}-${version}.tar.xz"
}

mkdir -p "${root}/archives"
download ferrocene-self-test "${FERROCENE_HOST}"
download rustc "${FERROCENE_HOST}"
download cargo "${FERROCENE_HOST}"
for target in $(list_targets); do
    download rust-std "${target}"
done

mkdir -p "${root}/sysroot"
for archive in ${root}/archives/*; do
    echo "===> installing $(basename ${archive})"

    tar -C "${root}/sysroot" -xf "${archive}"
done

echo "===> running the self-test tool"
"${root}/sysroot/bin/ferrocene-self-test"
