#!/bin/bash
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

set -euo pipefail
IFS=$'\n\t'

UPSTREAM_REPO="https://github.com/rust-lang/rust"
FERROCENE_REPO=https://github.com/ferrocene/ferrocene
TEMP_BRANCH="pull-upstream-temp--do-not-use-for-real-code"
GENERATED_COMPLETIONS_DIR="src/etc/completions/"
X_HELP=src/etc/xhelp

# Set a default max of merges per PR to 30, if it was not overridden in the
# environment.
if [[ -z "${MAX_MERGES_PER_PR+x}" ]]; then
    MAX_MERGES_PER_PR=30
fi

# Print all files with the `ferrocene-avoid-pulling-from-upstream` attribute.
#
# `sort | uniq` is used because during merges files might show up multiple
# times if they have a conflict, and we don't want that.
excluded_files() {
    git ls-files \
        | git check-attr ferrocene-avoid-pulling-from-upstream --stdin \
        | grep ' set$' \
        | sed 's/:.*$//' \
        | sort \
        | uniq
}

automation_warning() {
    message="$@"

    echo
    echo '/!\' "Automation warning:" '/!\'
    echo "${message}"
    echo

    # When running as part of the automation, the warnings are stored in a file instead of being
    # printed to stdout. That way the automation can include them in the PR body.
    if [[ -n "${PULL_WARNINGS_FILE+x}" ]]; then
        echo "${message}" >> "${PULL_WARNINGS_FILE}"
    fi
}

commit_if_modified() {
    file="$1"
    message="$2"

    if git status --porcelain=v1 | grep "^ M ${file}$" >/dev/null; then
        git add "${file}"
        git commit -m "${message}"
    fi
}

if [[ $# -lt 1 ]] || [[ $# -gt 3 ]]; then
    echo "usage: $0 <upstream-branch> [base-branch] [upstream-commit]"
    exit 1
fi
upstream_branch="$1"
if [[ $# -ge 2 ]]; then
    # Allow not having the ref fetched locally (can happen from manual workflow_dispatch runs)
    git fetch "$FERROCENE_REPO" "$2"
    current_branch="$(git rev-parse FETCH_HEAD)"
else
    current_branch="$(git rev-parse HEAD)"
fi
if [[ $# -ge 3 ]]; then
    upstream_commit="$3"
else
    upstream_commit="FETCH_HEAD"  # Latest commit in the branch we pull.
fi

# Ensure we are using an up to date nightly toolchain during the execution of this script. This is
# because some Cargo invocations might touch `Cargo.toml` files relying on unstable features, which
# won't work on the stable toolchain.
#
# We also tell rustup to try and update the nightly toolchain to ensure the behavior of the script
# is consistent even if you run the script in a machine with an out of date nightly.
rustup toolchain install nightly
export RUSTUP_TOOLCHAIN=nightly

# Move to the root of the repository to avoid the script from misbehaving.
cd "$(git rev-parse --show-toplevel)"

# Safety check to avoid messing with uncommitted changes.
# Submodules are updated before that, as submodules needing an update should
# not block merging changes from upstream.
#
# The update-index command ensures diff-index doesn't spuriously fail.
# https://stackoverflow.com/questions/3878624#comment108071431_3879077
git submodule update
git update-index --refresh
if ! git diff-index --quiet HEAD; then
    echo "pull-upstream: the current branch contains uncommitted changes!"
    echo "pull-upstream: make sure all changes are committed before running this script."
    exit 1
fi

# Make sure the temporary branch doesn't exist yet.
if git rev-parse --quiet --verify "${TEMP_BRANCH}" > /dev/null; then
    git branch -D "${TEMP_BRANCH}"
fi

git fetch "${UPSTREAM_REPO}" "${upstream_branch}"

# Avoid creating extra-large PRs by limiting the amount of merge commits
# included in automated pulls to ${MAX_MERGES_PER_DAY}.
partial_pull=no
if [[ "${upstream_commit}" = "FETCH_HEAD" ]]; then
    fetch_head="$(git rev-parse FETCH_HEAD)"
    upstream_commit="$(git log HEAD..FETCH_HEAD --first-parent --format="%H" | tail -n "${MAX_MERGES_PER_PR}" | head -n 1)"
    if [[ "${upstream_commit}" = "" ]]; then
        # When the branch is up to date, the `git log` above rightfully doesn't
        # return any difference between the two refs, resulting in an empty
        # ${upstream_commit}. To prevent the rest of the script from misbehaving,
        # we revert the commit back to FETCH_HEAD.
        upstream_commit="FETCH_HEAD"
    elif [[ "${upstream_commit}" != "${fetch_head}" ]]; then
        partial_pull=yes

        automation_warning "Only the first ${MAX_MERGES_PER_PR} PRs are included in this pull. You should run the automation again after this PR is merged."
    fi
fi

# Upstream is slowly trying to migrate away from submodules in favor of subtrees. Whenever they do
# that, in the same PR they remove the submodule and then create the subtree.
#
# This causes problems whenever the branch containing the subtree is checked out in a working
# directory containing the submodule. Submodules "hide" from git's view the files they contain, and
# when they are removed they stop hiding those, resulting in those files being untracked.
#
# When git tries to add the subtree files in the working directory, it will see the submodule files
# as untracked, and exit with an error to avoid having to overwrite the files, crashing the script.
#
# To avoid the problem, this snippet below `rm -rf`s every directory that is a submodule in the
# current branch but is not a submodule upstream. This correctly solves the problem whenever
# upstream converts a submodule to a subtree. It also has the side effect of removing the local
# contents of the submodules added in Ferrocene (not the submodules themselves), but that doesn't
# cause any problem (the Ferrocene submodules are then re-fetched later).
get_submodules() {
    ref="$1"
    git config --file <(git show "${ref}:.gitmodules") --get-regexp path | awk '{print($2)}'
}
new_submodules="$(get_submodules "${upstream_commit}")"
for submodule in $(get_submodules "${current_branch}"); do
    if ! grep --quiet "^${submodule}\$" <(echo "${new_submodules}"); then
        echo "submodule ${submodule} is not present upstream, removing its contents"
        rm -rf "${submodule}"
    fi
done

git checkout -b "${TEMP_BRANCH}" "${upstream_commit}"

# Delete all the files excluded from the pull. Those files are marked with the
# `ferrocene-avoid-pulling-from-upstream` in `.gitattributes`.
git checkout "${current_branch}" -- .gitattributes
excluded_files | xargs git rm
git checkout FETCH_HEAD -- .gitattributes

git commit -F- <<EOF
remove excluded files from upstream

This commit is generated by \`ferrocene/tools/pull-upstream/pull.sh\`.
The list of excluded files is defined in \`.gitattributes\`.
EOF

# Note that the generate_pr_body.py script relies on this commit message to
# format its output. When changing this, make sure you are not breaking it.
if [[ "${partial_pull}" = "yes" ]]; then
    merge_message="pull new changes from upstream (partial)"
else
    merge_message="pull new changes from upstream"
fi

git checkout "${current_branch}"
if ! git -c merge.conflictstyle=zdiff3 merge "${TEMP_BRANCH}" --no-edit -m "${merge_message}"; then
    # Merging failed, but the script might be able to resolve all the conflicts
    # on its own. This tries to resolve known conflicts and finish the merge.
    # If not all conflicts were resolved, control is given back to the user.

    # Files excluded by the pull that are also present in Ferrocene (for example
    # a different README) will cause merge conflicts. In those cases we always
    # want to preserve Ferrocene's version, so we can resolve the conflict
    # automatically.
    for file in $(excluded_files); do
        echo "pull-upstream: automatically resolving conflict for ${file}..."
        git show "${current_branch}:${file}" > "${file}"
        git add "${file}"
        echo "pull-upstream: automatically resolved conflict for ${file}"
    done

    # Git attempts to merge submodule bumps correctly, but it only works if one
    # of the two branches has the same submodule commit as the merge base. If
    # that's not true (for example if we get behind with pulls), git refuses to
    # merge automatically and outputs this fairly confusing diff:
    #
    # - Subproject commit 03bc66b55c290324bd46eb22e369c8fae1908f91
    #  -Subproject commit 694a579566a9a1482b20aff8a68f0e4edd99bd28
    # ++Subproject commit 0000000000000000000000000000000000000000
    #
    # To solve that, when a submodule gets in an unmerged state, the conflict is
    # fixed automatically by resetting the submodule to upstream's commit.
    # FIXME: if there are any Ferrocene changes to the submodule, those are discarded entirely.
    # This should open a Github issue saying the repo is invalid state and someone needs to manually fix it.
    all_submodules="$(git config --file .gitmodules --get-regexp 'submodule\..+\.path' | awk '{print($2)}')"
    for changed_file in $(git status --porcelain=v1 | sed -n 's/^UU //p'); do
        if grep -q "^${changed_file}$" <(echo "${all_submodules}"); then
            git reset "${upstream_commit}" -- "${changed_file}"
            echo "pull-upstream: discarded all changes for submodule ${changed_file} and reset to upstream's version"
        fi
    done

    if git diff --diff-filter=U --quiet; then
        # Setting the editor to `true` prevents the actual editor from being open,
        # as in this case we don't want to change the default message.
        GIT_EDITOR="$(which true)" git merge --continue
    elif [[ -n "${EXIT_ON_MERGE_CONFLICT+x}" ]]; then
        echo
        echo "pull-upstream: there are unresolved merge conflicts"
        echo "pull-upstream: resolve the conflicts manually and then run \`git merge --continue\`."
        exit 1
    else
        automation_warning "There are merge conflicts in this PR. Merge conflict markers have been committed."

        # We do a `git submodule update` ahead of time to ensure the wrong
        # submodule commits are not accidentally added.
        git submodule update

        # The person handling the conflict should decide what to do if a file
        # has been deleted on either side of the merge, but doing a `git add .`
        # would mask the conflict (it would simply revert the deletion).
        #
        # To avoid that, we prefix the file with a custom line noting the file
        # had a delete conflict, and the detect-conflict-markers.py script will
        # pick it up and block CI until it's resolved either way.
        handle_deleted_files() {
            marker="$1"
            who="$2"
            for changed_file in $(git status --porcelain=v1 | sed -n "s/^${marker} //p"); do
                sed -i "1s/^/<<<PULL-UPSTREAM>>> file deleted ${who}; move the Ferrocene annotations if any, and delete this file\\n/" "${changed_file}"
            done
        }
        handle_deleted_files DU "in Ferrocene" # DU means "deleted by us"
        handle_deleted_files UD "upstream"     # UD means "deleted by them"

        git add .

        # Setting the editor to `true` prevents the actual editor from being open,
        # as in this case we don't want to change the default message.
        GIT_EDITOR="$(which true)" git merge --continue
    fi
fi

# If there were no changes made since the last pull (i.e. when the diff from the
# previous commit and the pull is empty), remove the empty merge commit and
# exit with a special code to let the automation know it shouldn't open PRs.
if git diff --quiet HEAD^..HEAD; then
    echo "pull-upstream: no changes to pull"
    git reset HEAD^
    exit 42
fi

# We keep lockfile for Ferrocene tools fresh
if [ "${upstream_branch}" == "master" ]; then
    prefix="ferrocene/tools"
    lock="${prefix}/Cargo.lock"
    manifest="${prefix}/Cargo.toml"
    echo "pull-upstream: ensure ${lock} has latest semver-compatible crates"
    cargo update --manifest-path "${manifest}"
    commit_if_modified "${lock}" "update ${lock} to latest semver-compatible crates"
fi

# We expose additional commands for `x.py` which affects the completions file generation,
# so we just run the command to regenerate those in case they need updating as this usually
# does not need manual intervention.
echo "pull-upstream: checking whether ${GENERATED_COMPLETIONS_DIR} needs to be updated..."
if ./x.py run generate-completions >/dev/null; then
    commit_if_modified "${GENERATED_COMPLETIONS_DIR}" "update ${GENERATED_COMPLETIONS_DIR}"
else
    automation_warning "Couldn't regenerate the \`x.py\` completions. Please run \`./x run generate-completions\` after fixing the merge conflicts."
fi
echo "pull-upstream: checking whether ${X_HELP} needs to be updated..."
if ./x.py run generate-help >/dev/null; then
    commit_if_modified "${X_HELP}" "update ${X_HELP}"
else
    automation_warning "Couldn't regenerate the \`x.py\` help file. Please run \`./x run generate-help\` after fixing the merge conflicts."
fi

# Some parts of src/stage0 need to be updated when we branch off from main to a release branch.
# Running the fixup script here ensures the fix is always applied.
echo "pull-upstream: trying to fix src/stage0"
ferrocene/ci/scripts/fix-stage0-branch.py || automation_warning "Could not fix src/stage0; will commit with conflict markers"
commit_if_modified src/stage0 "update src/stage0"

git branch -D "${TEMP_BRANCH}"

echo
echo "You can generate the PR body manually by running:"
echo
echo "    ferrocene/tools/pull-upstream/generate_pr_body.py origin <base-branch> <current-branch>"
echo
