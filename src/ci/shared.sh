#!/bin/false
# shellcheck shell=bash

# This file is intended to be sourced with `. shared.sh` or
# `source shared.sh`, hence the invalid shebang and not being
# marked as an executable file in git.

export MIRRORS_BASE="https://ci-mirrors.rust-lang.org/rustc"

# Try a command up to 7 times, with randomized exponential backoff between
# attempts.
#
# The backoff sequence is {1-5, 1-10, 1-20, ..., 1-160} seconds, allowing an
# average of 160 seconds (just over 2.5 minutes) and a worst case of 315 seconds
# (just over 5 minutes) before giving up.
#
# Due to shell arithmetic limitations, we always wait for a whole number of
# seconds between retries, but each integer in the above ranges (inclusive)
# is equally likely.
#
# See https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/
# for analysis; this follows the "Full Jitter" method mentioned there, with
# the small difference that we always wait at least 1 second between tries.
#
# Duplicated in docker/scripts/shared.sh
#
# Ferrocene addition: Duplicated in BootstrapCommand::run_retry() in
# src/bootstrap/src/utils/exec.rs
function retry {
  echo "Attempting with retry:" "$@"
  local n=1
  local base=5
  local max=7
  while true; do
    "$@" && break || {
      if [[ $n -lt $max ]]; then
        # $RANDOM returns a random integer in [0, 32767],
        # so remap to a random integer in [1, base * 2^(n - 1)]
        local delay=$(( 1 + $RANDOM * $base * 2**($n - 1) / 32768 ))
        echo "Attempt $n/$max failed. Waiting $delay second(s) before retrying."
        sleep $delay
        ((n++))
        echo "Attempt $n/$max:"
      else
        echo "The command has failed after $n attempts."
        return 1
      fi
    }
  done
}

function isCI {
    [[ "${CI-false}" = "true" ]] || isGitHubActions
}

function isGitHubActions {
    [[ "${GITHUB_ACTIONS-false}" = "true" ]]
}


function isSelfHostedGitHubActions {
    [[ "${RUST_GHA_SELF_HOSTED-false}" = "true" ]]
}

function isMacOS {
    [[ "${OSTYPE}" = "darwin"* ]]
}

function isWindows {
    [[ "${OSTYPE}" = "cygwin" ]] || [[ "${OSTYPE}" = "msys" ]]
}

function isLinux {
    [[ "${OSTYPE}" = "linux-gnu" ]]
}

function isKnownToBeMingwBuild {
    # CI_JOB_NAME must end with "mingw" and optionally `-N` to be considered a MinGW build.
    isGitHubActions && [[ "${CI_JOB_NAME}" =~ mingw(-[0-9]+)?$ ]]
}

function isCiBranch {
    if [[ $# -ne 1 ]]; then
        echo "usage: $0 <branch-name>"
        exit 1
    fi
    name="$1"

    if isGitHubActions; then
        [[ "${GITHUB_REF}" = "refs/heads/${name}" ]]
    else
        echo "isCiBranch only works inside CI!"
        exit 1
    fi
}

function ciBaseBranch {
    if isGitHubActions; then
        echo "${GITHUB_BASE_REF#refs/heads/}"
    else
        echo "ciBaseBranch only works inside CI!"
        exit 1
    fi
}

function ciCommit {
    if isGitHubActions; then
        echo "${GITHUB_SHA}"
    else
        echo "ciCommit only works inside CI!"
        exit 1
    fi
}

function ciCheckoutPath {
    if isGitHubActions; then
        echo "${GITHUB_WORKSPACE}"
    else
        echo "ciCheckoutPath only works inside CI!"
        exit 1
    fi
}

function ciCommandAddPath {
    if [[ $# -ne 1 ]]; then
        echo "usage: $0 <path>"
        exit 1
    fi
    path="$1"

    if isGitHubActions; then
        echo "${path}" >> "${GITHUB_PATH}"
    else
        echo "ciCommandAddPath only works inside CI!"
        exit 1
    fi
}

function ciCommandSetEnv {
    if [[ $# -ne 2 ]]; then
        echo "usage: $0 <name> <value>"
        exit 1
    fi
    name="$1"
    value="$2"

    if isGitHubActions; then
        echo "${name}=${value}" >> "${GITHUB_ENV}"
    else
        echo "ciCommandSetEnv only works inside CI!"
        exit 1
    fi
}

function releaseChannel {
    if [[ -z "${RUST_CI_OVERRIDE_RELEASE_CHANNEL+x}" ]]; then
        cat "${ci_dir}/channel"
    else
        echo $RUST_CI_OVERRIDE_RELEASE_CHANNEL
    fi
}

# Parse values from src/stage0 file by key
function parse_stage0_file_by_key {
    local key="$1"
    local file="$ci_dir/../stage0"
    local value=$(awk -F= '{a[$1]=$2} END {print(a["'$key'"])}' $file)
    if [ -z "$value" ]; then
        echo "ERROR: Key '$key' not found in '$file'."
        exit 1
    fi
    echo "$value"
}
