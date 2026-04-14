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
import re
import os
from collections import defaultdict, namedtuple
from pathlib import Path

from ignorelib import IgnoreFilterManager
from more_itertools import peekable

from automations_common import cmd_capture

had_error = False
def error(msg):
    print('error:', msg)
    global had_error
    had_error = True

parser = argparse.ArgumentParser(prog='diff-upstream', description="Diff Ferrocene from upstream Rust project")
parser.add_argument('--hide-diff', action='store_true', help="Only test "
                    "ferrocene-annotations; don't run the diff. Implies --test.")
parser.add_argument('--test', action='store_true', help="Test ferrocene-annotations")
args = parser.parse_args()

if args.hide_diff:
    args.test = True

#### Diff all test annotations. ####

base_commit = cmd_capture("git log -n1 --author=bors@rust-lang.org --pretty=%H".split())

diff_tests = f"git diff -U0 --no-prefix --diff-filter=M {base_commit} -- tests/".split()
test_changes = peekable(cmd_capture(diff_tests).splitlines())

file = None
lineno = 0
changes: list[DiffKind, str] = []
test_changemap = {}
missing_annotations = {}
annotations = {}

Span = namedtuple('Span', ['file', 'line'])

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
    if (not change or change == '//'
            or change.startswith('//@ ignore-ferrocene.facade')):
        continue

    if re.search('^\s*// ferrocene-annotations: ', change):
        span = Span(file, lineno)
        id = next(reversed(change.split(' ')))
        if id.startswith('um_rustc'):
            annotations[id] = span, ''
            continue
        # Next line is the section name it corresponds to.
        section = test_changes.peek()
        if not section.startswith('+// '):
            missing_annotations[span] = id
        else:
            annotations[id] = span, section.split(' ', 1)[1]
            next(test_changes)  # discard section
        continue

    changes.append([kind, change])

if missing_annotations:
    error(f'found {len(missing_annotations)} ferrocene-annotations not followed by section name!')
    for span, annotation in missing_annotations.items():
        print('%s:%d' % span, annotation)

# load all section titles
def load_sections(fd, sections):
    for line in fd:
        if line.startswith('.. _fls'):
            # Get id, truncating trailing colon.
            id = line.strip().split('.. _', 1)[1][:-1]
            n = next(fd)
            assert n.strip() == '', n
            sections[id.lower()] = next(fd).strip().replace('`', '')

def validate_annotations():
    sections = {}
    for dir, _subdirs, files in os.walk('ferrocene/doc/specification'):
        for file in files:
            if file.endswith('.rst'):
                with open(dir + '/' + file) as fd:
                    load_sections(fd, sections)

    cli_ids = set()
    with open('build/host/doc/qualification/traceability-matrix.html') as fd:
        for line in fd:
            if m := re.search(r'id="(um_rustc_[^"]*)"', line):
                cli_ids.add(m.group(1))

    for id, (span, name) in annotations.items():
        if id.startswith('um_rustc_'):
            if id not in cli_ids:
                error("Unknown CLI spec id:", id)
        elif id not in sections:
            error(f"Unknown section '{id}'!")
        elif name.lower() != sections[id].lower():
            error("Incorrect section name on %s:%d:" % span,
                f"Expected '{sections[id]}', found '{name}'")

if args.test:
    print("Validating Ferrocene annotations")
    validate_annotations()
    if not had_error:
        print("All annotations match the spec!")

if args.hide_diff:
    exit(had_error)

#### Diff all other changes. ####

root_dir = Path(cmd_capture("git rev-parse --show-toplevel".split()))

ignored = []
with open(root_dir / ".gitattributes") as fd:
    for line in fd:
        parts = line.split()
        if parts and parts[-1] == "ferrocene-avoid-pulling-from-upstream":
            ignored.append(parts[0])

filterer = IgnoreFilterManager.build(path=root_dir, global_patterns=ignored)

all_changed = cmd_capture(f"git -c color.ui=never diff {base_commit} --name-status".split()).splitlines()
changemap = defaultdict(list)

class DiffKind:
    ADDED = 'A'
    MODIFIED = 'M'
    DELETED = 'D'
    RENAMED = 'R'

DIFF_NAMES = {
        DiffKind.ADDED: 'added',
        DiffKind.MODIFIED: 'modified',
        DiffKind.DELETED: 'deleted',
        DiffKind.RENAMED: 'renamed',
}

IGNORED_ADDITIONS = [
    ".dockerignore",
    ".python-version",
    "bors.toml",
    "src/bootstrap/defaults/bootstrap.ferrocene-dist.toml",
    "src/bootstrap/src/core/config/toml/ferrocene.rs",
    "src/tools/compiletest/src/ferrocene_annotations.rs",
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
            # Assume files in ferrocene/ directories are always ok
            if (path.startswith("ferrocene/") or path.endswith("/ferrocene-annotations")
                    or path.endswith('/ferrocene.rs' )
                    or path.startswith('.github') or path.startswith(".circleci")
                    or path.startswith("compiler/rustc_target/src/spec/targets/")
                    or path.startswith("tests/run-make/symbol-report/")
                    or '/ferrocene/' in path or '/ferrocene_test.rs' in path
                    or path in IGNORED_ADDITIONS):
                continue
            val = path
        case DiffKind.MODIFIED:
            path = rest.lstrip()
            if path.startswith("src/etc/completions"):
                continue
            val = path
        case DiffKind.RENAMED:
            val = rest.split()
        case _:
            raise ValueError(f"Don't know how to handle diff kind {kind}")
    changemap[kind].append(val)


for kind in ['A', 'D', 'R']:
    if changemap[kind]:
        print(f"\nPrinting all {DIFF_NAMES[kind]} files")
    for p in changemap[kind]:
        print(f'{kind}\t{p}')

print("\nPrinting all changed tests")
for f, changes in test_changemap.items():
    print(f)
    print('\n'.join(kind + change for kind, change in changes))

print(f"\ngit diff {base_commit} --name-status --diff-filter=M -I'#\[ferrocene::' -- ':!tests/' ':!.github' ':!**/.github/**' ':!ferrocene'")

for p in changemap['M']:
    if p.startswith('tests/'):
        continue
    print(f'M\t{p}')

exit(had_error)
