#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

CACHE_BUCKET="ferrocene-ci-caches"
CACHE_PREFIX="persist-between-jobs"

usage() {
    echo "usage: $0 upload <path ...>"
    echo "usage: $0 restore <job-name>"
}

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

if [[ "$#" -lt 1 ]]; then
    usage 1>&2
    exit 1
fi
case "$1" in
    upload)
        # Remove the first argument from the list of arguments supplied to this
        # script. This will allow `$@` to only contain the provided paths.
        shift

        if [[ "$#" -lt 1 ]]; then
            echo "error: no paths to upload were provided" 1>&2
            usage 1>&2
            exit 1
        fi

        # Call `zstd` separately to be able to use all cores available (`-T0`)
        # compression level chosen after light experimentation.
        # :NOTE: This compression level is good for our current setup, changes such
        # as faster CPU and/or different networks speeds have impact on the optimum
        #
        # Use verbose to produce output while packing so that we can see the job is
        # progressing. It shouldn't slow us down much here since we're largely dominated
        # by the effort of packing.
        echo "Creating tar archive"
        tar c --verbose --exclude build/metrics.json "$@" | zstd -10 -T0 -o /tmp/persist_${CIRCLE_JOB}.tar.zst
        echo "Uploading tar archive"
        # if you intend to add retries here, configure them in the AWS cli settings
        # read https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-retries.html
        aws s3 --progress-frequency=10 --progress-multiline cp /tmp/persist_${CIRCLE_JOB}.tar.zst "$(s3_url "${CIRCLE_JOB}")"
        ;;
    restore)
        if [[ "$#" -ne 2 ]]; then
            usage 1>&2
            exit 1
        fi
        job="$2"

        echo "Downloading tar archive"
        # if you intend to add retries here, configure them in the AWS cli settings
        # read https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-retries.html
        aws s3 --progress-frequency=10 --progress-multiline cp "$(s3_url "${job}")" /tmp/persist_${CIRCLE_JOB}.tar.zst
        echo "Unpacking tar archive"
        # Unpacking is faster than packing and logging each file causes a significant
        # slowdown, so no verbose here.
        unzstd --stdout /tmp/persist_${CIRCLE_JOB}.tar.zst  | tar x
        echo "Done unpacking"
        ;;
    *)
        usage 1>&2
        exit 1
        ;;
esac
