#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script builds a copy of LLVM if it's not already present in the cache,
# and uploads a tarball of the output to S3. The tarball will be used by
# subsequent jobs to avoid spending time compiling LLVM from scratch.
#
# To build the cache key the script generates the hash of multiple directories
# inside the repository. The hash considers unrelated directories, and could
# cause unwanted rebuilds, but it ensures we never use an LLVM built with a
# different set of flags or code.

set -euo pipefail
IFS=$'\n\t'

CACHE_BUCKET="ferrocene-ci-caches"
CACHE_PREFIX="prebuilt-llvm"

# Note that this *ignores* symlinks. If you need a binary that's actually a
# symlink please add to the list the binary the symlink points *to*.
KEEP_LLVM_BINARIES=(
    # Needed for the `llvm-tools` component
    "llvm-cov"
    "llvm-nm"
    "llvm-objcopy"
    "llvm-objdump"
    "llvm-profdata"
    "llvm-readobj"
    "llvm-size"
    "llvm-strip"
    "llvm-ar"
    "llvm-as"
    "llvm-dis"
    "llc"
    "opt"
    # Needed to link `rustc` with LLVM
    "llvm-config"
    # Needed by the Rust test suite
    "llvm-bcanalyzer"
    "llvm-dwarfdump"
    "FileCheck"
    # Needed to resolve symbols
    "llvm-dwp"
    # Needed to build LLD
    "llvm-tblgen"
)

EXE_SUFFIX=""
if [[ "${OSTYPE}" = "msys" ]]; then
    # Windows will postfix binaries with `.exe`
    EXE_SUFFIX=".exe"
fi

SHA_CMD=("sha256sum")
if ! [ -x "$(command -v ${SHA_CMD})" ]; then
    SHA_CMD=("shasum" "-a" "256")
fi

# Calculate a hash of the LLVM source code and all the files that could impact
# the LLVM build. This will be used as the cache key to avoid rebuilding LLVM
# from scratch every time.
get_llvm_cache_hash() {
    echo "For ${FERROCENE_HOST}" 1>&2
    file="$(mktemp)"

    ${SHA_CMD[@]} "$0" >> "${file}"
    ${SHA_CMD[@]} ferrocene/ci/configure.sh >> "${file}"
    ${SHA_CMD[@]} src/version >> "${file}"

    echo "git ls-files output" 1>&2
    git ls-files -z src/bootstrap ferrocene/ci/docker-images 1>&2
    # Apparently, git for windows doesn't understand when the `-z` flag of `git
    # ls-files` is passed after the paths, so we provide it before the list of
    # paths to list.
    echo "git ls-files | sort output" 1>&2
    git ls-files -z src/bootstrap ferrocene/ci/docker-images | sort --zero-terminated 1>&2
    echo "git ls-files | sort | xargs output" 1>&2
    git ls-files -z src/bootstrap ferrocene/ci/docker-images | sort --zero-terminated | xargs -0 ${SHA_CMD[@]}

    git ls-files -z src/bootstrap ferrocene/ci/docker-images | sort --zero-terminated | xargs -0 ${SHA_CMD[@]} >> "${file}"
    # Hashing all of the LLVM source code takes time. Instead we can simply get
    # the hash of the tree from git, saving time and achieving the same effect.
    git ls-tree HEAD src/llvm-project >> "${file}"

    cat "${file}" 1>&2
    if [[ "${OSTYPE}" = "msys" ]]; then
        # For some reason, Windows has different shasum output than Linux.
        # Lines are, inexplicably:
        # $CHECKSUM *$PATH
        # Instead of:
        # $CHECKSUM  $PATH
        # So, remove that. Also fix the line endings (\r)
        new_file="$(mktemp)"
        cat "${file}" | tr '*' ' ' | sed 's.\r$..' > "${new_file}"
        mv "${new_file}" "${file}"

        echo "After edits" 1>&2
        cat "${file}" 1>&2
    fi

    ${SHA_CMD[@]} "${file}" | awk '{print($1)}'
    rm -f "${file}"
}

# Build LLVM and generate a tarball we can cache with all the build artifacts.
build_llvm_tarball() {
    # Manually set the number of CPU cores to double the actually available
    # cores to build LLVM in a reasonable amount of time.
    #
    # Note that this uses a custom script to get the number of CPU cores, and
    # not a standard tool. This is because standard tools don't account for
    # Docker containers and return the number of cores of the *host* system,
    # while the tool inspects cgroups.
    ./x.py build src/llvm-project -j "${LLVM_BUILD_PARALLELISM}"

    # The llvm/build directory contains a *copy* of all the binaries, plus the
    # intermediate object files and other build artifacts we don't need. To
    # save space in the cached tarball remove it.
    #
    # On Windows, we skip this pruning since it *does* need intermediate
    # object files. (Notably, `llvm/Config/llvm-config.h` and many lib objects)
    if [[ "${OSTYPE}" != "msys" ]]; then
        rm -rf "build/${FERROCENE_HOST}/llvm/build"
        
        # Rustbuild is looking in `llvm/build/bin` instead of `bin` when checking
        # for an existing `llvm-config` binary. Create a symlink to make sure it
        # can still detect the existing build.
        
        mkdir -p "build/${FERROCENE_HOST}/llvm/build"
        ln -s ../bin "build/${FERROCENE_HOST}/llvm/build/bin"
    fi

    # The LLVM distribution as of 2021-08-23 contains more than 1GB of
    # binaries, but we only need a small subset of them. This "deletes" the
    # binaries we don't need to avoid caching them.
    #
    # Note that this does not *actually* remove the files, it replaces them
    # with a non-executable file containing a note that the file is deleted.
    # This is because LLVM's CMake configuration explicitly checks for all the
    # binaries to be present, and if the binaries are straight up missing
    # building anything depending on LLVM (like the sanitizers) will fail.
    for file in build/"${FERROCENE_HOST}"/llvm/bin/*; do
        # Ignore symlinks, to avoid `foo` being removed if `foo` is needed but
        # a symlink called `bar` pointing to `foo` is not needed.
        if [[ -L "${file}" ]]; then
            continue
        fi

        name="$(basename "${file}")"
        keep="no"
        
        for wanted in "${KEEP_LLVM_BINARIES[@]}"; do
            for wanted in "${KEEP_LLVM_BINARIES[@]}"; do
                if [[ "${name}" == "${wanted}${EXE_SUFFIX}" ]]; then
                    keep="yes"
                    break
                fi
            done
        done
        if [[ "${keep}" == "no" ]]; then
            chmod -x "${file}"
            echo "#!/usr/bin/env sh" > "${file}"
            echo "echo 'File soft-removed by ferrocene/ci/scripts/build-and-cache-llvm.sh'" >> "${file}"
            echo "exit 1" >> "${file}"
        fi
    done

    # Call `zstd` separately to be able to use all cores available (`-T0`) and
    # the lowest compression level possible, to speed the compression as much
    # as possible.
    #
    # On Windows we have to pass `-f -`, otherwise tar will write to \\.\tape0
    # rather than stdout by default.
    tar -cf- "build/${FERROCENE_HOST}/llvm" | zstd -1 -T0 > /tmp/llvm-cache.tar.zst
}

usage() {
    echo "usage: $0 <command>"
    echo
    echo "available commands:"
    echo "- prepare: build and cache LLVM"
    echo "- download: download the cached LLVM"
    echo "- s3-url: get the S3 URL of the LLVM cache"
}

cache_hash="$(get_llvm_cache_hash)"
cache_file="${CACHE_PREFIX}/${FERROCENE_HOST}-${cache_hash}.tar.zst"
s3_url="s3://${CACHE_BUCKET}/${cache_file}"

if [[ "$#" -ne 1 ]]; then
    usage 1>&2
    exit 1
fi
case "$1" in
    prepare)
        echo "building and caching LLVM to ${s3_url}"
        build_llvm_tarball
        aws s3 cp /tmp/llvm-cache.tar.zst "${s3_url}"
        echo "cached LLVM cache at ${s3_url}"
        ;;
    download)
        echo "restoring LLVM cache from ${s3_url}"
        # On Windows we have to pass `-f -`, otherwise tar will write to \\.\tape0
        # rather than stdout by default.
        aws s3 cp "${s3_url}" - | zstd --decompress --stdout | tar -xf-
        echo "restored LLVM cache from ${s3_url}"
        ;;
    s3-url)
        echo "${FERROCENE_HOST} s3 url is ${s3_url}" 1>&2
        echo "${s3_url}"
        ;;
    *)
        usage 1>&2
        exit 1
        ;;
esac
