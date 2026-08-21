#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = []
#
# [tool.uv.sources]
# utils = { path = "utils", editable = true }
# ///

# Check that Github pull request titles start with a reference to a Clickup ticket. Ticket IDs
# are a sequence of lowercase ASCII letters and digits.
# Example of expected format: "[869ed4uxf] Fix bug in ..."

import re
import sys
import os


def is_valid_pr_title(pr_title):
    ticket_reference = re.match(r"^\[[0-9a-z]+\]", pr_title)
    return ticket_reference is not None


if __name__ == "__main__":
    # Passing the PR title as an environment variable instead of an argument prevents
    # potential command-injection vulnerabilities. For details, see:
    # https://docs.github.com/en/actions/reference/security/secure-use#use-an-intermediate-environment-variable
    pr_title = os.environ["PR_TITLE"]
    if not is_valid_pr_title(pr_title):
        print(
            "Error: Pull request title does not start with a valid Clickup ticket reference",
            file=sys.stderr,
        )
        sys.exit(1)
