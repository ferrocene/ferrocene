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

from io import StringIO


parser = argparse.ArgumentParser(
    prog="check-fls-annotations", description="Check FLS annotations in tests"
)
parser.add_argument(
    "--bless", action="store_true", help="Automatically fix issues where possible"
)
args = parser.parse_args()


had_any_error = False
had_unfixable_error = False


def error(*messages, fixable=True):
    global had_any_error
    global had_unfixable_error

    had_any_error = True

    if fixable and args.bless:
        # Don't print an error message in this case
        pass
    else:
        print("error:", *messages)
        had_unfixable_error = True


# Wrapper around a file which tracks line numbers and allows peeking at the next line
# without advancing.
class FileWrapper:
    def __init__(self, data):
        self.fd = data
        self.lineno = 0
        self.pending_line = None

    def __iter__(self):
        return self

    def __next__(self):
        if self.pending_line is None:
            line = self.fd.readline()

            if line == "":
                raise StopIteration

            self.lineno += 1
            return (self.lineno, line)

        else:
            line = self.pending_line
            self.lineno += 1
            self.pending_line = None

            return (self.lineno, line)

    def next(self):
        return self.__next__()

    def peek(self):
        if self.pending_line is None:
            next_line = self.fd.readline()
            if next_line == "":
                return (self.lineno + 1, None)
            else:
                self.pending_line = next_line
                return (self.lineno + 1, next_line)

        else:
            return (self.lineno + 1, self.pending_line)


def load_sections(fd, file, sections):
    wrapper = FileWrapper(fd)
    fixed = StringIO()

    for lineno, line in wrapper:
        if line.startswith(".. _fls"):
            # Get id, truncating trailing colon.
            id = line[4:].rstrip()  # Remove the ".. _" part, keep the rest

            if not id.endswith(":"):
                error(f"FLS ID at {file}:{lineno} is not followed by a ':'")
            else:
                id = id[:-1]

            if not id.islower():
                error(f"Non-lowercase FLS ID definition at {file}:{lineno}")
                id = id.lower()  # to allow further processing

            (_, n) = wrapper.peek()
            if n is None or n.strip() != "":
                error(f"FLS ID at {file}:{lineno} is not followed by a blank line")
            else:
                wrapper.next()  # Skip the line we peeked at

            (_, title_line) = wrapper.next()
            sections[id] = title_line.strip().replace("`", "")

            # Normalize formatting
            fixed.write(f".. _{id}:\n")
            fixed.write("\n")
            fixed.write(title_line)
        else:
            fixed.write(line)

    return fixed.getvalue()


def validate_annotations_in_file(fd, file, sections, cli_ids):
    wrapper = FileWrapper(fd)
    fixed = StringIO()

    for lineno, line in wrapper:
        if line.lstrip().startswith("// ferrocene-annotations: "):
            indent = line.split("//", maxsplit=1)[0]
            x = line.strip().split()

            if len(x) < 3:
                error(f"Missing FLS ID at {file}:{lineno}", fixable=False)
                fixed.write(line)  # Can't automatically fix
                continue

            if len(x) > 3:
                error(f"Multiple FLS IDs at {file}:{lineno}", fixable=False)
                # In theory we could split this into one line per item, but don't do that
                # automatically for now.
                fixed.write(line)
                continue

            id = x[2]

            if id.startswith("um_rustc"):
                if id in cli_ids:
                    # Okay
                    fixed.write(line)
                    continue
                else:
                    error(
                        f"Unknown CLI spec id '{id}' at {file}:{lineno}", fixable=False
                    )
                    # Can't automatically fix
                    fixed.write(line)
                    continue

            if not id.islower():
                error(f"Non-lowercase FLS ID '{id}'' at {file}:{lineno}")
                id = id.lower()  # to allow further processing

            # Normalize formatting
            fixed.write(f"{indent}// ferrocene-annotations: {id}\n")

            if id in sections:
                # Next line should be the title of the corresponding section.
                (_, title_line) = wrapper.peek()
                if title_line.lstrip().startswith("//"):
                    title = title_line.split(maxsplit=1)[1].rstrip()
                    expected_title = sections[id]
                    if title != expected_title:
                        error(
                            f"Incorrect section title at {file}:{lineno}: expected '{expected_title}', found '{title}'"
                        )
                    fixed.write(f"{indent}// {expected_title}\n")
                    wrapper.next()
                else:
                    error(
                        f"Annotation at {file}:{lineno} ({id}) is not followed by a section title"
                    )

            else:
                error(f"Unknown section '{id}' at {file}:{lineno}")
                # Can't automatically fix
                # Next line should be the title of the corresponding section.
                # We can't check it if we don't know the section ID, but make sure that there's
                # *something* there
                (_, title_line) = wrapper.peek()
                if title_line.lstrip().startswith("//"):
                    fixed.write(title_line)
                    wrapper.next()
                else:
                    error(
                        f"Annotation at {file}:{lineno} ({id}) is not followed by a section title"
                    )

        else:
            fixed.write(line)

    return fixed.getvalue()


def load_cli_ids():
    cli_ids = set()
    matrix = "build/host/doc/qualification/traceability-matrix.html"
    id_regex = re.compile(r'id="(um_rustc_[^"]*)"')
    try:
        with open(matrix) as fd:
            for line in fd:
                if m := id_regex.search(line):
                    cli_ids.add(m.group(1))
    except FileNotFoundError:
        exit(f"error: {matrix} not found. try running './x build traceability-matrix'.")

    return cli_ids


def validate_annotations():
    sections = {}
    for dir, _subdirs, files in os.walk("ferrocene/doc/specification"):
        for file in files:
            if file.endswith(".rst"):
                path = os.path.join(dir, file)
                original = open(path).read()
                fixed = load_sections(StringIO(original), path, sections)
                if args.bless and fixed != original:
                    print(f"Modified file {path}")
                    open(path, "w").write(fixed)

    cli_ids = load_cli_ids()

    for dir, _subdirs, files in os.walk("tests"):
        for file in files:
            if file.endswith(".rs") or file == "ferrocene-annotations":
                path = os.path.join(dir, file)
                original = open(path).read()
                fixed = validate_annotations_in_file(
                    StringIO(original), path, sections, cli_ids
                )
                if args.bless and fixed != original:
                    print(f"Modified file {path}")
                    open(path, "w").write(fixed)


print("Validating Ferrocene annotations against traceability matrix")
validate_annotations()

if not had_any_error:
    print("All annotations match the spec!")

exit(had_unfixable_error)
