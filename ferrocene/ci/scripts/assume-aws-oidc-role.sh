#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script authenticates with our AWS account using the OpenID Connect
# credentials provided by GitHub Actions, and assumes the role defined in the
# $AWS_ROLE_ARN environment variable.
#
# Authenticating with OpenID Connect allows authentication and authorization
# without having to store persistent credentials in CI, and without having to
# periodically rotate them. See GitHub's documentation for more information:
#
#     https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect
#

set -euo pipefail
IFS=$'\n\t'

set_env() {
    set +u
    if [ ! -z "${GITHUB_ENV}" ]; then
        echo "$1=$2" >> "${GITHUB_ENV}"
    fi
    set -u
    export "$1=$2"
}

# Load the JWT from GitHub Actions's token server.
curl -H "Authorization: bearer ${ACTIONS_ID_TOKEN_REQUEST_TOKEN}" "${ACTIONS_ID_TOKEN_REQUEST_URL}&audience=sts.amazonaws.com" | jq -r '.value' > /tmp/awsjwt
chmod 0400 /tmp/awsjwt

# Configure AWS CLI to use the JWT we just received.
set_env AWS_WEB_IDENTITY_TOKEN_FILE "/tmp/awsjwt"
set_env AWS_ROLE_SESSION_NAME "github-actions"
set_env AWS_DEFAULT_REGION "${AWS_DEFAULT_REGION:-eu-central-1}"
set_env AWS_ROLE_ARN "${AWS_ROLE_ARN}"

# Prevent AWS CLI from connecting to the metadata server, which causes problems
# on GitHub Actions (as it's not running on AWS).
set_env AWS_EC2_METADATA_DISABLED "true"

# Authenticate with AWS and get information about the current session.
aws sts get-caller-identity
