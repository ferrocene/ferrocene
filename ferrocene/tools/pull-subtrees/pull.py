#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["pyyaml ~= 6.0", "automations-common"]
#
# [tool.uv.sources]
# automations-common = { path = "../automations-common", editable = true }
# ///

# Script to automatically pull new changes from all subtrees. The list of
# pulled subtrees is stored in the `subtrees` file in this directory.
#
# When the --automation flag is passed, this script will create pull requests
# for each subtree, and open issues if the pulling fails due to a merge
# conflict. The --automation flag is meant to be passed on GitHub Actions.
#
# Required enviroment variables for the --automation flag:
# - `GITHUB_TOKEN`: API token with access to the repo contents, issues and RPs
# - `GITHUB_REPOSITORY`: name of the GitHub repository to run this script on

from automations_common import AutomatedPR, AutomationResult, PRLinker
from dataclasses import dataclass
from pathlib import Path
from typing import List
import argparse
import itertools
import os
import subprocess
import sys
import yaml


@dataclass
class PullLatestTag:
    pass


@dataclass
class PullBranch:
    name: str


@dataclass
class Subtree:
    path: Path
    repo: str
    pull: PullLatestTag | PullBranch
    into: List[str]
    after: List[str]


def parse_configuration(path):
    with open(path) as f:
        contents = yaml.load(f.read(), Loader=yaml.SafeLoader)

    found = []
    poisoned = False
    for item in contents:
        try:
            path = item["path"]
            repo = item["repo"]
            raw_pull = item["pull"]
        except KeyError as e:
            print(f"error: missing required key in config item: {e}")
            poisoned = True
            continue

        if raw_pull == "latest-tag":
            pull = PullLatestTag()
        elif raw_pull.startswith("branch:"):
            pull = PullBranch(name=raw_pull.removeprefix("branch:"))
        else:
            print(f"error: invalid pull key: {raw_pull}")
            poisoned = True
            continue

        actions = []
        if "after" in item:
            for action in item["after"]:
                if action.split(" ")[0] not in AFTER_ACTIONS:
                    print(f"error: unknown after action: {action}")
                    poisoned = True
                    continue
                actions.append(action)

        try:
            into = item["into"]
        except KeyError:
            into = ["main"]

        found.append(Subtree(Path(path), repo, pull, into, actions))

    if poisoned:
        exit(1)
    return found


def retrieve_git_repo_root():
    return Path(run_capture(["git", "rev-parse", "--show-toplevel"]))


def resolve_commit(ref):
    return run_capture(["git", "rev-parse", ref])


def fetch_latest_commit(subtree):
    if isinstance(subtree.pull, PullLatestTag):
        print(f"fetching latest tag from {subtree.repo}")
        tags = run_capture(
            [
                "git",
                # To authenticate requests, GitHub Actions sets the Authorization
                # header to its own token in the HTTP extra headers:
                "-c",
                "http.https://github.com.extraheader=",
                # List tags in the remote:
                "ls-remote",
                "--tags",
                # Sort tags by the inverse (`-`) refname using version sorting:
                "--sort=-version:refname",
                f"https://github.com/{subtree.repo}",
            ]
        )
        # Go through the tag to find the latest one:
        for tag in tags.split("\n"):
            commit, name = tag.split("\t")

            # Check whether this tag is a tag of the main project or an unrelated tag in the
            # repository. We do this by ensuring the tag starts with a digit or with "v" followed by
            # a digit. This excludes the `backtrace-sys-0.NN.NN` tags present in the backtrace repo.
            if not name.startswith("refs/tags/"):
                continue
            name = name.removeprefix("refs/tags/")
            if name[0] == "v":
                name = name[1:]
            if not name[0].isdigit():
                continue

            ref = commit
            break
        else:
            raise RuntimeError(f"no suitable tags found in {subtree.repo}")
    elif isinstance(subtree.pull, PullBranch):
        ref = subtree.pull.name
    else:
        raise RuntimeError(f"unknown subtree.pull: {subtree.pull}")

    print(f"fetching ref {ref} from {subtree.repo}")
    run(
        [
            "git",
            # To authenticate requests, GitHub Actions sets the Authorization
            # header to its own token in the HTTP extra headers.
            "-c",
            "http.https://github.com.extraheader=",
            # Fetch the latest commit in the subtree's ref
            "fetch",
            f"https://github.com/{subtree.repo}",
            ref,
        ]
    )
    return resolve_commit("FETCH_HEAD")


def find_previous_commit(subtree):
    before = None
    while True:
        cmd = [
            "git",
            "log",
            # Only fetch the first result
            #
            # We prefer to fetch just one result and then re-issue the git log
            # command in case the commit we found is not the one we're looking
            # for, rather than issuing the git log command without a limit.
            # That's because our history is *looong*.
            "-1",
            # Only show the first line of the commit message
            "--format=%h|%s",
            # Only show commits generated by this tool
            f"--grep=bump subtree {subtree.path} to",
        ]
        if before is not None:
            cmd.append(f"{before}^")
        output = run_capture(cmd)
        if not output:
            return
        hash, message = output.split("|", 1)

        # git log's --grep flag also searches in the commit message, not just the
        # commit summary (the first line of the message). To work around the
        # problem we double-check that it's indeed the commit we want, and
        # otherwise rerun the search starting from the commit before the one we
        # found just now.
        if not message.startswith(f"bump subtree {subtree.path} to"):
            print(f"warning: commit {hash} is not a subtree pull, skipping")
            before = hash
            continue
        return resolve_commit(message.split(" ")[-1])


def generate_merged_pull_requests_list(subtree, previous_commit, latest_commit):
    messages = run_capture(
        [
            "git",
            "log",
            # Only show merges in the current branch
            "--first-parent",
            "--merges",
            # Only show the first line of the commit message
            "--format=%s",
            # Reverse the order
            "--reverse",
            # Use the byte zero to separate commits rather than newlines
            # This avoids misbehaving when commit messages include newlines
            "-z",
            f"{previous_commit}..{latest_commit}",
        ]
    )

    linker = PRLinker()

    result = ""
    for line in messages.split("\0"):
        if not line:
            continue
        if (prs := remove_prefix(line, "Merge ")) != line:
            for pr in prs.split(" "):
                if not pr:
                    continue
                if (number := remove_prefix(pr, "#")) == pr:
                    continue
                if not number.isdigit():
                    continue
                result += f"* {linker.link(subtree.repo, number)}\n"
        elif (pr := remove_prefix(line, "Auto merge of #")) != line:
            number, _ = pr.split(" - ", 1)
            if not number.isdigit():
                continue
            result += f"* {linker.link(subtree.repo, number)}\n"

    return result


def update_subtree(repo_root, subtree):
    latest_commit = fetch_latest_commit(subtree)
    message = f"bump subtree {subtree.path} to {latest_commit[:12]}"

    diff = "* *Initial pull of the repository.*\n"
    if (repo_root / subtree.path).is_dir():
        previous_commit = find_previous_commit(subtree)
        if previous_commit is None:
            print("warning: could not find any commit previously bumping this subtree")
        elif previous_commit == latest_commit:
            print(f"subtree {subtree.path} is already up to date")
            return
        else:
            diff = generate_merged_pull_requests_list(
                subtree, previous_commit, latest_commit
            )

        print(f"updating subtree {subtree.path}")
        commit_before = resolve_commit("HEAD")
        try:
            run(
                [
                    "git",
                    "subtree",
                    "merge",
                    "--prefix",
                    subtree.path,
                    latest_commit,
                    "-m",
                    message,
                ],
                cwd=repo_root,
            )
        except subprocess.CalledProcessError:
            print("pull-subtrees: there are unresolved merge conflicts")
            print("pull-subtrees: comitting with merge conflicts markers in the source")

            # handle deleted files
            git_status = run_capture(["git", "status", "--porcelain=v1"])
            for line in git_status.splitlines():
                path = None
                who = None
                if line.startswith("DU"):
                    line.split(' ', 1)[1].strip()
                    who = 'in Ferrocene'
                elif line.startswith("UD"):
                    path = line.split(' ', 1)[1].strip()
                    who = 'upstream'
                if path:
                    header = "<<<PULL-UPSTREAM>>> file deleted " + who + "; move the Ferrocene annotations if any, and delete this file"
                    with open(path, 'r') as original:
                        data = original.read()
                    with open(path, 'w') as modified:
                        modified.write(header + '\n' + data)

            run(["git", "add", "."], cwd=subtree.path)

            git_env = os.environ.copy()
            git_env["GIT_EDITOR"] = "true"
            run(["git", "merge", "--continue"], env=git_env)

        # Mark the update as not being executed (returning None) when no commit
        # was created by the subtree pull. Otherwise the PR automation will try
        # creating a PR without diff, which will be rejected by GitHub.
        if resolve_commit("HEAD") == commit_before:
            print("warning: tried to update subtree, but no change was committed")
            return
    else:
        print(f"creating subtree {subtree.path}")
        (repo_root / subtree.path.parent).mkdir(parents=True, exist_ok=True)
        run(
            [
                "git",
                "subtree",
                "add",
                "--prefix",
                subtree.path,
                latest_commit,
                "-m",
                message,
            ],
            cwd=repo_root,
        )

    for action in subtree.after:
        print(f"executing after action {action}")
        action = action.split(" ")
        AFTER_ACTIONS[action[0]](subtree, repo_root, action[1:])

    return diff


def action_update_cargo_lock(subtree, repo_root, params):
    # Force Cargo to refresh the Cargo.lock without building anythiing.
    run(
        ["cargo", "metadata", "--format-version=1"],
        # Some Cargo.tomls use unstable features.
        env={"RUSTC_BOOTSTRAP": "1", **os.environ},
        stdout=subprocess.DEVNULL,
        cwd=repo_root,
    )

    status = run_capture(["git", "status", "--porcelain", "Cargo.lock"], cwd=repo_root)
    if status.strip():
        run(["git", "add", "Cargo.lock"], cwd=repo_root)
        run(["git", "commit", "-m", "update Cargo.lock"], cwd=repo_root)

    if len(params) < 2:
        raise RuntimeError("update-cargo-lock expects at least two parameters")

    for [package_name, workspace] in itertools.batched(params, 2):
        run(
            [
                "cargo",
                "update",
                "--manifest-path",
                f"{repo_root}/{workspace}/Cargo.toml",
                "-p",
                package_name,
            ],
            # Some Cargo.tomls use unstable features.
            env={"RUSTC_BOOTSTRAP": "1", **os.environ},
            stdout=subprocess.DEVNULL,
            cwd=repo_root,
        )

        lockfile = f"{repo_root}/{workspace}/Cargo.lock"
        status = run_capture(["git", "status", "--porcelain", lockfile], cwd=repo_root)
        if status.strip():
            run(
                [
                    "git",
                    "add",
                    lockfile,
                ],
                cwd=repo_root,
            )
            run(
                ["git", "commit", "-m", f"update {workspace}/Cargo.lock"], cwd=repo_root
            )


AFTER_ACTIONS = {
    "update-cargo-lock": action_update_cargo_lock,
}


class PullSubtreePR(AutomatedPR):
    def __init__(self, subtree, into):
        assert into in subtree.into

        self.subtree = subtree
        self.into = into
        self.repo_link = f"[`{subtree.repo}`](https://github.com/{subtree.repo})"

    def run(self):
        try:
            self.diff = update_subtree(self.repo_root, self.subtree)
            if self.diff is None:
                return AutomationResult.NO_CHANGES
            else:
                return AutomationResult.SUCCESS
        except subprocess.CalledProcessError:
            self.cmd(["git", "merge", "--abort"], check=False)
            return AutomationResult.FAILURE

    def base_branch(self):
        return self.into

    def automation_name(self):
        return f"pull-subtree"

    def pr_title(self):
        if len(self.subtree.into) > 1:
            return f"Automated pull from `{self.subtree.repo}` into `{self.into}`"
        else:
            return f"Automated pull from `{self.subtree.repo}`"

    def pr_labels(self):
        return {"automation", "backport:never"}

    def pr_body(self, branch_name):
        return f"This PR pulls the following changes from the {self.repo_link} repository:\n\n{self.diff}"

    def error_issue_title(self):
        return f"Repository diverged from `{self.subtree.repo}`"

    def error_issue_labels(self):
        return {"automation"}

    def error_issue_body(self):
        diff = self.cmd_capture(
            ["git", "diff", "--diff-filter=U", "--no-color"],
        )

        return f"""
While trying to pull the latest changes from {self.repo_link}, the \
automation failed due to the following merge conflict:

```diff
{diff}
```

The automation will not open any more pull requests pulling this subtree \
until the merge conflict is fixed locally.

<details>
<summary><i>How to fix the merge conflict manually</i></summary><br>

To fix the conflict, create a new branch and execute the following command:

```
ferrocene/tools/pull-subtrees/pull.py {self.subtree.repo}
```

The command will try pull the latest changes from the repository into the \
branch, and exit when the merge conflict is detected. Fix the merge conflict \
and then run:

```
git merge --continue
```

The command will finish the merge. You'll then be able to push the \
local branch to GitHub and open a PR for it.

</details>
"""

    def error_issue_fixed_comment(self, pull_request_url):
        return f"The automation successfully pulled the latest changes from {self.repo_link} in {pull_request_url}! Closing this issue."

    def error_issue_repeated_comment(self):
        return f"The automation failed to pull the latest changes from {self.repo_link} again."


# Polyfill for str.removeprefix for older Python versions
def remove_prefix(string, prefix):
    if string.startswith(prefix):
        return string[len(prefix) :]
    else:
        return string


def run(*args, **kwargs):
    kwargs.setdefault("check", True)
    return subprocess.run(*args, **kwargs)


def run_capture(*args, **kwargs):
    kwargs.setdefault("check", True)
    kwargs.setdefault("stdout", subprocess.PIPE)
    kwargs.setdefault("text", True)
    return subprocess.run(*args, **kwargs).stdout.strip()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    exclusive = parser.add_mutually_exclusive_group()
    exclusive.add_argument("--automation-for-branch", help="run the automation for the provided branch")
    exclusive.add_argument("subtree_repo", help="the subtree to pull", default=None, nargs="?")
    args = parser.parse_args()

    config_file = Path(__file__).parent / "subtrees.yml"
    repo_root = retrieve_git_repo_root()

    subtrees = parse_configuration(config_file)
    if args.automation_for_branch is not None:
        # args.automation_for_branch contains the target branch.
        for subtree in subtrees:
            if args.automation_for_branch not in subtree.into:
                continue
            PullSubtreePR(subtree, args.automation_for_branch).create()
    # We can use an `elif` here because automation_for_branch and subtree_repo are mutually exclusive
    elif args.subtree_repo is not None:
        subtree = None
        for candidate in subtrees:
            if args.subtree_repo == candidate.repo:
                subtree = candidate
                break
        if subtree is None:
            print(f"error: no subtree for repository {args.subtree_repo}")
            exit(1)
        update_subtree(repo_root, subtree)
    else:
        for subtree in subtrees:
            update_subtree(repo_root, subtree)
