#!/usr/bin/env bash

# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail

api_url="https://circleci.com/api/v2/pipeline/$CIRCLE_PIPELINE_ID?circle-token=$CIRCLECI_API_TOKEN"
i=0

while ! [ "$i" = 12 ]; do
    STATUS=$(curl --location --silent --dump-header /dev/stderr "$api_url" \
        | { tee /dev/stderr; echo >>/dev/stderr; } \
        | jq -r '.state')
    case "$STATUS" in
        created) exit 0;;
        errored)
            echo "pipeline errored! failing job."
            exit 1
            ;;
        null|"") ## *something* went wrong
            echo "internal error in 'wait-for-pipeline.sh'"
            exit 2
            ;;
        *) : $((i++)); sleep 5;;
    esac
done

echo "error: continutation workflow took more than a minute to start"
exit 3
