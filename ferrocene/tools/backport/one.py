#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["requests ~= 2.32"]
# ///

import argparse
import os
import requests
import sys
import subprocess


GITHUB_REPOSITORY_ENV = "GITHUB_REPOSITORY"
DEFAULT_REPOSITORY = "ferrocene/ferrocene"
RUST_REPOSITORY = "rust-lang/rust"


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

def git(*args, **kwargs):
    return subprocess.run(
        ["git"] + list(args),
        text=True,
        check=True,
        **kwargs
    )

def git_output(*args):
    return git(*args, stdout=subprocess.PIPE).stdout

def handle_merge_conflict(pr_number, commit_message, author, e):
    print(f"ERROR: failed to apply diff for {pr_number}: {e}")
    git_dir = git_output("rev-parse", "--git-dir").rstrip()
    msg_file = git_dir + "/COMMIT_EDITMSG.ferrocene"
    help = f"HELP: once you have resolved the conflict, run `git commit --author='{author}' --edit "
    try:
        with open(msg_file, 'w') as fd:
            fd.write(commit_message)
        help += f"--file='{msg_file}'"
    except Exception as e:
        print(f"WARN: failed to write {msg_file}: {e}", file=sys.stderr)
        help += f"--message='{commit_message}'"
    print(help, file=sys.stderr)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--rust", help="Pull from rust-lang/rust instead", action="store_true"
    )
    parser.add_argument("pr_number", help="The PR to backport")
    args = parser.parse_args()

    pr_number = args.pr_number
    if args.rust:
        repository = RUST_REPOSITORY
    else:
        repository = DEFAULT_REPOSITORY

    try:
        token = os.environ["GITHUB_TOKEN"]
    except KeyError:
        print("warning: if no API token is set in the GITHUB_TOKEN env var, requests may be rate-limited", file=sys.stderr)
        token = None

    metadata = get_pr_metadata(token, repository, pr_number)
    base, head = metadata["base"]["sha"], metadata["head"]["sha"]

    git("fetch", f"https://github.com/{repository}", base, head)

    authors = git_output("log", "--format=%aN <%aE>", f"{base}..{head}").splitlines()
    primary_author = authors[0]
    co_authors = set(authors)
    co_authors.remove(primary_author)
    formatted_authors = ""
    if co_authors:
        formatted_authors = "\n".join("Co-authored-by: {author}" for author in co_authors)

    commits = git_output("log", "--format=%H", f"{base}..{head}")
    commits = commits.splitlines()

    repo = metadata['head']['repo']['html_url']
    formatted_commits = ' '.join(f"[{sha}]({repo}/commit/{sha})" for sha in commits)

    commit_message = f"""
Backport {pr_number} (`{metadata['html_url']}`)

{metadata['title']}

{metadata['body']}{formatted_authors}

Ferrocene-backport-of: {repository}#{pr_number}
Ferrocene-backported-commits: {formatted_commits}"""

    pr_diff = git_output("-c", "core.pager=", "diff", base, head)

    try:
        git("apply", "--3way", "--index", input=pr_diff)
    except subprocess.CalledProcessError as e:
        handle_merge_conflict(pr_number, commit_message, primary_author, e)
        exit(e.returncode)

    git("commit", "--author", primary_author, "--file=-", input=commit_message)


if __name__ == "__main__":
    main()
