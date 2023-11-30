#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

# Ensure we're in the repository root regardless of the current directory.
cd "$(dirname "$0")/../.."

# Generate `config.toml`.
ferrocene/ci/configure.sh

# Use a Cargo home inside the build directory instead of the default ~/.cargo,
# including it in the CircleCI workspace. This has multiple advantages:
#
# * The contents of the directory are copied to the following jobs, preventing
#   the index from being cloned over and over again.
#
# * The crate sources in ~/.cargo/registry/src/${registry}/${crate} are copied
#   to the following jobs. When Cargo extracts the dependency sources in that
#   directory it doensn't try to set a consistent modification time (mtime).
#   Copying the directory ensures we're using the same mtime across all jobs,
#   preventing spurious rebuilds if a build script uses `rerun-if-changed`
#   inside the sources directory.
export CARGO_HOME="$(pwd)/build/cargo-home"

# Give control to the actual CI script. $SCRIPT
exec bash -euo pipefail -c "${SCRIPT}"
