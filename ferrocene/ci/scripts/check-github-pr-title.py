#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

# Check that Github pull request titles contain at least one reference to a Clickup ticket.
# Ticket IDs are the last part of the ticket URL, and are always formed from lowercase ASCII letters
# and digits.
#
# For example, PRs related to `https://app.clickup.com/.../869ed4uxf` might have titles like:
#
#     [869ed4uxf] Fix bug in ... (normal PRs to the main branch)
#     WIP: [869ed4uxf] Fix bug in ... (or other similar labels)
#     [1.99] [869ed4uxf] Fix bug in ... (for PRs to a specific release branch)

import os
import re
import sys


def is_automated_pr(pr_source_branch):
    return pr_source_branch.startswith("automation/")


def is_valid_pr_title(pr_title):
    ticket_reference = re.match(r"\[[0-9a-z]+\]", pr_title)
    return ticket_reference is not None


if __name__ == "__main__":
    # Passing the PR title as an environment variable instead of an argument prevents
    # potential command-injection vulnerabilities. For details, see:
    # https://docs.github.com/en/actions/reference/security/secure-use#use-an-intermediate-environment-variable
    pr_title = os.environ["PR_TITLE"]

    pr_source_branch = os.environ["GITHUB_HEAD_REF"]

    print(f"pr_title = {pr_title!r}")
    print(f"pr_source_branch = {pr_source_branch!r}")

    if is_automated_pr(pr_source_branch):
        print("Automated branch, so Clickup ticket rule does not apply")
        sys.exit(0)

    if not is_valid_pr_title(pr_title):
        print(
            "Error: Pull request title does not contain a valid Clickup ticket reference.\n"
            "The expected format is the last part of the ticket URL, in square brackets.\n"
            "E.g. a PR for `https://app.clickup.com/.../869ed4uxf` should contain `[869ed4uxf]` in the title",
            file=sys.stderr,
        )
        sys.exit(1)
