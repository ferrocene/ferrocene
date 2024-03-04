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

# Calculate a hash of the LLVM source code and all the files that could impact
# the LLVM build. This will be used as the cache key to avoid rebuilding LLVM
# from scratch every time.
get_llvm_cache_hash() {
    file="$(mktemp)"

    sha256sum "$0" >> "${file}"
    sha256sum ferrocene/ci/configure.sh >> "${file}"
    sha256sum src/version >> "${file}"
    git ls-files src/bootstrap ferrocene/ci/docker-images -z | sort -z | xargs -0 sha256sum >> "${file}"
    # Hashing all of the LLVM source code takes time. Instead we can simply get
    # the hash of the tree from git, saving time and achieving the same effect.
    git ls-tree HEAD src/llvm-project >> "${file}"

    sha256sum "${file}" | awk '{print($1)}'
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
    rm -rf "build/${FERROCENE_HOST}/llvm/build"

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
            if [[ "${name}" == "${wanted}" ]]; then
                keep="yes"
                break
            fi
        done
        if [[ "${keep}" == "no" ]]; then
            chmod -x "${file}"
            echo "#!/bin/false" > "${file}"
            echo "File soft-removed by ferrocene/ci/scripts/build-and-cache-llvm.sh" >> "${file}"
        fi
    done

    # Rustbuild is looking in `llvm/build/bin` instead of `bin` when checking
    # for an existing `llvm-config` binary. Create a symlink to make sure it
    # can still detect the existing build.
    mkdir "build/${FERROCENE_HOST}/llvm/build"
    ln -s ../bin "build/${FERROCENE_HOST}/llvm/build/bin"

    # Call `zstd` separately to be able to use all cores available (`-T0`) and
    # the lowest compression level possible, to speed the compression as much
    # as possible.
    tar cv "build/${FERROCENE_HOST}/llvm" | zstd -1 -T0 > /tmp/llvm-cache.tar.zst
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
        echo "building and caching LLVM with hash ${cache_hash}"
        build_llvm_tarball
        aws s3 cp /tmp/llvm-cache.tar.zst "${s3_url}"
        ;;
    download)
        aws s3 cp "${s3_url}" - | unzstd --stdout | tar x
        echo "restored LLVM cache with hash ${cache_hash}"
        ;;
    s3-url)
        echo "${s3_url}"
        ;;
    *)
        usage 1>&2
        exit 1
        ;;
esac
