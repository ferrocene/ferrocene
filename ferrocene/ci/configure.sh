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

if [[ -z "${CI+x}" ]]; then
    echo "error: this doesn't look like it's being executed in CI."
    echo
    echo "The purpose of this script is to configure the build system for CI"
    echo "use, and it's not recommended for local development. If you are"
    echo "developing Ferrocene, please check the Internal Procedures:"
    echo
    echo "    https://public-docs.ferrocene.dev/main/qualification/internal-procedures/setup-local-env.html"
    echo
    echo "If you're trying to simulate CI to debug an issue locally, change the"
    echo "profile in your bootstrap.toml to 'ferrocene-dist'."
    exit 1
fi

if [[ -n "${OUTSIDE_FERROUS+x}" ]]; then
    echo "error: this script is for Ferrous Systems use only."
    echo "note: caused by the OUSIDE_FERROUS environment variable"
    echo
    echo "In the past, the ferrocene/ci/configure.sh script contained both the"
    echo "configuration of Ferrocene itself, and the configuration specific to"
    echo "Ferrous System's CI setup. The OUTSIDE_FERROUS environment variable"
    echo "was created to let the script only configure Ferrocene."
    echo
    echo "Now, the Ferrocene configuration lives in the 'ferrocene-dist'"
    echo "bootstrap profile, and this script contains only the Ferrous Systems"
    echo "specific configuration. If you want the generic configuration, do not"
    echo "run this script, and instead run (with your extra arguments):"
    echo
    echo "    ./configure --set profile=ferrocene-dist " '$your_arguments_here'
    echo
    exit 1
fi

configure_args=()
add() {
    while [[ $# > 0 ]]; do
        configure_args+=("$1")
        shift
    done
}

# On unprivileged CI we use a downloaded LLVM, meaning we cannot set options like
# rust.lld = true, so don't enable the `ferrocene-dist` profile
if [[ -n "${FERROCENE_UNPRIVILEGED_CI+x}" ]]; then
    echo "Using unprivileged CI, not setting profile to \`ferrocene-dist\`"
else
    # Load the generic configuration from our dist profile.
    add --set profile=ferrocene-dist
fi

# Prevent `./x.py` from managing submodules, as those are cloned and managed
# already by scripts in the CI configuration.
add --disable-manage-submodules

# In our setup we generate coverage reports in the docs job, not in the tests jobs.
add --set ferrocene.generate-coverage-report-after-tests=false

# Set the target used for the build itself (build system, initial compiler
# stages, etc). This depends on the OS used in CI.
#
# If this configuration is missing or changed, the wrong build platform will be
# used by the build system.
if [[ -x "${FERROCENE_BUILD_HOST+x}" ]]; then
    add "--build=${FERROCENE_BUILD_HOST}"
fi

# The Rust build system defaults to calling `cc` on Windows, which does not exist
if [[ "${FERROCENE_BUILD_HOST:-}" = "x86_64-pc-windows-msvc" ]]; then
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

# QNX toolchains aren't automatically inferred, set them explicitly.
#
# Assumes `qnxsdp-env.sh` has been sourced or the binaries are otherwise
# already on path
add --set target.aarch64-unknown-nto-qnx710.cc=qcc
add --set target.aarch64-unknown-nto-qnx710.cxx=q++
add --set target.aarch64-unknown-nto-qnx710.ar=ntoaarch64-ar
add --set target.x86_64-pc-nto-qnx710.cc=qcc
add --set target.x86_64-pc-nto-qnx710.cxx=q++
add --set target.x86_64-pc-nto-qnx710.ar=ntox86_64-ar
add --set target.x86_64-pc-nto-qnx710.profiler=false # Build failures were noted if this is enabled.

# these default to `cc` but require cross compilation
add --set 'target."aarch64-unknown-ferrocene.facade".cc=aarch64-linux-gnu-gcc'
add --set 'target."thumbv7em-ferrocene.facade-eabi".cc=arm-none-eabi-gcc'
add --set 'target."thumbv7em-ferrocene.facade-eabihf".cc=arm-none-eabi-gcc'
add --set 'target."aarch64-unknown-ferrocene.subset".cc=aarch64-linux-gnu-gcc'
add --set 'target."thumbv7em-ferrocene.subset-eabi".cc=arm-none-eabi-gcc'
add --set 'target."thumbv7em-ferrocene.subset-eabihf".cc=arm-none-eabi-gcc'
add --set 'target."aarch64-ferrocene-none".cc=aarch64-linux-gnu-gcc'
add --set 'target."thumbv7em-ferrocene-none-eabi".cc=arm-none-eabi-gcc'
add --set 'target."thumbv7em-ferrocene-none-eabihf".cc=arm-none-eabi-gcc'

# musl toolchains use the architecture, also we need to set the `musl-root`
add --set target.x86_64-unknown-linux-musl.musl-root=/usr/local/x86_64-linux-musl/
add --set target.x86_64-unknown-linux-musl.cc=x86_64-linux-musl-gcc
add --set target.aarch64-unknown-linux-musl.musl-root=/usr/local/aarch64-linux-musl/
add --set target.aarch64-unknown-linux-musl.cc=aarch64-linux-musl-gcc

# disable sanitizers (defaults to true in ferrocene-dist profile) on MUSL as
# LLVM does not support MUSL
add --set target.aarch64-unknown-linux-musl.sanitizers=false
add --set target.x86_64-unknown-linux-musl.sanitizers=false

# experiment to enable code coverage
add --set 'target."aarch64-unknown-ferrocene.facade".profiler=true'
add --set 'target."thumbv7em-ferrocene.facade-eabi".profiler=true'
add --set 'target."thumbv7em-ferrocene.facade-eabihf".profiler=true'

# Set the host platform to build. The environment variable is set from the CI
# configuration (see the .circleci directory).
#
# If this configuration is missing or changed the wrong host platform might be
# used for compilation.
add --host="${FERROCENE_HOST}"

if python_version=$(uv python find); then
    add --set "build.python=$python_version"
fi

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

# Set the directory to the test outcomes files, if CI provides it.
#
# If this configuration is missing or changed, the test outcomes use to render our reports and
# documentation will be incorrect.
if [[ -n "${FERROCENE_TEST_OUTCOMES_DIR+x}" ]]; then
    add --set ferrocene.test-outcomes=custom
    add --set "ferrocene.test-outcomes-dir=${FERROCENE_TEST_OUTCOMES_DIR}"
fi

# Set the directory to the coverage outcomes files, if CI provides it.
if [[ -n "${FERROCENE_COVERAGE_OUTCOMES_DIR+x}" ]]; then
    add --set ferrocene.coverage-outcomes=custom
    add --set "ferrocene.coverage-outcomes-dir=${FERROCENE_COVERAGE_OUTCOMES_DIR}"
fi

# Prevent `cargo` from updating the `Cargo.lock` file if the contents of the
# file are out of date, failing the build instead.
#
# If this configuration is missing the compiler could be built with different
# dependencies rather than the pinned ones. Never remove this flag.
add --enable-locked-deps

if [[ -n "${FERROCENE_UNPRIVILEGED_CI+x}" ]]; then
    # Disable document signatures if we are in an unprivileged CI environment.
    add --set ferrocene.document-signatures=disabled
else
    # Sign packages generated by CI with the packages key.
    #
    # If this configuration is missing the packages generated by CI will not be
    # signed, and will not be compatible with criticalup.
    add --set ferrocene.tarball-signing-kms-key-arn="arn:aws:kms:us-east-1:886866542769:key/cfbd0673-04d8-4368-b09f-56998ede9b96"

    # Download the correct version of the OxidOS source code from our mirrors bucket.
    #
    # If this configuration is missing, building OxidOS will fail as the source
    # code will not be available. OxidOS is proprietary, so we cannot fetch the
    # source code automatically from a public repository.
    #
    # This will not work for non-employees of Ferrous Systems
    add --set ferrocene.oxidos-src="s3://ferrocene-ci-mirrors/manual/oxidos/oxidos-source-2025-04-30.tar.xz"

    # Include the compiler technical report from the assessor in the documentation.
    #
    # If this is not provided, the report will not be included in the generated
    # documentation. This should only be set in stable, qualified releases.
    #add --set ferrocene.compiler-technical-report-url="s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-compiler-technical-report.pdf"

    # Include the core library technical report from the assessor in the documentation.
    #
    # If this is not provided, the report will not be included in the generated
    # documentation. This should only be set in stable, qualified releases.
    #add --set ferrocene.core-technical-report-url="s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-core-technical-report.pdf"
fi

./configure ${configure_args[@]}
