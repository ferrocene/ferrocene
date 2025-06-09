#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# This script downloads all the build metrics for test jobs into the local
# filesystem, filtering for builders with "test" in the name.

set -euo pipefail
IFS=$'\n\t'

ARTIFACTS_BUCKET="${ARTIFACTS_BUCKET:-ferrocene-ci-artifacts}"
ARTIFACTS_PREFIX="${ARTIFACTS_PREFIX:-ferrocene/dist/}"
DEST="/tmp/coverage-outcomes"

aws s3 cp --recursive "s3://${ARTIFACTS_BUCKET}/${ARTIFACTS_PREFIX}${CIRCLE_SHA1}/coverage/" "${DEST}"
