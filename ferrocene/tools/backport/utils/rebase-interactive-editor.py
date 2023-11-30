#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script serves as a `git rebase --interactive` editor, tweaking the todo
# list the rebase will use. We need to tweak the todo list to make sure the
# `update-last-message.py` script is executed at the end of the rebase, and to
# set the correct branch at the end of the rebase.

import sys
import os


def main():
    if len(sys.argv) != 2:
        raise RuntimeError("this program only accepts the path to the todo file")
    path = sys.argv[1]
    pr_number = os.environ["FERROCENE_PR_NUMBER"]
    current_branch = os.environ["FERROCENE_CURRENT_BRANCH"]

    utils_dir = os.path.abspath(os.path.dirname(__file__))

    with open(path, "a") as f:
        f.write(f"exec {utils_dir}/update-last-message.py {pr_number}\n")

        # `git rebase --onto` leaves you into a detached HEAD state when the
        # rebase finishes, which is annoying if you then need to do more work
        # on that branch. The next two execs **HAVE TO BE LAST**, and will
        # switch to the initial branch right before the rebase ends.
        if current_branch:
            f.write(f"exec git branch -f {current_branch}\n")
            f.write(f"exec git checkout {current_branch}\n")


if __name__ == "__main__":
    main()
