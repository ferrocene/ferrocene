#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Script to automatically update the lockfiles in this repo. The list of
# lockfiles to update is stored in the `lockfiles.yml` file in this directory.
#
# When the --automation flag is passed, this script will create pull requests
# for each lockfile, and open issues if updating fails conflict.
# The --automation flag is meant to be passed on GitHub Actions.
#
# Required enviroment variables for the --automation flag:
# - `GITHUB_TOKEN`: API token with access to the repo contents, issues and RPs
# - `GITHUB_REPOSITORY`: name of the GitHub repository to run this script on
#
# If the `HTTP_CLONE_TOKEN` environment variable is provided, that token will
# be used for HTTP cloning, rather than using SSH for cloning.

from dataclasses import dataclass
from pathlib import Path
import argparse
import os
import subprocess
import sys
import yaml

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "common"))

from automated_prs import AutomatedPR, AutomationResult
from pr_links import PRLinker


@dataclass
class CargoLock:
    manifest_path: Path
    lockfile: Path


def parse_configuration(path):
    with open(path) as f:
        contents = yaml.load(f.read(), Loader=yaml.SafeLoader)

    found = []
    poisoned = False
    for item in contents:
        try:
            path = item["path"]
        except KeyError as e:
            print(f"error: missing required key in config item: {e}")
            poisoned = True
            continue
        manifest_path = Path(path)
        found.append(CargoLock(manifest_path, manifest_path.with_suffix(".lock")))

    if poisoned:
        exit(1)
    return found


def retrieve_git_repo_root():
    return Path(run_capture(["git", "rev-parse", "--show-toplevel"]))

def update_lockfile(repo_root, cargo_lockfile):
    output = run_capture(
        [
            "cargo",
            "update",
            "--manifest-path",
            cargo_lockfile.manifest_path,
        ],
        env={"RUSTC_BOOTSTRAP": "1", **os.environ},
        cwd=repo_root,
    )


    status = run_capture(["git", "status", "--porcelain", cargo_lockfile.lockfile], cwd=repo_root)
    if status.strip():
        print(f"Lockfile `{cargo_lockfile.lockfile}` updated")
        run(["git", "add", cargo_lockfile.lockfile], cwd=repo_root)
        run(["git", "commit", "-m", f"Update {cargo_lockfile.lockfile}"], cwd=repo_root)
        return output
    else:
        print(f"Lockfile `{cargo_lockfile.lockfile}` is up to date")
        return None

class UpdateLockfilesPR(AutomatedPR):
    def __init__(self, cargo_lockfile, target):
        self.cargo_lockfile = cargo_lockfile
        self.target = target

    def run(self):
        try:
            self.diff = update_lockfile(self.repo_root, self.cargo_lockfile)
            if self.diff is None:
                return AutomationResult.NO_CHANGES
            else:
                return AutomationResult.SUCCESS
        except subprocess.CalledProcessError:
            return AutomationResult.FAILURE

    def base_branch(self):
        return self.target

    def automation_name(self):
        return "update-lockfiles"

    def pr_title(self):
        return f"Automated lock file update for `{self.cargo_lockfile.lockfile}`"

    def pr_labels(self):
        return {"automation", "backport:never"}

    def pr_body(self, branch_name):
        return f"This PR updates the following entries in {self.cargo_lockfile.lockfile}:\n\n```\n{self.diff}\n```"

    def error_issue_title(self):
        return f"Automatic lockfile updated failed for `{self.cargo_lockfile.lockfile}`"

    def error_issue_labels(self):
        return {"automation"}

    def error_issue_body(self):
        return f"""
While trying to update the lockfile {self.cargo_lockfile.lockfile}, the \
automation failed.
"""

    def error_issue_fixed_comment(self, pull_request_url):
        return f"The automation successfully updated the lockfile {self.cargo_lockfile.lockfile} in {pull_request_url}! Closing this issue."

    def error_issue_repeated_comment(self):
        return f"The automation failed to update the lockfile {self.cargo_lockfile.lockfile} again."

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
    parser.add_argument("--target", help="branch this update is targeting", default=None)
    parser.add_argument(
        "--automation", action="store_true", help="automatically create PRs"
    )
    args = parser.parse_args()

    config_file = Path(__file__).parent / "lockfiles.yml"
    repo_root = retrieve_git_repo_root()
    for lockfile in parse_configuration(config_file):
        if args.automation:
            UpdateLockfilesPR(lockfile, args.target).create()
        else:
            update_lockfile(repo_root, lockfile)
