#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script adds the backported PR number to the last commit message. It's
# not meant to be invoked directly, but only as part of an interactive rebase.

import os
import subprocess
import sys


TRAILER_NAME = "Ferrocene-backport-of"


def get_current_commit_message():
    return subprocess.run(
        ["git", "show", "--pretty=format:%B", "--no-patch"],
        stdout=subprocess.PIPE,
        text=True,
        check=True,
    ).stdout


def amend_commit_message(new_message):
    p = subprocess.Popen(["git", "commit", "--amend", "-F", "-"], stdin=subprocess.PIPE)
    p.stdin.write(new_message.encode("utf-8"))
    p.stdin.close()
    if p.wait() != 0:
        raise RuntimeError("amending the commit message failed")


def main(pr_number):
    current_message = get_current_commit_message()
    repo = os.environ.get("GITHUB_REPOSITORY", "")
    fixed_message = current_message + f"\n{TRAILER_NAME}: {repo}#{pr_number}\n"
    amend_commit_message(fixed_message)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise RuntimeError("expected one argument (pr number)")
    main(sys.argv[1])
