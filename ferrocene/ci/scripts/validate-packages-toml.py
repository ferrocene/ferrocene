#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["boto3 ~= 1.36"]
# ///

# This script ensures that all of the tarballs referenced by packages.toml are uploaded, and that
# all uploaded artifacts are referenced in packages.toml (except when explicitly ignored).

import argparse
import boto3
import fnmatch
import io
import os
import subprocess
import sys
import tomllib


ARTIFACTS_BUCKET=(os.environ.get("ARTIFACTS_BUCKET") or "ferrocene-ci-artifacts")
ARTIFACTS_PREFIX=(os.environ.get("ARTIFACTS_PREFIX") or "ferrocene/dist")
GIT_REMOTE = "origin"

SUPPORTED_MANIFEST_VERSION = 2
TARBALL_COMPRESSION = ["xz"]

IGNORE_PATTERNS = [
    # Files that are not components:
    "build-metrics/*",
    "coverage/*",
    "ferrocene-ci-metadata.json",
    "packages.toml",
    # Components we don't distribute:
    "rust-docs-json-*.tar.*",
    "rustc-dev-*.tar.*",
    "rust-dev-*.tar.*",
]


s3 = boto3.client("s3")


def get_version_string(commit):
    channel = git_file_content(commit, "ferrocene/ci/channel")
    if channel.strip() == "stable":
        return git_file_content(commit, "ferrocene/version").strip()
    else:
        return commit[:9]


def packages_from_toml(commit):
    raw = s3_file_content(ARTIFACTS_BUCKET, f"{ARTIFACTS_PREFIX}/{commit}/packages.toml")
    toml = tomllib.loads(raw)

    if toml["manifest-version"] != SUPPORTED_MANIFEST_VERSION:
        print(
            f"error: manifest version {toml["manifest-version"]} is not supported by this script"
        )
        exit(1)

    version = get_version_string(commit)
    for group in toml["groups"].values():
        for target in group["targets"]:
            for package in group["packages"]:
                for compression in TARBALL_COMPRESSION:
                    if target == "*":
                        target_str = ""
                    else:
                        target_str = f"-{target}"
                    yield f"{package["name"]}{target_str}-{version}.tar.{compression}"


class Checker:
    def __init__(self, ignore_rules):
        self.ignore_rules = ignore_rules
        self.unused_ignore_rules = set(ignore_rules)
        self.errored = False

    def check(self, files, check_presence_in, error_message):
        for file in files:
            if file in check_presence_in:
                continue
            for pattern in IGNORE_PATTERNS:
                if fnmatch.fnmatch(file, pattern):
                    if pattern in self.unused_ignore_rules:
                        self.unused_ignore_rules.remove(pattern)
                    break
            else:
                self.errored = True
                print(f"error: {error_message}: {file}")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("commit", help="Commit hash to validate")
    args = parser.parse_args()

    uploaded = set(s3_list_files(ARTIFACTS_BUCKET, f"{ARTIFACTS_PREFIX}/{args.commit}/"))
    expected = set(packages_from_toml(args.commit))

    checker = Checker(IGNORE_PATTERNS)
    checker.check(uploaded, expected, "unexpected file uploaded to artifacts")
    checker.check(expected, uploaded, "expected file missing from artifacts")

    if checker.errored:
        print()
        print(
            "help: if the unexpected files should not be distributed, add new ignore "
            "patterns in this script."
        )
        exit(1)

    # Ensure we never unintentionally leave a stray ignore rule.
    if checker.unused_ignore_rules:
        for unused in checker.unused_ignore_rules:
            print(f"error: unused ignore rule: {unused}")
        exit(1)


def s3_list_files(bucket, prefix):
    extra_args = {}
    while True:
        resp = s3.list_objects_v2(Bucket=bucket, Prefix=prefix, **extra_args)
        for obj in resp["Contents"]:
            yield obj["Key"].removeprefix(prefix)
        if "ContinuationToken" in resp:
            extra_args["ContinuationToken"] = resp["ContinuationToken"]
        else:
            break


def s3_file_content(bucket, key):
    buf = io.BytesIO()
    s3.download_fileobj(bucket, key, buf)
    buf.seek(0)
    return buf.read().decode("utf-8")


def git_file_content(commit, path):
    exists = subprocess.run(["git", "cat-file", "-e", commit])
    if exists.returncode != 0:
        print(
            f"note: commit {commit} is not present locally, fetching it...",
            file=sys.stderr,
        )
        subprocess.run(["git", "fetch", GIT_REMOTE, commit], check=True)

    content = subprocess.run(
        ["git", "show", f"{commit}:{path}"],
        check=True,
        text=True,
        capture_output=True,
    )
    return content.stdout


if __name__ == "__main__":
    main()
