#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

# Ensure we're in the repository root regardless of the current directory.
cd "$(dirname "$0")/../.."

# std tests spawn whoami for simple process testing, so make sure the command can succesfully run
if ! whoami >/dev/null; then
  echo "error: the whoami command failed, you probably need to create a user with uid $(id -u) and gid $(id -g)"
  exit 1
fi

# Preserve the existing configuration file.
#
# This also prevents the build from failing if the script was called multiple
# times as part of the same CI job: rust-lang/rust#110123 changed configure.py
# to error out if an existing configuration file would be overridden, so we
# have to move the old file out of the way to proceed.
if [[ -f config.toml ]]; then
    dest="build/config-backup.toml"
    mv config.toml "${dest}"
    echo "note: existing configuration file moved to ${dest}" >&2
    echo
fi

echo "Configuring"
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

echo "Running:"
echo "${SCRIPT}"

# Give control to the actual CI script. $SCRIPT
exec bash -euo pipefail -c "${SCRIPT}"
