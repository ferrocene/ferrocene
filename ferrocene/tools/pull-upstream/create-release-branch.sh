#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script creates the base branch for each rolling release channel (which
# corresponds to the upstream stable channel) by looking at the version of the
# provided upstream ref (like "stable" or "beta") and creating the branch with
# the last Ferrocene commit of that version.
#
# Note that the last Ferrocene commit of that version might be behind the last
# upstream commit of that version: upstream takes care not to rollup version
# bumps with other changes, while we don't do that during our regular pulls
# from upstream. That's not a problem though, as any followup pull from
# upstream on the newly created branch will bring Ferrocene on par.

set -euo pipefail
IFS=$'\n\t'

UPSTREAM_REPO="rust-lang/rust"
VERSION_FILE="src/version"
FERROCENE_ORIGIN="origin"

fetch_upstream_version() {
    branch="$1"

    # curl will print the response (and thus the version) to stdout, returning
    # it from the function.
    if ! curl -s --fail "https://raw.githubusercontent.com/${UPSTREAM_REPO}/${branch}/${VERSION_FILE}"; then
        echo "error: failed to fetch ${VERSION_FILE} for branch ${branch}" 1>&2
        exit 1
    fi
}

find_last_commit_with_version() {
    version="$1"
    root="$(git rev-parse --show-toplevel)"

    # To find the last commit of the version we want to create a branch of, we
    # look for all the commits that changed the `src/version` file and collect
    # their *parents*. Since version numbers are tracked in that file, the
    # commit before a version number change has to be the last one with the
    # previous version number.
    #
    # Explanation of the flags:
    #   --merges: only print merge commits
    #   --first-parent: do not traverse merges inside merges
    #   --format="%P": only show the SHA of the parent commit
    #
    # Since merge commits have multiple parents, awk is also used to fetch the
    # first parent.
    commits="$(git log --merges --first-parent --format="%P" -- "${root}/${VERSION_FILE}" | awk '{print($1)}')"

    for previous_commit in ${commits}; do
        content="$(git show "${previous_commit}:${VERSION_FILE}")"
        # The star after "${version}" matches all trailing content. This is
        # needed to properly match the last patch release of the minor version.
        if [[ "${content}" = "${version}"* ]]; then
            echo "${previous_commit}"
            return
        fi
    done

    echo "error: failed to find the last commit with version ${version}" 1>&2
    exit 1
}

if [[ $# -ne 1 ]]; then
    echo "error: this command accepts one argument (the upstream branch name)"
    exit 1
fi
upstream_branch="$1"

version="$(fetch_upstream_version "${upstream_branch}")"
# Discard the patch version and trailing contents in the version number by only
# retaining the first two dot-separated numbers.
minor_version="$(echo "${version}" | awk '{split($0,n,"."); print(n[1] "." n[2])}')"
branch_name="release/${minor_version}"

# `grep -Fx` matches the whole line (`-x`) and matches the pattern literally
# without treating it as a regex (`-F`).
if git ls-remote "${FERROCENE_ORIGIN}" | awk '{print($2)}' | grep -Fx "refs/heads/${branch_name}" >/dev/null; then
    echo "branch ${branch_name} already exists" 1>&2

    # Ensure the local branch is present (tracking the upstream branch)
    if ! git rev-parse --quiet --verify "${branch_name}" > /dev/null; then
        git fetch "${FERROCENE_ORIGIN}"
        git branch "${branch_name}" "${FERROCENE_ORIGIN}/${branch_name}"
    fi
else
    echo "branch ${branch_name} does not exist, pushing it" 1>&2

    last_commit="$(find_last_commit_with_version "${minor_version}")"

    # Create the branch pointing to the commit. If the branch already exists
    # locally delete it beforehand.
    if git rev-parse --quiet --verify "${branch_name}" > /dev/null; then
        git branch -D "${branch_name}"
    fi
    git branch "${branch_name}" "${last_commit}"

    git push "${FERROCENE_ORIGIN}" "${branch_name}"
fi

# Let other parts of the GitHub Actions workflow know what the branch name is.
if [[ -v GITHUB_ACTIONS ]]; then
    echo "name=${branch_name}" >> "${GITHUB_OUTPUT}"
fi
