#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["automations-common"]
#
# [tool.uv.sources]
# automations-common = { path = "../automations-common", editable = true }
# ///

# Run the beta branch.
# See https://public-docs.ferrocene.dev/main/qualification/internal-procedures/release/stable.html.

from automations_common import AutomatedPR, AutomationResult
from automations_common import cmd, cmd_capture

import argparse
import fileinput
import getpass
import shutil
import os

## Global state

VERBOSE = False
DRY_RUN = False
CURRENT_USER = f"`@{getpass.getuser()}`"

## Helpers

RELEASE_DOCS_URL = "https://public-docs.ferrocene.dev/main/qualification/internal-procedures/release"
FERROCENE_URL = "https://github.com/ferrocene/ferrocene"
FERROCENE_PULLS_URL = f"{FERROCENE_URL}/pulls"
KNOWN_PROBLEMS_REPO = "ferrocene/problems"

def git(*args, **kwargs):
    return cmd(["git"] + list(args), **kwargs)

def git_output(*args, **kwargs):
    return cmd_capture(["git"] + list(args), **kwargs)

def release_branch(remote, upstream_version):
    return remote + "/release/" + upstream_version

def checkout_branch(branch):
    git("switch", "--detach", "--no-guess", branch)

def modified_files():
    changed = git_output("diff", "--name-only", "HEAD")
    if not changed:
        return []
    return changed.split()

## Read-only logic

def parse_args():
    parser = argparse.ArgumentParser(description="Run the beta version bump")
    parser.add_argument("--dry-run", action="store_true",
                        help="Don't open PRs, only make local changes. "
                             "Still requires network access to query GitHub.")
    parser.add_argument("--ferrocene-remote", default="origin")
    parser.add_argument("--known-problems-checkout", default="../problems",
                        help="Path to a checkout of https://github.com/ferrocene/problems/")
    parser.add_argument("ferrocene_version",
                        help="Which version is going to be released? (example: 26.08)")
    parser.add_argument("upstream_version",
                        help="Which version of rust-lang/rust is the new beta going to be based on? (example: 1.97)")
    parser.add_argument("old_upstream_version",
                        help="Which version of rust-lang/rust is the current stable based on? (example: 1.95)")
    args = parser.parse_args()
    if args.dry_run:
        global DRY_RUN
        DRY_RUN = True
    return args

def verify_kps_exist(kps):
    if not os.path.exists(kps):
        exit(f"expected {kps} to be a checkout of ferrocene/problems, found nothing")

def verify_clean_checkout():
    if not git_output("branch", "--show-current"):
        print("error: You are not on a git branch.")
        raise SystemExit(1)
    if changed := modified_files():
        print("error: Your changes will be lost if you run this script! Please commit them.")
        if len(changed) < 5:
            print("these files are modified:")
            print('\n'.join(changed))
        else:
            print(len(changed), "files are modified")
        raise SystemExit(1)

def verify_unreleased_branch(release_branch):
    for f in ["ferrocene/version", "ferrocene/ci/channel"]:
        contents = git_output("show", release_branch + ":" + f)
        if contents != "rolling":
            raise ValueError(f"can only branch {release_branch} beta from a `rolling` version (got {contents})\n"
                             f"help: see {RELEASE_DOCS_URL}/index.html#channel-names")

## Destructive / writeable logic

def rewrite_release_notes(notes, ferrocene_version, upstream_version, old_upstream_version):
    skip_next = False
    with fileinput.input(files=(notes,), inplace=True, encoding="utf-8") as fd:
        for line in fd:
            if skip_next:
                skip_next = False
                continue
            if line == "Next Ferrocene release\n":
                skip_next = True
                print(f"Ferrocene {ferrocene_version}")
                print( "==========" + '='*len(ferrocene_version))
                continue
            print(line, end="")

    with open(notes, 'a') as fd:
        fd.write(
    """
    Rust changes
    ------------

    This release includes the following changes introduced by the upstream Rust
    project. Note that this changelog is maintained by upstream. The target support
    changes described here describe Rust's support levels, and have no correlation
    to the targets and platforms supported by Ferrocene.

    .. rust-changelog::\n
    """
        )
        fd.write(f"    :from: {old_upstream_version}\n")
        fd.write(f"    :to: {upstream_version}\n")

class UpdateKnownProblemsPR(AutomatedPR):
    def __init__(self, kp_repo, ferrocene_version, upstream_version):
        self.kp_repo = kp_repo
        self.ferrocene_version = ferrocene_version
        self.upstream_version = upstream_version

    def run(self):
        config = "src/config.yml"
        new_version_kps = f"src/versions/{self.ferrocene_version}.md"
        version_template = "version_template.md"

        with open(config, 'a') as fd:
            fd.write(" "*4 + f'release/{self.upstream_version}: "{self.ferrocene_version}"\n')
        shutil.copyfile(version_template, new_version_kps)

        with fileinput.input(files=(new_version_kps,), inplace=True, encoding="utf-8") as fd:
            for line in fd:
                line = line.replace("{ferrocene_version}", self.ferrocene_version)
                print(line, end="")

        git("add", config, new_version_kps)
        git("commit", "--message", f"Update KPs for {self.ferrocene_version} beta branch")

        return AutomationResult.SUCCESS

    def base_branch(self):
        return "main"

    def automation_name(self):
        return "beta-branch"

    def pr_title(self):
        return f"[{self.ferrocene_version} release] Add KPs for {self.upstream_version}"

    def pr_labels(self):
        return {"automation"}

    def pr_body(self, branch_name):
        return (
            f"Add Known Problems for {self.ferrocene_version} release.\n"
            f"See {RELEASE_DOCS_URL}/stable.html#add-version-to-known-problems.\n"
            f"\nThis PR was created by {CURRENT_USER}."
        )

    def on_failure(self, issue):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_title(self):
        None
    def error_issue_labels(self):
        None
    def error_issue_body(self):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_fixed_comment(self):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_repeated_comment(self):
        raise ValueError("something went wrong! don't know what, but something!")

class UpdateReleaseNotesPR(AutomatedPR):
    def __init__(self, ferrocene_version, upstream_version, old_upstream_version):
        self.ferrocene_version = ferrocene_version
        self.upstream_version = upstream_version
        self.old_upstream_version = old_upstream_version

    def run(self):
        template = "ferrocene/tools/release-tooling/release-notes-template.rst"
        dir = "ferrocene/doc/release-notes/src/"
        next = dir + "next.rst"
        released = dir + self.ferrocene_version + ".rst"

        # Avoid changing the current branch when we commit
        git("checkout", "--detach", "HEAD")

        os.rename(next, released)
        shutil.copyfile(template, next)
        rewrite_release_notes(released, self.ferrocene_version, self.upstream_version,
                              self.old_upstream_version)

        git("add", released, next)
        git("commit", "--message", f"Update release notes for {self.ferrocene_version} release")

        return AutomationResult.SUCCESS

    def base_branch(self):
        return "main"

    def automation_name(self):
        return "beta-branch"

    def pr_title(self):
        return f"[{self.ferrocene_version} release] Bump release notes"

    def pr_labels(self):
        return {"automation", f"backport:{self.upstream_version}"}

    def pr_body(self, branch_name):
        return (
            f"Update release notes for {self.ferrocene_version} release.\n"
            f"See {RELEASE_DOCS_URL}/stable.html#version-bump-release-notes.\n"
            f"\nThis PR was created by {CURRENT_USER}."
        )

    def on_failure(self, issue):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_title(self):
        None
    def error_issue_labels(self):
        None
    def error_issue_body(self):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_fixed_comment(self):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_repeated_comment(self):
        raise ValueError("something went wrong! don't know what, but something!")


class UpdateBetaBranchPR(AutomatedPR):
    def __init__(self, ferrocene_remote, ferrocene_version, upstream_version, main_branch_pr):
        self.ferrocene_remote = ferrocene_remote
        self.ferrocene_version = ferrocene_version
        self.upstream_version = upstream_version
        self.main_branch_pr = main_branch_pr

    def run(self):
        current_commit = git_output("rev-parse", "HEAD")
        checkout_branch(release_branch(self.ferrocene_remote, self.upstream_version))
        if self.main_branch_pr is not None:
            backport_one = "ferrocene/tools/backport/one.py"
            git("restore", "--source", current_commit, backport_one)
            cmd([backport_one, str(self.main_branch_pr)])
        with open("ferrocene/version", 'w') as fd:
            fd.write(self.ferrocene_version + '\n')
        with open("ferrocene/ci/channel", 'w') as fd:
            fd.write("beta\n")
        git("commit", "ferrocene/version", "ferrocene/ci/channel",
            "--message", f"Branch to {self.ferrocene_version} beta")

        return AutomationResult.SUCCESS

    def base_branch(self):
        return "release/" + self.upstream_version

    def automation_name(self):
        return "beta-branch"

    def pr_title(self):
        return f"[{self.upstream_version}] beta branch to {self.ferrocene_version}"

    def pr_labels(self):
        return {"automation", f"backport:{self.upstream_version}"}

    def pr_body(self, branch_name):
        return (
            f"Update release notes for {self.ferrocene_version} release.\n"
            f"See {RELEASE_DOCS_URL}/stable.html#version-bump-release-notes.\n"
            f"\nThis PR was created by {CURRENT_USER}."
        )

    def on_failure(self, issue):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_title(self):
        None
    def error_issue_labels(self):
        None
    def error_issue_body(self):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_fixed_comment(self):
        raise ValueError("something went wrong! don't know what, but something!")
    def error_issue_repeated_comment(self):
        raise ValueError("something went wrong! don't know what, but something!")

def main():
    args = parse_args()
    verify_kps_exist(args.known_problems_checkout)
    verify_unreleased_branch(release_branch(args.ferrocene_remote, args.upstream_version))
    verify_clean_checkout()

    # This is a hack, but otherwise we have to mess with the AutomatedPR code and it's
    # just not worth it.
    cwd = os.getcwd()
    os.chdir(args.known_problems_checkout)
    kps_pr = UpdateKnownProblemsPR(args.known_problems_checkout, args.ferrocene_version,
                                   args.upstream_version) \
                        .create(args.dry_run, repo=KNOWN_PROBLEMS_REPO,
                                origin=f"https://github.com/{KNOWN_PROBLEMS_REPO}")
    os.chdir(cwd)

    main_branch_pr = UpdateReleaseNotesPR(args.ferrocene_version, args.upstream_version,
                                          args.old_upstream_version) \
                        .create(args.dry_run, origin=FERROCENE_URL)
    beta_pr = UpdateBetaBranchPR(args.ferrocene_remote, args.ferrocene_version,
                                 args.upstream_version, main_branch_pr) \
                        .create(args.dry_run, origin=FERROCENE_URL)

    print(f"Opened: https://github.com/{KNOWN_PROBLEMS_REPO}/pulls/{kps_pr} -> main")
    print(f"        {FERROCENE_PULLS_URL}/{main_branch_pr} -> main")
    print(f"        {FERROCENE_PULLS_URL}/{beta_pr} -> release/{args.upstream_version}")
    print("You are NOT ready to sign the release. You still need to:")
    print("- backport any relevant PRs: "
          "https://github.com/ferrocene/ferrocene/pulls?q=is%3Apr+label%3Abackport%3A"
          + args.upstream_version)
    print("- perform release validation: https://public-docs.ferrocene.dev/main/qualification/plan/validation.html#release-validation")

if __name__ == '__main__':
    main()

