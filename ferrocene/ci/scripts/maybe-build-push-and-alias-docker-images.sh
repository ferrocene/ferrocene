#!/usr/bin/env bash
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
    aws ecr batch-get-image \
        --repository-name "$ECR_REPOSITORY" \
        --image-ids imageTag="$1" \
        --output text \
        --query 'images[].imageManifest' \
        || return 1
}

function get_manifest_digest() {
    echo "$1" | jq -r .config.digest
}

aws ecr get-login-password --region "${ECR_REGION}" \
    | docker login --username AWS --password-stdin "${registry}"

src_manifest="$(get_manifest "$SRC")"
src_digest="$(get_manifest_digest "$src_manifest")"
if [[ -n "$src_digest" ]]; then
    echo "image $SRC exists"
    alias_manifest="$(get_manifest "$ALIAS")"
    alias_digest="$(get_manifest_digest "$alias_manifest")"
    if [[ "$src_digest" == "$alias_digest" ]]; then
        echo "alias $ALIAS already points to $SRC"
        exit 0
    fi
else
    echo "image $SRC doesn't exist, building..."
    IMAGE_NAME="$IMAGE" \
    IMAGE_TAG="$SRC" \
    ferrocene/ci/scripts/build-and-push-docker-image.sh
    src_manifest="$(get_manifest "$SRC")"
fi

echo "aliasing $ALIAS to $SRC"
aws ecr put-image --repository-name "$ECR_REPOSITORY" --image-tag "$ALIAS" --image-manifest "$src_manifest"
