#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# This script uploads all the dist artifacts to our S3 bucket.

set -euo pipefail
IFS=$'\n\t'

DIST_DIR="build/dist"
ARTIFACTS_BUCKET="${ARTIFACTS_BUCKET:-ferrocene-ci-artifacts}"
ARTIFACTS_PREFIX="${ARTIFACTS_PREFIX:-ferrocene/dist}"
BUILD_METRICS_FILE="build/metrics.json"
COVERAGE_DIR="build/ferrocene/coverage"

mkdir -p "${DIST_DIR}"

if [[ -f "${BUILD_METRICS_FILE}" ]]; then
    mkdir -p "${DIST_DIR}/build-metrics"
    cp "${BUILD_METRICS_FILE}" "${DIST_DIR}/build-metrics/${CIRCLE_JOB}.json"
fi

if [[ -d "${COVERAGE_DIR}" ]]; then
    cp -r "${COVERAGE_DIR}" "${DIST_DIR}/coverage"
fi

DEST="s3://${ARTIFACTS_BUCKET}/${ARTIFACTS_PREFIX}/${CIRCLE_SHA1}/"
aws s3 cp --recursive "${DIST_DIR}/" "$DEST"

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    for file in ${DIST_DIR}/*.tar.xz; do
        echo "::notice title=Uploaded: $(basename $file)::$DEST"
    done
fi
      
