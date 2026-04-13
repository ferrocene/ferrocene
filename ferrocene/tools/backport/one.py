#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["requests ~= 2.32"]
# ///

# WARNING: This PR is used by `all.py` via `git checkout COMMIT one.py`.
# Backporting PRs that change `one.py` itself is not supported.

import argparse
import os
import requests
import subprocess


GITHUB_REPOSITORY_ENV = "GITHUB_REPOSITORY"
DEFAULT_REPOSITORY = "ferrocene/ferrocene"
RUST_REPOSITORY = "rust-lang/rust"

VERBOSE = False
FORCE = False

def get_pr_metadata(token, repository, pr_number):
    headers = {}
    if token is not None:
        headers["Authorization"] = f"token {token}"
    result = requests.get(
        f"https://api.github.com/repos/{repository}/pulls/{pr_number}",
        headers=headers,
    )
    result.raise_for_status()
    return result.json()

def run(*args, **kwargs):
    if VERBOSE:
        print("running:", *args, file=sys.stderr)
    proc = subprocess.run(
        list(args),
        check=True,
        **kwargs
    )
    if VERBOSE:
        if proc.stdout is not None:
            print("output:", proc.stdout, file=sys.stderr)
        if proc.stderr is not None:
            print("error output:", proc.stderr, file=sys.stderr)
    return proc

def git(*args, **kwargs):
    return run("git", *args, **kwargs)

def git_output(*args):
    return git(*args, text=True, stdout=subprocess.PIPE, stderr=sys.stderr).stdout

def handle_merge_conflict(pr_name, target_branch):
    unmerged = git_output("diff", "--name-status", "--diff-filter=U").splitlines()

    unmerged_solveable = set(file.split()[1] for file in unmerged if not file.startswith("UD"))
    unmerged = set(file.split()[1] for file in unmerged)

    print(f"\nERROR: Encountered a merge conflict backporting {pr_name}")

    if unmerged_solveable:
        print(f"\nINFO: attempting to auto-solve conflicts with mergiraf")
        if FORCE:
            print("INFO: but first, committing the merge markers to git")
            git("add", "--", *unmerged)
            git("-c", "core.editor=true", "cherry-pick", "--continue")
    for file in unmerged_solveable:
        proc = subprocess.run(["mergiraf", "solve", file])
        if proc.returncode == 0:
            unmerged.remove(file)

    if not unmerged:
        print("INFO: Solved all conflicts with mergiraf")
        if FORCE:
            git("add", "--", *unmerged_solveable)
            if git_output("diff", "--name-only", "--cached") == "":
                print("INFO: mergiraf was not able to solve all conflicts")
            else:
                print("INFO: Committing changes")
                git("commit", "--message", "Resolve conflicts using mergiraf")
                exit(0)

    print("HELP: Fix the conflicts, run 'git add/rm <filepath>', then run 'git cherry-pick --continue'.")
    print("NOTE: Running 'git cherry-pick --abort' will put you back in your original git state.")

    print("\nNOTE: conflicts exist in the following locations:")
    subprocess.run("ferrocene/ci/scripts/detect-conflict-markers.py")
    print(f"NOTE: 'HEAD' in the conflicted files refers to your current branch, {target_branch}")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--rust", help="Pull from rust-lang/rust instead", action="store_true"
    )
    parser.add_argument("--verbose", action='store_true', help="Show invoked commands and their outputs")
    parser.add_argument("--force", action='store_true',
                        help="Commit mergiraf changes in a separate commit, rather than"
                             " keeping them around for inspection")
    parser.add_argument("pr_number", help="The PR to backport")
    args = parser.parse_args()
    global VERBOSE
    global FORCE
    if args.verbose:
        VERBOSE = True
    if args.force:
        FORCE = True

    pr_number = args.pr_number
    if args.rust:
        repository = RUST_REPOSITORY
    else:
        repository = DEFAULT_REPOSITORY
    pr_name = f"{repository}#{pr_number}"

    if git_output("diff", "--name-only", "--cached") != "":
        print("ERROR: this script cannot be run when you have staged changes; try running `git reset`")
        exit(1)

    try:
        token = os.environ["GITHUB_TOKEN"]
    except KeyError:
        print("warning: if no API token is set in the GITHUB_TOKEN env var, requests may be rate-limited", file=sys.stderr)
        token = None

    metadata = get_pr_metadata(token, repository, pr_number)
    base, head = metadata["base"]["sha"], metadata["head"]["sha"]

    git("fetch", "--quiet", f"https://github.com/{repository}", base, head)

    authors = git_output("log", "--format=%aN <%aE>", f"{base}..{head}").splitlines()
    primary_author = authors[0]
    co_authors = set(authors)
    co_authors.remove(primary_author)
    formatted_coauthors = ""
    if co_authors:
        formatted_coauthors = "\n".join("Co-authored-by: {author}" for author in co_authors)

    commits = git_output("log", "--format=%H", f"{base}..{head}")
    commits = commits.splitlines()

    formatted_commits = ' '.join(commits)

    commit_message = f"""
Backport {pr_name}

{metadata['title']}

{metadata['body']}{formatted_coauthors}

Ferrocene-backport-of: {pr_name}
Ferrocene-backported-commits: {formatted_commits}"""

    # Save where we currently are.
    # We can't unconditionally use `rev-parse HEAD` because that won't update the target
    # branch ref once we switch back.
    target_branch = git_output("rev-parse", "--abbrev-ref", "HEAD").rstrip()
    if target_branch == "HEAD":
        target_branch = git_output("rev-parse", "HEAD").rstrip()

    print(f"INFO: backporting {pr_name} to {target_branch} (please do not CTRL-C)\n")

    # Avoid modifying the current branch ref when we run `git reset`.
    git("checkout", "--quiet", "--detach", "HEAD")
    # Set the index to the PR head, without modifying the working tree.
    git("reset", "--quiet", "--mixed", head)
    # Set HEAD to the PR base.
    git("reset", "--quiet", "--soft", base)
    # Squash all the PR changes into a single commit.
    git("commit", "--quiet", "--author", primary_author, "--file=-", input=commit_message, text=True)
    new_commit = git_output("rev-parse", "HEAD").rstrip()

    # Set HEAD and index back to the backport branch.
    git("reset", "--quiet", "--mixed", target_branch)
    # Reattach HEAD to the backport branch ref.
    git("checkout", "--quiet", target_branch)
    # Cherry-pick our squashed PR.
    try:
        git("-c", "advice.mergeConflict=false", "-c", "rerere.enabled=false",
            "-c", "merge.conflictStyle=diff3",
            "cherry-pick", "--ff", new_commit)
    except subprocess.CalledProcessError:
        handle_merge_conflict(pr_name, target_branch)
        exit(1)

if __name__ == "__main__":
    main()
