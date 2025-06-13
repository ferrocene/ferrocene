#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# The llvm-project monorepo is really big, and it takes around 45 seconds to do
# a shallow clone of the latest commit. During most of the jobs we need just a
# small subset of it though, wasting a lot of time cloning unnecessary parts.
#
# This script uses two git features to only fetch what we need:
#
# - Partial clones are used to do a file-less clone of the repository,
#   deferring the fetch of the individual files only when they're needed.
#
# - Sparse checkouts are used to only checkout a subset of directories instead
#   of the whole tree. Combined with partial clones this results in only the
#   files from those directories being fetched.
#
# The script is configured to only fetch the subset of the monorepo needed to
# do everything we need *but* building LLVM: building LLVM requires a big chunk
# of the monorepo to be present, and at that point it's faster to just do a
# shallow clone of the whole monorepo rather than using partial clones.

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::group::Clone llvm subset"
fi

MONOREPO_PATH="src/llvm-project"
MONOREPO_URL="https://github.com/rust-lang/llvm-project"
CLONE_DIRECTORIES=(
    "cmake"
    "compiler-rt"
    "libunwind"
    "lld"
    "llvm/LICENSE.txt"
    "llvm/cmake/modules"
    # Some files in these directories have different licenses not used anywhere
    # else in the Ferrocene monorepo. If we don't fetch them as part of the
    # subset, invoking REUSE will complain about missing licenses in jobs that
    # use the LLVM subset rather than the whole LLVM clone.
    "llvm/include"
)

set -euo pipefail
IFS=$'\n\t'

git_llvm() {
    git -C "${MONOREPO_PATH}" $@
}

git submodule init "${MONOREPO_PATH}"
llvm_commit="$(git ls-tree HEAD "${MONOREPO_PATH}" | awk '{print($3)}')"

git init "${MONOREPO_PATH}"
git_llvm remote add origin "${MONOREPO_URL}"

# Configure partial clone not to download any blob.
git_llvm config remote.origin.promisor true
git_llvm config remote.origin.partialCloneFilter blob:none

# Configure sparse checkout to only checkout the directories we need.
#
# The --cone flag is an optimization when you need to checkout whole
# directories, not arbitrary patterns, which speeds up later checkouts.
git_llvm sparse-checkout init --cone
git_llvm sparse-checkout set "${CLONE_DIRECTORIES[@]}"

git_llvm fetch --depth=1 origin "${llvm_commit}"
git_llvm checkout "${llvm_commit}"

# Load the git repository we created in src/llvm-project as the submodule.
git submodule absorbgitdirs

if [[ "$GITHUB_ACTIONS" == "true" ]]; then
    echo "::endgroup::"
fi