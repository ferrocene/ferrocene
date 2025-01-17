#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["requests ~= 2.32", "automations-common"]
#
# [tool.uv.sources]
# automations-common = { path = "../automations-common", editable = true }
# ///

import os
import requests
import sys

from automations_common import AutomatedPR, AutomationResult


TEMP_BRANCH = "backport-temp--do-not-use-for-real-code"


class BackportAllPR(AutomatedPR):
    def __init__(self, repo, label, target):
        self.__repo = repo
        self.__label = label
        self.__target = target
        self.__backported_pull_requests = []

    def run(self):
        prs = self.__prs_with_label(self.__label)
        if not prs:
            return AutomationResult.NO_CHANGES

        current_branch = self.cmd_capture(["git", "branch", "--show-current"])
        base_branch = f"{self.origin}/{self.__target}"

        # When a backport failure happens, the script leaves git in the
        # temporary branch. If we were to delete the temporary branch to create
        # a new one from scratch, it would fail as git prevents deleting the
        # current branch. To handle that, if we're already in the temporary
        # branch we just reset to the base we want.
        if current_branch == TEMP_BRANCH:
            self.cmd(["git", "reset", "--hard", base_branch])
        else:
            self.cmd(["git", "branch", "-D", TEMP_BRANCH], check=False)
            self.cmd(
                [
                    "git",
                    "checkout",
                    "-b",
                    TEMP_BRANCH,
                    base_branch,
                ]
            )

        for pr in prs:
            pr = pr["number"]
            print(f"backporting #{pr}")

            result = self.cmd(
                [f"{self.repo_root}/ferrocene/tools/backport/one.py", str(pr)],
                check=False,
            )
            if result.returncode == 0:
                self.__backported_pull_requests.append(pr)
            else:
                self.cmd(["git", "rebase", "--abort"], check=False)
                self.cmd(["git", "checkout", TEMP_BRANCH])
                self.__mark_as_manual(pr)

        if self.__backported_pull_requests:
            return AutomationResult.SUCCESS
        else:
            return AutomationResult.NO_CHANGES

    def __prs_with_label(self, label):
        QUERY = """
            query($owner: String!, $name: String!, $label: String!, $cursor: String) {
                repository(owner: $owner, name: $name) {
                    pullRequests(labels: [$label], states: [MERGED], first: 100, after: $cursor) {
                        nodes { number, merged, mergedAt }
                        pageInfo { endCursor, hasNextPage }
                    }
                }
            }
        """

        owner, name = self.__repo.split("/")
        result = []
        cursor = None
        while True:
            response = self.http.post(
                "https://api.github.com/graphql",
                json={
                    "query": QUERY,
                    "variables": {
                        "owner": owner,
                        "name": name,
                        "label": label,
                        "cursor": cursor,
                    },
                },
            )
            response.raise_for_status()
            data = response.json()["data"]["repository"]["pullRequests"]
            for pr in data["nodes"]:
                result.append(pr)

            if data["pageInfo"]["hasNextPage"]:
                cursor = data["pageInfo"]["endCursor"]
            else:
                break

        return list(sorted(result, key=lambda pr: pr["mergedAt"]))

    def __mark_as_manual(self, pr):
        response = self.http.post(
            f"https://api.github.com/repos/{self.__repo}/issues/{pr}/labels",
            json={"labels": ["backport:manual"]},
        )
        response.raise_for_status()

    def base_branch(self):
        return self.__target

    def automation_name(self):
        return "backport"

    def pr_title(self):
        return f"Backport to `{self.__target}`"

    def pr_labels(self):
        return {"automation", "backport:never"}

    def pr_body(self, branch_name):
        if len(self.__prs_with_label("backport:manual")):
            url = f"https://github.com/{self.__repo}/pulls?q=is%3Apr+label%3Abackport%3Amanual"
            manual_message = (
                "\n\n"
                "Note that the automation failed to backport some PRs automatically. "
                f"Check out the [backport:manual]({url}) for a list of them."
            )
        else:
            manual_message = ""

        prs = "\n".join(f"* #{pr}" for pr in self.__backported_pull_requests)
        intro = f"This PR backports the following PRs to the `{self.__target}` branch:"
        return f"{intro}\n\n{prs}{manual_message}"

    def error_issue_title(self):
        return "ERROR_ISSUES_NOT_USED_BY_BACKPORT_ALL"

    def error_issue_labels(self):
        return {"ERROR_ISSUES_NOT_USED_BY_BACKPORT_ALL"}

    def error_issue_body(self):
        raise RuntimeError("error issues not used by backport/all.py")

    def error_issue_fixed_comment(self, pull_request_url):
        raise RuntimeError("error issues not used by backport/all.py")

    def error_issue_repeated_comment(self):
        raise RuntimeError("error issues not used by backport/all.py")


def list_backport_labels(repo):
    http = requests.Session()
    http.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"

    backport_labels = []
    url = f"https://api.github.com/repos/{repo}/labels?per_page=100"
    while url is not None:
        resp = http.get(url)
        resp.raise_for_status()

        for label in resp.json():
            if label["name"].startswith("backport:1."):
                backport_labels.append(label["name"])

        if "next" in resp.links:
            url = response.links["next"]["url"]
        else:
            url = None

    return backport_labels


if __name__ == "__main__":
    repo = os.environ["GITHUB_REPOSITORY"]
    labels = list_backport_labels(repo)

    for label in labels:
        print(f"==> backporting PRs with label {label}")
        pr = BackportAllPR(repo, label, f"release/{label.removeprefix('backport:')}")
        pr.create()
