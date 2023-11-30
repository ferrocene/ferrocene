#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# This file records the current commit SHA as the head of the current branch in
# the artifacts S3 bucket. This is used by other tooling accessing the release
# artifacts (like ferrocene/publish-release) to know what the head commit is
# without having to gain read access to ferrocene/ferrocene.

set -euo pipefail
IFS=$'\n\t'

REFS_BUCKET="ferrocene-ci-artifacts"
REFS_PREFIX="ferrocene/refs/"

tempfile="$(mktemp)"
git rev-parse HEAD > "${tempfile}"
aws s3 cp "${tempfile}" "s3://${REFS_BUCKET}/${REFS_PREFIX}$(git rev-parse --abbrev-ref HEAD)"
