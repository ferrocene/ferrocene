#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Configures Rust's build system with the Ferrocene-specific configuration.
#
# In order to run the script, you might need to set some environment variables
# to provide the values of some keys. We recommend to read through the script
# for the list of variables you need to set and the explanation of them.
#
# Note that some of the configuration created by this file interacts with the
# Ferrous Systems production infrastructure, and will not work outside of it.
# If you don't have access to that infrastructure, you can invoke the script
# with the OUTSIDE_FERROUS=1 environment variable.

set -euo pipefail
IFS=$'\n\t'

add() {
    # Add each argument split by a `\t` instead of a space. This is needed to
    # support flags with spaces in them.
    while [[ $# -gt 0 ]]; do
        RUST_CONFIGURE_ARGS="${RUST_CONFIGURE_ARGS-}"$'\t'"$1"
        shift
    done
}

if [[ -z "${CI+x}" ]]; then
    echo "error: this doesn't look like it's being executed in CI."
    echo
    echo "The purpose of this script is to configure the build system for CI"
    echo "use, and it's not recommended for local development. If you are"
    echo "developing Ferrocene, please check the Internal Procedures:"
    echo
    echo "    https://public-docs.ferrocene.dev/main/qualification/internal-procedures/setup-local-env.html"
    echo
    echo "If you're trying to simulate CI to debug an issue locally, set the"
    echo '$CI' "environment variable when invoking this script to suppress"
    echo "this error message."
    exit 1
fi

# Helper function to check whether we should include things that only Ferrous
# Systems has access to in the configuration.
is_internal() {
    if [[ -z "${OUTSIDE_FERROUS+x}" ]]; then
        return 0
    else
        return 1
    fi
}

##################################################################
#                                                                #
#   Configuration items not affecting the resulting toolchain.   #
#                                                                #
##################################################################

# Enable the generation of build metrics, which provide extra information on
# the duration of each step of the build. This is then used by scripts and
# tools to analyze how time is spent on CI.
add --set build.metrics

# Prevent `./x.py` from managing submodules, as those are cloned and managed
# already by scripts in the CI configuration.
add --disable-manage-submodules

##############################################################################
#                                                                            #
#   Configuration items changing the resulting toolchain WITHOUT affecting   #
#   its functionality, reliability or security.                              #
#                                                                            #
##############################################################################

# Statically link Cargo's native dependencies.
#
# If this configuration is missing the resulting `cargo` binary will be
# different, and might require native dependencies to be installed on the
# user's systems.
add --enable-cargo-native-static

# Statically link libstdc++ in the resulting LLVM.
#
# If this confiugration is missing the resulting LLVM will be different, and
# might require libstdc++ to be installed on the user's system.
add --enable-llvm-static-stdcpp

# Produce XZ-compressed tarballs when building dist artifacts.
#
# If this configuration is missing or set to a different value the resulting
# dist tarballs will be compressed with a different algorithm.
add --dist-compression-formats=xz

# Increase the compression ratio when building dist artifacts.
#
# If this configuration is missing or set to a different value, a different
# compression ration will be used when creating tarballs.
add --set dist.compression-profile=balanced

# Remap debuginfo to `/rustc/{commit-sha}`.
#
# If this configuration is missing, the directory structure of the build
# machine will leak into the resulting binaries, preventing reproducibility.
add --set rust.remap-debuginfo

# Include the lines table in the standard library's debuginfo.
#
# If this configuration is missing backtraces will not include file and line
# information for the standard library, making it harder for end users to debug
# the cause of a panic.
add --debuginfo-level-std=1

# Disable debug logging in the compiler, shrinking the binary size.
#
# If this configuration is missing all debug logging will be included in the
# compiler, which can then be shown with the RUSTC_LOG environment variable.
add --set rust.debug-logging=false

# Switches the compiler from the system allocator to jemalloc. Jemalloc is more
# performant compared to the system allocator for the compiler workloads,
# speeding up the compilation process.
#
# If this configuration is missing the system allocator will be used, slowing
# down the compiler.
#
# On Windows, Jemalloc is not tested, and manual testing suggests it is not
# supported.
if [[ "${FERROCENE_HOST}" != "x86_64-pc-windows-msvc" ]]; then
    add --set rust.jemalloc
fi

# Adds a custom string to the output of `rustc --version` to properly mark this
# is not the upstream compiler.
#
# If this configuration is missing or changed the output of `rustc --version`
# will change accordingly.
add --release-description="Ferrocene by Ferrous Systems"

##############################################################################
#                                                                            #
#   Configuration items changing the resulting toolchain AFFECTING its       #
#   functionality, reliability or security! NEVER change these items.        #
#                                                                            #
##############################################################################

# Set the target used for the build itself (build system, initial compiler
# stages, etc). This depends on the OS used in CI.
#
# If this configuration is missing or changed, the wrong build platform will be
# used by the build system.
if [[ -x "${FERROCENE_BUILD_HOST+x}" ]]; then
    add "--build=${FERROCENE_BUILD_HOST}"
fi

# The Rust build system defaults to calling `cc` on Windows, which does not exist
if [[ is_internal && "${FERROCENE_BUILD_HOST:-}" = "x86_64-pc-windows-msvc" ]]; then
    add --set target.aarch64-unknown-none.cc=clang
    add --set target.aarch64-unknown-none.cxx=clang
    add --set target.aarch64-unknown-none.ar=llvm-ar

    add --set target.thumbv6m-none-eabi.cc=clang
    add --set target.thumbv6m-none-eabi.cxx=clang
    add --set target.thumbv6m-none-eabi.ar=llvm-ar

    add --set target.thumbv7em-none-eabi.cc=clang
    add --set target.thumbv7em-none-eabi.cxx=clang
    add --set target.thumbv7em-none-eabi.ar=llvm-ar

    add --set target.thumbv7em-none-eabihf.cc=clang
    add --set target.thumbv7em-none-eabihf.cxx=clang
    add --set target.thumbv7em-none-eabihf.ar=llvm-ar

    add --set target.wasm32-unknown-unknown.cc=clang
    add --set target.wasm32-unknown-unknown.cxx=clang
    add --set target.wasm32-unknown-unknown.ar=lld-ar
fi

if [[ is_internal ]]; then
    # QNX toolchains aren't automatically inferred, set them explicitly.
    #
    # Assumes `qnxsdp-env.sh` has been sourced or the binaries are otherwise
    # already on path
    add --set target.aarch64-unknown-nto-qnx710.cc=qcc
    add --set target.aarch64-unknown-nto-qnx710.cxx=q++
    add --set target.aarch64-unknown-nto-qnx710.ar=ntoaarch64-ar
    add --set target.aarch64-unknown-nto-qnx710.profiler=false # Build failures were noted if this is enabled.
    add --set target.x86_64-pc-nto-qnx710.cc=qcc
    add --set target.x86_64-pc-nto-qnx710.cxx=q++
    add --set target.x86_64-pc-nto-qnx710.ar=ntox86_64-ar
    add --set target.x86_64-pc-nto-qnx710.profiler=false # Build failures were noted if this is enabled.
fi

# Set the host platform to build. The environment variable is set from the CI
# configuration (see the .circleci directory).
#
# If this configuration is missing or changed the wrong host platform might be
# used for compilation.
add --host="${FERROCENE_HOST}"

# Set the targets to build. The environment variable is set from the CI
# configuration (see the .circleci directory), and if the variable is not set
# the host target will be used.
#
# If this configuration is changed the wrong target platforms might be used for
# compilation.
if [[ -z "${FERROCENE_TARGETS+x}" ]]; then
    add --target="${FERROCENE_HOST}"
else
    add --target="${FERROCENE_TARGETS}"
fi

# Set a custom LLVM installation root rather than using the copy of LLVM
# managed by upstream. The root must contain at least all the required LLVM
# shared libraries, and the llvm-config binary.
#
# If this configuration is changed, Ferrocene will be built with a different
# LLVM, and the behavior might change.
if [[ -n "${FERROCENE_CUSTOM_LLVM+x}" ]]; then
    add --llvm-root="${FERROCENE_CUSTOM_LLVM}"
fi

# Prevent `cargo` from updating the `Cargo.lock` file if the contents of the
# file are out of date, failing the build instead.
#
# If this configuration is missing the compiler could be built with different
# dependencies rather than the pinned ones. Never remove this flag.
add --enable-locked-deps

# Use a single codegen unit when compiling the standard library.
#
# Rust upstream had issues in the past [1] when compiling the standard library
# with more than 1 codegen unit. Compiling with more codegen units also
# prevents some optimizations. Never remove this flag due to the risk of the
# standard library failing to build correctly.
#
# [1] https://github.com/rust-lang/rust/issues/83600
add --set rust.codegen-units-std=1

# Enable LLVM assertions in the resulting compiler.
#
# If this configuration is missing LLVM assertions will be disabled, which
# could result in compiler bugs or miscompilations not being detected. Never
# remove this flag.
add --enable-llvm-assertions

# Enable debug assertions in the resulting compiler.
#
# If this configuration is missing Rust's debug assertions will be disabled,
# which could result in compiler bugs or miscompilations not being detected.
# Never remove this flag.
add --enable-debug-assertions

# Enable LLVM IR verification. Verification has a small compiler performance
# hit, but has a chance of catching compiler bugs.
#
# If this configuration is missing LLVM IR verification will be disabled.
# Never remove this flag.
add --set rust.verify-llvm-ir

# Enable support for LLVM sanitizers inside the compiler.
#
# If this configuration is missing it won't be possible to use sanitizers.
add --enable-sanitizers

# Enable only the LLVM codegen backend, preventing other codegen backends from
# being built and shipped.
#
# If this configuration is missing we'll build all codegen backends built by upstream,
# which in the future *might* include GCC.
add --codegen-backends=llvm

# Enable the extended build, which produces dist artifacts for tools in
# addition to just the compiler and the documentation.
#
# If this configuration is missing, the full distribution won't be built.
add --enable-extended

# Choose which tools must be built and distributed.
#
# If this configuration is missing or changed, the wrong set of tools will be
# built, and the build could fail if some tool is not tested and fails.
#
# NOTE: If you add a new tool here, make sure to also change
# `ferrocene/packages.toml` to include it in new releases.
add --tools=rustdoc,cargo,llvm-tools,rustfmt,rust-analyzer,clippy

# Build and enable the profiler runtime.
#
# If this configuration is missing, profile guided optimizations and code
# coverage will not be supported by the resulting compiler.
add --enable-profiler

# Disable the profiler runtime for WASM. The profiler runtime depends on libc,
# which is not available on WASM bare metal.
#
# If this configuration is missing, building the WASM target will fail.
add --set target.wasm32-unknown-unknown.profiler=false

# Build and include LLD in the resulting compiler package.
#
# If this configuration is missing or changed, LLD will not be included.
add --enable-lld

# Set the release channel for this branch. The channel is read from the
# `src/ci/channel` file to easily allow tools and automations to know and
# update the current channel.
#
# Changing the release channel to `nightly` enables unstable features, and it
# should not be done for any build shipped to customers.
release_channel="$(cat src/ci/channel)"
add "--release-channel=${release_channel}"

# Set the Ferrocene channel for this branch. The channel is read from
# `ferrocene/ci/channel` file to easily allow tools and automations to know and
# update the current channel.
add --set "ferrocene.channel=$(cat ferrocene/ci/channel)"

# Run the traceability matrix tool in CI mode, producing the correct links.
#
# If this configuration is missing the traceability matrix might not be
# properly enforced.
add --set ferrocene.traceability-matrix-mode=ci

# Sign packages generated by CI with the packages key.
#
# If this configuration is missing the packages generated by CI will not be
# signed, and will not be compatible with criticalup.
if is_internal; then
    add --set ferrocene.tarball-signing-kms-key-arn="arn:aws:kms:us-east-1:886866542769:key/cfbd0673-04d8-4368-b09f-56998ede9b96"
fi

# Download the correct version of the OxidOS source code from our mirrors bucket.
#
# If this configuration is missing, building OxidOS will fail as the source
# code will not be available. OxidOS is proprietary, so we cannot fetch the
# source code automatically from a public repository.
#
# This will not work for non-employees of Ferrous Systems
if is_internal; then
    add --set ferrocene.oxidos-src="s3://ferrocene-ci-mirrors/manual/oxidos/oxidos-source-2023-09-21.tar.xz"
fi

# Include the technical report from the assessor in the documentation.
#
# If this is not provided, the report will not be included in the generated
# documentation. This should only be set in stable, qualified releases.
#if is_internal; then
#    add --set ferrocene.technical-report-url="s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-technical-report.pdf"
#fi

# When building Ferrocene outside of Ferrous Systems, folks will not have
# access to the document signature files stored in AWS. In that case, configure
# the build system to ignore document signatures.
if ! is_internal; then
    add --set ferrocene.document-signatures=disabled
fi

###############################################
#                                             #
#  Write the configuration to `config.toml`   #
#                                             #
###############################################

./configure ${RUST_CONFIGURE_ARGS}
