#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["ignorelib ~= 0.3", "more-itertools ~= 10.8", "automations-common"]
#
# [tool.uv.sources]
# automations-common = { path = "../automations-common", editable = true }
# ///

import argparse
import os
import re
from collections import defaultdict, namedtuple
from pathlib import Path

from automations_common import cmd_capture
from ignorelib import IgnoreFilterManager
from more_itertools import peekable

had_error = False


def error(*messages):
    print("error:", *messages)
    global had_error
    had_error = True


parser = argparse.ArgumentParser(
    prog="check-fls-annotations", description="Check FLS annotations in tests"
)
args = parser.parse_args()


class DiffKind:
    ADDED = "A"
    MODIFIED = "M"
    DELETED = "D"
    RENAMED = "R"


DIFF_NAMES = {
    DiffKind.ADDED: "added",
    DiffKind.MODIFIED: "modified",
    DiffKind.DELETED: "deleted",
    DiffKind.RENAMED: "renamed",
}


base_commit = cmd_capture("git log -n1 --author=bors@rust-lang.org --pretty=%H".split())

diff_tests = f"git diff -U0 --no-prefix --diff-filter=M {base_commit} -- tests/".split()
test_changes = peekable(cmd_capture(diff_tests).splitlines())

file = None
lineno = 0
changes: list[DiffKind, str] = []
test_changemap = {}
missing_annotations = {}
annotations = {}

Span = namedtuple("Span", ["file", "line"])

for line in test_changes:
    if line.startswith("diff --git"):
        # Record the file we just finished parsing.
        if changes:
            assert file, "saw changes for unknown file"
            test_changemap[file] = changes
        # Now parse the header for the new file.
        file, changes, lineno = line.split()[2], [], 0
        for _ in range(4):
            next(test_changes)  # discard `index ...`
        continue

    lineno += 1
    kind, change = line[0], line[1:]
    if not change or change == "//" or change.startswith("//@ ignore-ferrocene.facade"):
        continue

    if re.search(r"^\s*// ferrocene-annotations: ", change):
        span = Span(file, lineno)
        id = next(reversed(change.split(" ")))
        if id.startswith("um_rustc"):
            annotations[id] = span, ""
            continue
        # Next line is the section name it corresponds to.
        section = test_changes.peek()
        if not ((section[0] == "+") and section[1:].lstrip().startswith("//")):
            missing_annotations[span] = id
        else:
            annotations[id] = span, section.split(" ", 1)[1]
            next(test_changes)  # discard section
        continue

    changes.append([kind, change])

if missing_annotations:
    error(
        f"found {len(missing_annotations)} ferrocene-annotations not followed by section name!"
    )
    for span, annotation in missing_annotations.items():
        print("%s:%d" % span, annotation)


# load all section titles
def load_sections(fd, sections):
    for line in fd:
        if line.startswith(".. _fls"):
            # Get id, truncating trailing colon.
            id = line.strip().split(".. _", 1)[1][:-1]
            n = next(fd)
            assert n.strip() == "", n
            sections[id.lower()] = next(fd).strip().replace("`", "")


def validate_annotations():
    sections = {}
    for dir, _subdirs, files in os.walk("ferrocene/doc/specification"):
        for file in files:
            if file.endswith(".rst"):
                with open(dir + "/" + file) as fd:
                    load_sections(fd, sections)

    cli_ids = set()
    matrix = "build/host/doc/qualification/traceability-matrix.html"
    try:
        with open(matrix) as fd:
            for line in fd:
                if m := re.search(r'id="(um_rustc_[^"]*)"', line):
                    cli_ids.add(m.group(1))
    except FileNotFoundError:
        exit(
            f"error: {matrix} not found. try running './x build traceaqbility-matrix'."
        )

    for id, (span, name) in annotations.items():
        if id.startswith("um_rustc_"):
            if id not in cli_ids:
                error("Unknown CLI spec id:", id)
        elif id not in sections:
            error(f"Unknown section '{id}'!")
        elif name.lower() != sections[id].lower():
            error(
                "Incorrect section name on %s:%d:" % span,
                f"Expected '{sections[id]}', found '{name}'",
            )


print("Validating Ferrocene annotations against traceability matrix")
validate_annotations()
if not had_error:
    print("All annotations match the spec!")

exit(had_error)
