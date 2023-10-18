#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# To avoid having to run it locally, and to make conflict resolution more
# visible, the automation to pull changes from upstream commits the conflict
# markers when a merge conflict occurs, leaving it to the reviewer to clone the
# branch and fix the conflicts.
#
# To alert the reviewer where the conflict markers are, and to block CI
# whenever those are found, this script lists all the conflict markers in the
# repository, and errors out if some are found.

import subprocess


START = "<<<<<<< "
MIDDLE = "======="
END = ">>>>>>> "

EXCEPTIONS = {
    # Used in multiple rustc tests as a test case for conflict detection.
    "<<<<<<< HEAD //~ ERROR encountered diff marker",
}


def files_with_possible_conflict_markers():
    # git grep automatically filters out submodules.
    lines = subprocess.run(
        ["git", "grep", "-l", "^<<<<<<<"],
        check=True,
        stdout=subprocess.PIPE,
        text=True,
    )
    for line in lines.stdout.split("\n"):
        if not line:
            continue
        yield line


def conflict_markers_in_file(file):
    with open(file) as f:
        contents = f.read()

    expect = START
    start_line = None
    for num, line in enumerate(contents.split("\n")):
        num += 1  # Humans interpret lines as starting with 1
        if line.startswith(expect):
            if line in EXCEPTIONS:
                continue
            elif expect == START:
                start_line = num
                expect = MIDDLE
            elif expect == MIDDLE:
                expect = END
            elif expect == END:
                yield (start_line, num)
                start_line = None
                expect = START


def main():
    found_conflicts = False
    for file in files_with_possible_conflict_markers():
        for start_line, end_line in conflict_markers_in_file(file):
            print(f"{file}: conflict between lines {start_line} and {end_line}")
            found_conflicts = True

    if found_conflicts:
        exit(1)


if __name__ == "__main__":
    main()
