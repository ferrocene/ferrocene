#!/usr/bin/env bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script shows the behavior of `pull-upstream/pull.sh` when there is a conflict in a submodule.
# It does not perform any validation. You have to manually check that your git repo is in the state you expect afterwards.

set -euo pipefail

if ! git diff-index --quiet HEAD; then
    echo "the current branch contains uncommitted changes!"
    echo "make sure all changes are committed before running this script."
    exit 1
fi

git checkout --detach HEAD
git submodule update --init src/tools/cargo
(
    cd src/tools/cargo
    echo 'testing' > src/doc/src/CHANGELOG.md
    git commit src/doc/src/CHANGELOG.md -m "test commit"
)
git commit src/tools/cargo -m "update cargo"
ferrocene/tools/pull-upstream/pull.sh f5e2df741b4a9820a7579f0c8eccc951706a8782 22911fe33f0fd8f4b4fa812f59f0969c46358beb
