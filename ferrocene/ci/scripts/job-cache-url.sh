#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

CACHE_BUCKET="ferrocene-ci-caches"
CACHE_PREFIX="persist-between-jobs"

s3_url() {
    # CircleCI provides two IDs in each job's environment:
    #
    # - Workflow ID: the ID of the current workflow, which is the same for all
    #   jobs that are part of it.
    #
    # - Workspace ID: used for CircleCI's attach_workspace feature, in most
    #   cases it's the same as the Workflow ID.
    #
    # The only time the two IDs differ is when a job is retried: then a new
    # Workflow ID is assigned, but the Workspace ID is kept the same as the
    # original one to allow the workspace to be reused.
    #
    # We're reimplementing CirlceCI workspaces in this script, so we're using
    # the Workspace ID as the cache key.
    echo "s3://${CACHE_BUCKET}/${CACHE_PREFIX}/${CIRCLE_WORKFLOW_WORKSPACE_ID}/$1.tar.zst"
}

CIRCLE_JOB=${CIRCLE_JOB:-$1}

s3_url $CIRCLE_JOB
