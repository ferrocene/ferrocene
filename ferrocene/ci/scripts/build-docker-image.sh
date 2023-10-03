#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Build a Docker image used for CI and push it to AWS ECR The required
# environment variables are:
#
# * `IMAGE_NAME`: name of the image that has to be built
# * `IMAGE_TAG`: tag to assign to the pushed image
#

set -euo pipefail
IFS=$'\n\t'

REBUILD_IMAGES_OLDER_THAN="$(( 86400 * 7 ))"  # 7 days
ECR_REPOSITORY="ci-docker-images"
ECR_REGION="us-east-1"

# Generates ACCOUNT_ID.dkr.ecr.REGION.amazonaws.com/REPOSITORY:TAG
account="$(aws sts get-caller-identity --region "${ECR_REGION}" | jq -r .Account)"
registry="${account}.dkr.ecr.${ECR_REGION}.amazonaws.com"
image="${registry}/${ECR_REPOSITORY}:${IMAGE_TAG}"

# Authenticate with AWS ECR
aws ecr get-login-password --region "${ECR_REGION}" \
    | docker login --username AWS --password-stdin "${registry}"

# Build and push the image
docker build -t "${image}" "ferrocene/ci/docker-images/${IMAGE_NAME}"
docker push "${image}"
