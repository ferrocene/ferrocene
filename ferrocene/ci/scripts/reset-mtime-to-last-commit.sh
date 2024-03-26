#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# For each target, Ferrocene's CI pipeline runs an initial job that builds the
# compiler, and then multiple parallel jobs are started to finish the work,
# with each new job reusing the artifacts produced in the initial job.
#
# Unfortunately just transferring the `build` directory from a job to another
# doesn't work, because the freshly cloned git repository on the new job will
# have a higher modification time (mtime) compared to the `build` directory
# transferred from the old job. This results in Cargo thinking the `build`
# directory is outdated, leading to a rebuild of everything.
#
# To avoid the problem this script sets the mtime of all the files tracked by
# git to a consistent timestamp in the past, namely the commit time. The commit
# time will always be lower than the creation time of the `build` directory,
# fixing the problem of Cargo rebuilding too much stuff.

set -euo pipefail
IFS=$'\n\t'

TOUCH="touch"
# Mac's touch does not enjoy parsing dates in any format git will output,
# use a more consistent version of touch
if [[ "${OSTYPE}" =~ ^darwin.* ]]; then
    TOUCH="$HOMEBREW_PREFIX/opt/uutils-coreutils/libexec/uubin/touch"
fi
echo "Touch is ${TOUCH}"

# `%ci` outputs the commit date formatted according to ISO8601.
last_commit_date="$(git log -1 "--format=%ci" HEAD)"

project_root="$(pwd)"
for repo in . $(git config --file .gitmodules --get-regexp path | awk '{ print $2 }'); do
    cd "${project_root}/${repo}"

    # `-z` separates the file names with a byte 0 instead of a newline, making the
    # invocation more resilient to strange file names. The counterpart in `xargs`
    # is the `-0` flag.
    #
    # `-r` recurses into subdirectories.
    #
    # `-c` prevents touch from creating missing files (needed for sparse
    # checkouts of the LLVM monorepo, see the clone-llvm-subset.sh script).
    #
    # `--` instructs touch to treat all following arguments as files, even if
    # their names start with `-` (and would thus be interpreted as an option).
    git ls-tree HEAD -rz --name-only | xargs -0 ${TOUCH} -c -d "${last_commit_date}" --
done

echo "finished resetting the mtime of all tracked files to ${last_commit_date}"
