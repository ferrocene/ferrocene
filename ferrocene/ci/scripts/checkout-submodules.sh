#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers
#
# Checkout all the submodules in parallel.

set -euo pipefail
IFS=$'\n\t'

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::group::Checkout submodules"
fi

git submodule init
submodules=${*:-$(git config --file .gitmodules --get-regexp path | awk '{ print $2 }')}
current_branch=$(git symbolic-ref --short --quiet HEAD || echo "main")

# Spawn all `git submodule update`s in parallel and then wait for all of them
# to complete. This intentionally doesn't use the `-j` flag, as that flag
# apparently still clones the individual submodules in serial.
for submodule in ${submodules}; do
    # Don't break if the default remote has a name other than `origin`.
    # See the comment in bootstrap::config::update_submodule.
    git -c "branch.${current_branch}.remote=origin" submodule update --depth 1 "${submodule}" &
done
wait

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::endgroup::"
fi