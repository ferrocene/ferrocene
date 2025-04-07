#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# This script uploads all the dist artifacts to our S3 bucket.

set -euo pipefail
IFS=$'\n\t'

DIST_DIR="build/dist"
ARTIFACTS_BUCKET="ferrocene-ci-artifacts"
ARTIFACTS_PREFIX="ferrocene/dist/"
BUILD_METRICS_FILE="build/metrics.json"

mkdir -p "${DIST_DIR}"

if [[ -f "${BUILD_METRICS_FILE}" ]]; then
    mkdir -p "${DIST_DIR}/build-metrics"
    DEST="${DIST_DIR}/build-metrics/${CIRCLE_JOB}.json"
    cp "${BUILD_METRICS_FILE}" "$DEST"
    echo "::notice title=Uploaded: $file::$DEST"
fi

DEST="s3://${ARTIFACTS_BUCKET}/${ARTIFACTS_PREFIX}${CIRCLE_SHA1}/"
aws s3 cp --recursive "${DIST_DIR}/" "$DEST"

for file in "${DIST_DIR}/*.tar.xz"; do
    echo "::notice title=Uploaded: $file::$DEST"
done
      