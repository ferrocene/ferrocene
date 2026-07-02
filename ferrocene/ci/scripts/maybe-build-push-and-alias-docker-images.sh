#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail

IMAGE="${1:-}"
SRC="${2:-}"
ALIAS="${3:-}"

if [[ -z "$IMAGE" ]] || [[ -z "$SRC" ]] || [[ -z "$ALIAS" ]]; then
    echo "USAGE: $0 IMAGE SRC ALIAS"
    exit 1
fi

account="$(aws sts get-caller-identity --region "${ECR_REGION}" | jq -r .Account)"
registry="${account}.dkr.ecr.${ECR_REGION}.amazonaws.com"

function get_manifest() {
    aws ecr batch-get-image --repository-name "$ECR_REPOSITORY" --image-ids imageTag="$1" --output text --query 'images[].imageManifest'
}

function get_manifest_digest() {
    jq -r .config.digest
}

aws ecr get-login-password --region "${ECR_REGION}" \
    | docker login --username AWS --password-stdin "${registry}"

manifest=""
if manifest="$(get_manifest "$SRC")"; then
    echo "image $SRC doesn't exist, building..."
    IMAGE_NAME="$IMAGE" \
    IMAGE_TAG="$SRC" \
    ferrocene/ci/scripts/build-and-push-docker-image.sh
    manifest="$(get_manifest "$SRC")"
else
    alias_manifest="$(get_manifest "$ALIAS")"
    src_digest="$(echo "$manifest" | get_manifest_digest)"
    alias_digest="$(echo "$alias_manifest" | get_manifest_digest)"
    if [[ "$src_digest" == "$alias_digest" ]]; then
        echo "alias $ALIAS already points to $SRC"
        exit 0
    fi
fi

aws ecr put-image --repository-name "$ECR_REPOSITORY" --image-tag "$ALIAS" --image-manifest "$manifest"
