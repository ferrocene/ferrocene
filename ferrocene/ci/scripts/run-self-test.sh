#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

COMMIT="$(git rev-parse HEAD)"

BUCKET="ferrocene-ci-artifacts"
PREFIX="ferrocene/dist/${COMMIT}"

root="$(mktemp -d)"
cleanup() {
    rm -rf "${root}"
}
trap cleanup EXIT

case "$(cat ferrocene/ci/channel)" in
    stable)
        version="$(cat ferrocene/version)"
        ;;
    beta|rolling)
        version="${COMMIT:0:9}"  # First 9 chars of the commit hash
        ;;
    *)
        echo "error: unexpected content of ferrocene/ci/channel"
        exit 1
esac

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

download rust-std "${FERROCENE_HOST}"
IFS=',' read -ra targets <<< "${FERROCENE_TARGETS:-}"
for target in ${targets[@]}; do
    download rust-std "${target}"
done

mkdir -p "${root}/sysroot"
for archive in ${root}/archives/*; do
    echo "===> installing $(basename ${archive})"

    tar -C "${root}/sysroot" -xf "${archive}"
done

echo "===> running the self-test tool"
"${root}/sysroot/bin/ferrocene-self-test"
