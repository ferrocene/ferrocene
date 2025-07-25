#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

# The `src/stage0` file not only describe which compiler to use as the first bootstrap stage, but it
# also serves as a generic "configuration file" for the repository.
#
# One of its fields is `nightly_branch`, configuring the name of the branch PRs will be sent to. For
# us that is not just `main`, but also `release/1.NN` if we are in a release branch. Having the
# wrong value in the field breaks bootstrap's download from CI feature, as bootstrap won't be able
# to determine the commit to download from.
#
# This script updates `src/stage0` to always point to the correct branch, and it's executed as part
# of upstream pulls. It also provides a --check flag to ensure the branch name is correct, for CI.

from dataclasses import dataclass
from pathlib import Path
from typing import Optional
import argparse


ROOT = Path(__file__).resolve().parent.parent.parent.parent
STAGE0_PATH = ROOT / "src" / "stage0"
BRANCH_KEY = "nightly_branch"


def expected_branch():
    rust_channel = (ROOT / "src" / "ci" / "channel").read_text().strip()
    rust_version = (ROOT / "src" / "version").read_text().strip()

    if rust_channel == "nightly":
        return "main"
    else:
        major, minor, _patch = rust_version.split(".")
        return f"release/{major}.{minor}"


def parse_stage0():
    stage0 = Stage0()
    raw = STAGE0_PATH.read_text()

    for line in raw.split("\n"):
        if line.startswith(f"{BRANCH_KEY}="):
            if stage0.branch is None:
                stage0.branch = line.removeprefix(f"{BRANCH_KEY}=")
            else:
                raise RuntimeError(f"duplicate {BRANCH_KEY} in stage0")
        else:
            if stage0.branch is None:
                stage0.before_raw += f"{line}\n"
            else:
                stage0.after_raw += f"\n{line}"

    if stage0.branch is None:
        raise RuntimeError(f"missing {BRANCH_KEY} in stage0")

    return stage0


def check():
    current = parse_stage0().branch
    expected = expected_branch()

    if current != expected:
        print(f"error: {BRANCH_KEY} is '{current}' but must be '{expected}' in {STAGE0_PATH}")
        print("note: the pull from upstream automation is supposed to fix this, you should check")
        print("      why it didn't update the key in the pull from beta.")
        exit(1)


def fix():
    stage0 = parse_stage0()
    expected = expected_branch()

    stage0_file = STAGE0_PATH.open("w")
    stage0_file.write(f"{stage0.before_raw}{BRANCH_KEY}={expected}{stage0.after_raw}")
    stage0_file.close()

    


@dataclass
class Stage0:
    before_raw: str = ""
    branch: Optional[str] = None
    after_raw: str = ""


if __name__ == "__main__":
    cli = argparse.ArgumentParser()
    cli.add_argument("--check", action="store_true")
    args = cli.parse_args()

    if args.check:
        check()
    else:
        fix()
