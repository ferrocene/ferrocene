#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["requests ~= 2.32"]
# ///

import argparse
import base64
import os
import requests
import sys


LABEL_PREFIX = "merged-in:"
LABEL_COLOR = "c5def5"  # light blue


http = requests.Session()
http.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"

repo = os.environ["GITHUB_REPOSITORY"]


def main(pr_number):
    pr = get(f"pulls/{pr_number}", check=False)

    if pr is None:
        error(f"PR {pr_number} doesn't exist")
    if pr["merged_at"] is None:
        error(f"PR {pr_number} has not been merged")

    # Determine the expected label.
    src_version_resp = get(f"contents/src/version?ref={pr['merge_commit_sha']}")
    major, minor, _patch = (
        base64.b64decode(src_version_resp["content"]).decode("utf-8").strip().split(".")
    )
    new_label = f"{LABEL_PREFIX}{major}.{minor}"

    # Create the label if it doesn't exist.
    if get(f"labels/{new_label}", check=False) is None:
        post("labels", json={"name": new_label, "color": LABEL_COLOR})

    # Add the label to the PR. This uses the "issues" API because the GitHub API treats issues and
    # PRs the same way (PRs have an extra set of APIs specific to them though).
    post(f"issues/{pr_number}/labels", json=[new_label])


def get(path, *, check=True):
    resp = http.get(f"https://api.github.com/repos/{repo}/{path}")
    if not check and resp.status_code == 404:
        return None
    resp.raise_for_status()
    return resp.json()


def post(path, *, json=None):
    resp = http.post(f"https://api.github.com/repos/{repo}/{path}", json=json)
    resp.raise_for_status()


def error(message):
    print(f"error: {message}", file=sys.stderr)
    exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("pr_number")
    cli = parser.parse_args()

    main(cli.pr_number)
