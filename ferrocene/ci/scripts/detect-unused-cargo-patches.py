#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires_python = ">= 3.12"
# dependencies = []
# ///

import subprocess
import tomllib
from collections import defaultdict


def find_files(name):
    cmd = subprocess.run(
        ["git", "ls-tree", "HEAD", "--name-only", "-r", "-z"],
        stdout=subprocess.PIPE,
        check=True,
        text=True,
    )

    # The -z flag changes the output to separate file names with a null byte, preventing files with
    # spaces or newlines from messing things up.
    for file in cmd.stdout.split("\0"):
        if file == name or file.endswith(f"/{name}"):
            yield file


def find_unused_patches(file):
    with open(file, "rb") as f:
        try:
            toml = tomllib.load(f)
        # Probably due to conflict markers:
        except tomllib.TOMLDecodeError:
            print(f"note: skipping {file} as it is not valid TOML")
            return

    try:
        unused_patches = toml["patch"]["unused"]
    except KeyError:
        return

    package_versions = defaultdict(list)
    for package in toml["package"]:
        package_versions[package["name"]].append(package["version"])

    for patch in unused_patches:
        # `patch` contains `name` and `version` as fields (straight from the TOML)
        yield {**patch, "used_versions": package_versions[patch["name"]]}


def main():
    found_unused_patches = False

    for lockfile in find_files("Cargo.lock"):
        for patch in find_unused_patches(lockfile):
            print(
                f"error: {lockfile}: our patch for {patch['name']} has version {patch['version']} "
                f"but the lockfile depends on {patch['used_versions']}",
            )
            found_unused_patches = True

    if found_unused_patches:
        print()
        print("help: if a patch comes from a subtree, the subtree is probably outdated")
        exit(1)


if __name__ == "__main__":
    main()
