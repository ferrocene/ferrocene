#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Authors

# /// script
# requires_python = ">= 3.12"
# dependencies = []
# ///

from pathlib import Path
import os
import subprocess
import sys


# All files in one of the paths included withing CHECK_PATHS but not in EXCLUDE_PATHS will be
# checked. Only exclude paths if they are external subtrees not managed by us.
CHECK_PATHS = ["ferrocene", ".circleci", ".github"]
EXCLUDE_PATHS = ["ferrocene/library/libc"]

UV_SHEBANG = b"#!/usr/bin/env -S uv run"


def main():
    cmd = subprocess.run(
        ["git", "ls-tree", "HEAD", "--name-only", "-r", "-z"],
        stdout=subprocess.PIPE,
        check=True,
        text=True,
    )

    # The -z flag changes the output to separate file names with a null byte, preventing files with
    # spaces or newlines in their names from messing things up.
    has_errors = False
    for file in cmd.stdout.split("\0"):
        file = Path(file)

        def error(message):
            nonlocal has_errors
            has_errors = True
            print(f"error: {file}: {message}", file=sys.stderr)

        if file.is_dir():
            continue
        if file.suffix != ".py":
            continue
        if not any(file.is_relative_to(path) for path in CHECK_PATHS):
            continue
        if any(file.is_relative_to(path) for path in EXCLUDE_PATHS):
            continue

        contents = open(file, "rb").read()
        has_shebang = contents.startswith(b"#!")
        has_uv_shebang = contents.startswith(UV_SHEBANG)
        is_executable = os.access(file, os.X_OK)

        if has_uv_shebang and is_executable:
            # Yay!
            continue
        elif not has_shebang and not is_executable:
            # This is not an executable script, ignore.
            continue
        elif has_shebang and not is_executable:
            error("the file has a #!shebang but is not executable")
        elif has_shebang and not has_uv_shebang:
            error("script has a different #!shebang than uv")
        elif is_executable and not has_uv_shebang:
            error("script is executable but doesn't have the uv #!shebang")

    if has_errors:
        print(
            f"\nAs a reminder, the uv #!shebang is:\n\n    {UV_SHEBANG.decode('utf-8')}\n",
            file=sys.stderr,
        )
        exit(1)


if __name__ == "__main__":
    main()
