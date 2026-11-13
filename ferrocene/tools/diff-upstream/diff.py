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
from collections import defaultdict
from pathlib import Path

from automations_common import cmd_capture
from ignorelib import IgnoreFilterManager

had_error = False


def error(*messages):
    print("error:", *messages)
    global had_error
    had_error = True


parser = argparse.ArgumentParser(
    prog="diff-upstream", description="Diff Ferrocene from upstream Rust project"
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

root_dir = Path(cmd_capture("git rev-parse --show-toplevel".split()))

ignored = []
with open(root_dir / ".gitattributes") as fd:
    for line in fd:
        parts = line.split()
        if parts and parts[-1] == "ferrocene-avoid-pulling-from-upstream":
            ignored.append(parts[0])

filterer = IgnoreFilterManager.build(path=root_dir, global_patterns=ignored)

all_changed = cmd_capture(
    f"git -c color.ui=never diff {base_commit} --name-status".split()
).splitlines()
changemap = defaultdict(list)

IGNORED_ADDITIONS = [
    ".dockerignore",
    ".python-version",
    "bors.toml",
]
IGNORED_ADDITIONS_DIRS = [
    ".github",
    ".circleci",
    "compiler/rustc_target/src/spec/targets",
    "tests/run-make/symbol-report",
]

for line in all_changed:
    kind, rest = line[0], line[1:]
    match kind:
        case DiffKind.DELETED:
            path = rest.lstrip()
            if filterer.is_ignored(path):
                continue
            val = path
        case DiffKind.ADDED:
            path = rest.lstrip()
            if (
                # Assume files in ferrocene/ directories are always ok
                "ferrocene" in path
                or path in IGNORED_ADDITIONS
                or any(path.startswith(dir) for dir in IGNORED_ADDITIONS_DIRS)
            ):
                continue
            val = path
        case DiffKind.MODIFIED:
            path = rest.lstrip()
            if path.startswith("src/etc/completions"):
                continue
            val = path
        case DiffKind.RENAMED:
            similarity, old_name, new_name = rest.split("\t")
            val = f"{old_name} -> {new_name} ({int(similarity)}%)"
        case _:
            raise ValueError(f"Don't know how to handle diff kind {kind}")
    changemap[kind].append(val)


for kind in ["A", "D", "R", "M"]:
    if changemap[kind]:
        print(f"\nPrinting all {DIFF_NAMES[kind]} files")
    for p in changemap[kind]:
        print(f"{kind}\t{p}")

exit(had_error)
