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

LAST_COMMIT_DATE=""
if [[ "${OSTYPE}" =~ ^darwin.* ]]; then
    # Darwin's bash does not support RFC2822 on `touch`
    # `%ct` outputs the commit date formatted according UNIX timestamp
    LAST_COMMIT_DATE="$(git log -1 "--format=%ct" HEAD)"
else
    # `%cD` outputs the commit date formatted according to RFC2822.
    LAST_COMMIT_DATE="$(git log -1 "--format=%cD" HEAD)"
fi


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
    if [[ "${OSTYPE}" =~ ^darwin.* ]]; then
        # Darwin's bash does not support RFC2822 on `touch`, use a UNIX timestamp
        git ls-tree HEAD -rz --name-only | xargs -0 touch -c -t "${LAST_COMMIT_DATE}" --
    else
        git ls-tree HEAD -rz --name-only | xargs -0 touch -c -d "${LAST_COMMIT_DATE}" --
    fi
done

echo "finished resetting the mtime of all tracked files to ${LAST_COMMIT_DATE}"
