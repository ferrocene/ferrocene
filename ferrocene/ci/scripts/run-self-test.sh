#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

COMMIT="${COMMIT:-$(git rev-parse HEAD)}"
SKIP_CLEANUP="${SKIP_CLEANUP:-}"

ARTIFACTS_BUCKET="${ARTIFACTS_BUCKET:-ferrocene-ci-artifacts}"
ARTIFACTS_PREFIX="${ARTIFACTS_PREFIX:-ferrocene/dist/}"
PREFIX="${ARTIFACTS_PREFIX}${COMMIT}"

TAR="tar"
FERROCENE_SELF_TEST="ferrocene-self-test"
# Ensure we use GNU tar on Windows, bsdtar will not handle links well.
# bsdtar (the default tar.exe) does not handle symlinks properly,
# and will get 'stuck' in a cycle while unpacking and packing.
# GNU Tar does not have this issue.
if [[ "${OSTYPE}" = "msys" ]]; then
    TAR="/c/Program Files/Git/usr/bin/tar.exe"
    FERROCENE_SELF_TEST="ferrocene-self-test.exe"
fi

TEMPDIR="$(mktemp -d -p .)"

if [[ -z "$SKIP_CLEANUP" ]]; then
    cleanup() {
        rm -rf "${TEMPDIR}"
    }
    trap cleanup EXIT
fi

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
    aws s3 cp "s3://${ARTIFACTS_BUCKET}/${PREFIX}/${package}-${target}-${version}.tar.xz" "${TEMPDIR}/archives/${package}-${target}-${version}.tar.xz"
}

mkdir -p "${TEMPDIR}/archives"
download ferrocene-self-test "${FERROCENE_HOST}"
download rustc "${FERROCENE_HOST}"
download cargo "${FERROCENE_HOST}"
download llvm-tools "${FERROCENE_HOST}" # This appears to be only required for Windows

IFS=',' read -ra targets <<< "${FERROCENE_TARGETS:-}"
targets+=("${FERROCENE_HOST}")
for target in ${targets[@]:-}; do
    download rust-std "${target}"
done

mkdir -p "${TEMPDIR}/sysroot"
for archive in ${TEMPDIR}/archives/*; do
    echo "===> installing $(basename ${archive})"

    ${TAR} -C "${TEMPDIR}/sysroot" -xf "${archive}"
done

echo "===> running the self-test tool"
"${TEMPDIR}/sysroot/bin/${FERROCENE_SELF_TEST}"
