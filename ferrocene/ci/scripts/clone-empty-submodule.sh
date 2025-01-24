#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Some submodules (like gcc and enzyme) are really big, but we don't need them in some jobs (like
# automations). This script uses partial clones and sparse checkouts to clone an empty version of
# the submodule, without breaking external tools relying on the submodule's presence.

set -euo pipefail
IFS=$'\n\t'

if [[ $# -ne 1 ]]; then
    echo "usage: $0 <path>"
    exit 1
fi
path="$1"

git_submodule() {
    git -C "${path}" $@
}

git submodule init "${path}"
submodule_commit="$(git ls-tree HEAD "${path}" | awk '{print($3)}')"
submodule_url="$(git config --file .gitmodules --get-regexp '\.url$' | grep "submodule.${path}.url" | awk '{print($2)}')"

git init "${path}"
git_submodule remote add origin "${submodule_url}"

# Configure partial clone not to download any blob.
git_submodule config remote.origin.promisor true
git_submodule config remote.origin.partialCloneFilter blob:none

# Configure sparse checkout to only checkout the paths we need.
#
# The --cone flag is an optimization when you need to checkout whole
# directories, not arbitrary patterns, which speeds up later checkouts.
#
# We are not actually adding paths because we don't really care about this submodule.
git_submodule sparse-checkout init --cone

git_submodule fetch --depth=1 origin "${submodule_commit}"
git_submodule checkout "${submodule_commit}"

# Load the git repository we created in src/gcc as the submodule.
git submodule absorbgitdirs
