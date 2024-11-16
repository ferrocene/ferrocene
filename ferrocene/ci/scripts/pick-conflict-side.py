#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# To avoid having to run it locally, and to make conflict resolution more
# visible, the automation to pull changes from upstream commits the conflict
# markers when a merge conflict occurs, leaving it to the reviewer to clone the
# branch and fix the conflicts.
#
# This script fixes a conflict by picking one side of it.

import subprocess
import argparse
import os.path


START = "<<<<<<< "
MIDDLE = "======="
END = ">>>>>>> "

# git doesn't include any marker when a file is deleted by one side of the
# merge while the other side made changes to it. To ensure those are still
# marked as conflict, the pull-upstream tool adds custom markers to them. We
# should detect those as well.
CUSTOM_DELETE_MARKER = "<<<PULL-UPSTREAM>>> file deleted "

EXCEPTIONS = {
    # Used in multiple rustc tests as a test case for conflict detection.
    "<<<<<<< HEAD //~ ERROR encountered diff marker",
    # Introduced in https://github.com/rust-lang/rust/pull/116712.
    ">>>>>>> 7a4f13c blah blah blah",
}


def pick_side_in_file(file, ours):
    with open(file) as f:
        contents = f.readlines()

    expect = START
    start_line = None
    middle_line = None

    nums_to_delete = set()

    for num, line in enumerate(contents):
        if line.startswith(expect):
            if line in EXCEPTIONS:
                continue
            elif expect == START:
                start_line = num
                expect = MIDDLE
            elif expect == MIDDLE:
                middle_line = num
                expect = END
            elif expect == END:
                if ours:
                    # delete the start marker
                    nums_to_delete.add(start_line)
                    # delete their side
                    nums_to_delete.update(range(middle_line, num + 1))
                else:
                    # delete our side
                    nums_to_delete.add(range(start_line, middle_line + 1))
                    # delete the end marker
                    nums_to_delete.update(num)
                start_line = None
                expect = START
        elif line.startswith(CUSTOM_DELETE_MARKER):
            if ours:
                # delete the custom marker
                nums_to_delete.add(num)
            else:
                # delete the whole file
                nums_to_delete.update(range(0, len(contents)))

    lines_to_keep = []

    for num, line in enumerate(contents):
        if num not in nums_to_delete:
            lines_to_keep.append(line)

    with open(file, "w") as f:
        f.writelines(lines_to_keep)


def main(files, ours):
    for file in files:
        pick_side_in_file(file, ours)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="+")
    parser.add_argument("--ours", action="store_true", help="pick the ferrocene change")
    parser.add_argument("--theirs", action="store_true", help="pick the upstream change")
    args = parser.parse_args()

    if args.ours == args.theirs:
        print("exactly one of `--ours` or `--theirs` must be passed")

    main(args.files, args.ours)

