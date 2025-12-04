#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail

fatal() {
    echo "$@" >&2
    exit 1
}
if ! [ $# = 3 ]; then
    msg="usage: $0 <repo> <run id> <out.zip>"
    msg+=$(printf "\nexample: %s ferrocene/ferrocene 19787554633 build/logs.zip" "$0")
    msg+=$(printf "\nhelp: to get a run id, go to the logs of the job that failed and copy the part of the URL immediately following '/runs/'.")
    fatal "$msg"
fi
if ! [ -n "${GITHUB_TOKEN:-}" ]; then
    fatal "GTIHUB_TOKEN must be set to download logs"
fi

repo=$1
run=$2
out=$3

curl -L \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    "https://api.github.com/repos/$repo/actions/runs/$run/logs" \
    -o "$out"
