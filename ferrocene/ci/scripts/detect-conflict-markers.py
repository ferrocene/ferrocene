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
from dataclasses import dataclass


START = "<<<<<<< "
MIDDLE = "======="
END = ">>>>>>> "

# git doesn't include any marker when a file is deleted by one side of the
# merge while the other side made changes to it. To ensure those are still
# marked as conflict, the pull-upstream tool adds custom markers to them. We
# should detect those as well.
CUSTOM_DELETE_MARKER = "<<<PULL-UPSTREAM>>> file deleted by "

EXCEPTIONS = {
    # Used in multiple rustc tests as a test case for conflict detection.
    "<<<<<<< HEAD //~ ERROR encountered diff marker",
}


def files_with_possible_conflict_markers():
    # git grep automatically filters out submodules.
    lines = subprocess.run(
        ["git", "grep", "-l", "^<<<"],
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
                yield ConflictMarker(file=file, start_line=start_line, end_line=num)
                start_line = None
                expect = START
        elif line.startswith(CUSTOM_DELETE_MARKER):
            yield CustomDeleteMarker(file=file)


def main():
    found_conflicts = False
    for file in files_with_possible_conflict_markers():
        for conflict in conflict_markers_in_file(file):
            print(conflict.repr())
            found_conflicts = True

    if found_conflicts:
        exit(1)


@dataclass
class ConflictMarker:
    file: str
    start_line: int
    end_line: int

    def repr(self):
        return f"{self.file}: conflict between lines {self.start_line} " \
               f"and {self.end_line}"


@dataclass
class CustomDeleteMarker:
    file: str

    def repr(self):
        return f"{self.file}: file deleted by one side of the merge"


if __name__ == "__main__":
    main()
