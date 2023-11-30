#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# This script generates all the build metadata files, and stores them in the
# dist directory. It's then the responsibility of another script to upload the
# metadata files to the S3 bucket.

set -euo pipefail
IFS=$'\n\t'

DIST_DIR="build/dist"

mkdir -p "${DIST_DIR}"

# Perform validation on the contents of the version field, to avoid generating
# artifacts that will break the release process.
if [[ "$(cat ferrocene/ci/channel)" = "rolling" ]]; then
    if [[ "$(cat ferrocene/version)" != "rolling" ]]; then
        echo "error: ferrocene/version must be 'rolling' when ferrocene/ci/channel is 'rolling'"
        exit 1
    fi
fi

# Whenever the contents of this JSON file change, even just adding new fields,
# make sure to increase the metadata version number and update publish-release
# accordingly. Note that new releases *won't* be made until publish-release and
# this use the same version number.
cat > "${DIST_DIR}/ferrocene-ci-metadata.json" <<EOF
{
    "metadata_version": 2,
    "rust_version": "$(cat src/version)",
    "rust_channel": "$(cat src/ci/channel)",
    "ferrocene_version": "$(cat ferrocene/version)",
    "ferrocene_channel": "$(cat ferrocene/ci/channel)",
    "sha1_full": "$(git rev-parse HEAD)",
    "sha1_short": "$(git rev-parse --short HEAD)"
}
EOF

# Add the list of packages to include in the release to the artifacts, so that
# publish-release knows what to expect for this commit.
cp ferrocene/packages.toml "${DIST_DIR}"
