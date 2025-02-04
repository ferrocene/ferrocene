#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script downloads the publish-release binary, prebuilt by CI. Since the
# binary is stored in a different AWS account, this script also assumes the role
# to be able to download it.
#
# This script then executes the binary with the provided arguments.

set -euo pipefail
IFS=$'\n\t'

BIN_S3_REGION="us-east-1"
BIN_S3_PATH="s3://ferrocene-ci-artifacts/publish-release/publish-release-x86_64-unknown-linux-gnu"
BIN_DEST="/tmp/publish-release"

# Download the publish-release binary from S3. The binary is stored in the CI
# account, not the primary account, so we have to assume the artifacts role to
# be able to download the file.
assume_role="$(aws sts assume-role --role-arn "${PUBLISHRELEASE_ARTIFACTS_ROLE}" --role-session-name "github-actions")"
env \
    AWS_ACCESS_KEY_ID="$(echo "${assume_role}" | jq -r ".Credentials.AccessKeyId")" \
    AWS_SECRET_ACCESS_KEY="$(echo "${assume_role}" | jq -r ".Credentials.SecretAccessKey")" \
    AWS_SESSION_TOKEN="$(echo "${assume_role}" | jq -r ".Credentials.SessionToken")" \
    aws --region "${BIN_S3_REGION}" s3 cp --no-progress "${BIN_S3_PATH}" "${BIN_DEST}"
chmod +x "${BIN_DEST}"

# Execute the release.
"${BIN_DEST}" "$@"
