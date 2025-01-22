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

# Script to automatically create pull requests for pulling from upstream, and
# to open issues if the pulling fails due to a merge conflict. The script is
# meant to be run on GitHub Actions.
#
# Required enviroment variables:
# - `GITHUB_TOKEN`: API token with access to the repo contents, issues and RPs
# - `GITHUB_REPOSITORY`: name of the GitHub repository to run this script on

from automations_common import AutomatedPR, AutomationResult
from dataclasses import dataclass
from typing import Optional
import generate_pr_body
import os
import sys
import tempfile


class PullUpstreamPR(AutomatedPR):
    def __init__(self, upstream_branch, base_branch):
        self._upstream_branch = upstream_branch
        self._base_branch = base_branch

        # The pull.sh script records warnings that should be displayed to the user. They will be
        # stored by the script in this file, which we can then read. The file is open in read mode
        # since the writing will be done by push.sh.
        self._warnings_file = tempfile.NamedTemporaryFile(mode="r", delete_on_close=False)

        super().__init__()

    def run(self):
        """
        Shell out to the script actually pulling from upstream.
        """
        res = self.cmd(
            [
                os.path.join(self.repo_root, "ferrocene/tools/pull-upstream/pull.sh"),
                self._upstream_branch,
                self._base_branch,
            ],
            env={
                **os.environ,
                "PULL_WARNINGS_FILE": self._warnings_file.name,
            },
            check=False,
        )
        if res.returncode == 0:
            return AutomationResult.SUCCESS
        elif res.returncode == 42:
            return AutomationResult.NO_CHANGES
        else:
            self.cmd(["git", "merge", "--abort"], check=False)
            return AutomationResult.FAILURE

    def automation_name(self):
        return f"pull-upstream-{self._upstream_branch}"

    def pr_title(self):
        return f"Automated pull from upstream `{self._upstream_branch}`"

    def pr_labels(self):
        return {"automation", "backport:never"}

    def pr_body(self, branch_name):
        message = ""

        warnings = self._warnings_file.read().strip().split("\n")
        if warnings:
            message += ":warning: The automation reported these warnings: :warning:\n\n"
            for warning in warnings:
                message += f"* {warning}"
            message += "\n"

        message += "This PR pulls the following changes from the upstream repository:\n\n"
        message += generate_pr_body.render_changes(self.origin, self.base_branch(), branch_name)
        return message

    def error_issue_title(self):
        return f"Failed to pull from upstream `{self._upstream_branch}`"

    def error_issue_labels(self):
        return {"automation"}

    def error_issue_body(self):
        return """
While trying to pull the latest changes from the upstream `{{upstream_branch}}` \
branch, the automation failed. The automation will not open any more pull \
requests pulling from upstream until the merge is fixed.

Check the log on GitHub Actions for more details on the cause of the failure.
""".replace(
            "{{upstream_branch}}", self._upstream_branch
        )

    def error_issue_fixed_comment(self, pull_request_url):
        return f"""
The automation successfully pulled the latest changes from upstream in \
{pull_request_url}! Closing this issue.
"""

    def error_issue_repeated_comment(self):
        return "The automation failed to pull the latest changes from upstream again."

    def base_branch(self):
        return self._base_branch


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} <upstream-branch> <base-branch>")
        exit(1)
    PullUpstreamPR(sys.argv[1], sys.argv[2]).create()
