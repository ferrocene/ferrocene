#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script writes to a temporary file the difference of the signed documents built locally and
# the signed documents built by CI. This is helpful to determine differences between the two.

# /// script
# dependencies = ["requests"]
# ///

from dataclasses import dataclass
from pathlib import Path
import tempfile
from typing import Any, Generator, Optional, Set
import argparse
import os
import requests
import subprocess
import sys


GITHUB_REPO = "ferrocene/ferrocene"
BORS_USER_ID = 87868125
DOCS_COMPONENTS = ["ferrocene-docs", "ferrocene-docs-signatures"]


github = requests.Session()
github.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"


@dataclass
class BorsCommit:
    hash: str
    kind: str
    prs: Set[int]


def main(pr: int):
    eprint("===> checking whether you are in a bors GitHub repository...")
    if not os.path.exists("ferrocene/ci/channel"):
        eprint(
            "error: please run this script in the Ferrocene checkout you used to sign"
        )
        exit(1)

    eprint("===> checking whether you are authenticated with AWS...")
    subprocess.run(["aws", "sts", "get-caller-identity"], check=True)

    eprint("===> getting the last bors build from the GitHub API...")
    build = get_last_bors_build(pr)
    eprint(f"found {build.hash}!")

    destdir = download_docs(build.hash)

    eprint("===> building local documentation")
    subprocess.run(["./x.py", "doc", "ferrocene/doc", "--fresh"])

    eprint("===> comparing the local and upstream documentation")
    diff_path = generate_diff(destdir)

    eprint()
    eprint(f"path to the diff: {diff_path}")
    eprint()


def get_last_bors_build(pr: int) -> BorsCommit:
    commits = list(commit for commit in get_bors_commits(pr) if pr in commit.prs)
    if not commits:
        eprint(f"could not find any bors merge commit (try or auto) for PR #{pr}")
        exit(1)
    return commits[-1]


def get_bors_commits(pr: int) -> Generator[BorsCommit, None, None]:
    for event in get_pr_timeline(pr):
        if event["event"] != "referenced":
            continue
        if event["actor"]["id"] != BORS_USER_ID:
            continue

        commit = github.get(event["commit_url"])
        handle_github_api_error(commit)

        parsed = parse_bors_commit_message(
            event["commit_id"], commit.json()["commit"]["message"]
        )
        if parsed:
            yield parsed


def get_pr_timeline(pr: int) -> Generator[Any, None, None]:
    url = f"https://api.github.com/repos/{GITHUB_REPO}/issues/{pr}/timeline"

    while url is not None:
        resp = github.get(url)
        handle_github_api_error(resp)

        for event in resp.json():
            yield event

        url = resp.links["next"]["url"] if "next" in resp.links else None


def parse_bors_commit_message(hash: str, message: str) -> Optional[BorsCommit]:
    message = message.removesuffix(":")

    parts = message.split(" ")
    kind = parts[0]

    if kind not in ("Merge", "Try"):
        return None

    prs = set()
    for pr in parts[1:]:
        if not pr.startswith("#"):
            return None
        pr = pr.removeprefix("#")
        try:
            prs.add(int(pr))
        except ValueError:
            # Not an integer
            return None

    if prs:
        return BorsCommit(hash=hash, kind=kind, prs=prs)


def download_docs(commit: str) -> Path:
    tempdir = Path(tempfile.mkdtemp())

    for component in DOCS_COMPONENTS:
        tarball = ferrocene_tarball_name(commit, component)
        eprint(f"===> downloading and extracting {tarball} from S3")
        file = tempdir / "tarballs" / tarball
        url = f"s3://ferrocene-ci-artifacts/ferrocene/dist/{commit}/{tarball}"

        subprocess.run(["aws", "s3", "cp", url, file], check=True)
        subprocess.run(["tar", "-xf", file], cwd=tempdir, check=True)

    return tempdir


def ferrocene_tarball_name(commit: str, component: str) -> str:
    channel = file_for_commit(commit, "ferrocene/ci/channel").strip()
    if channel == "stable":
        version = file_for_commit(commit, "ferrocene/version").strip()
        return f"{component}-{version}.tar.xz"
    else:
        return f"{component}-{commit[:9]}.tar.xz"


def file_for_commit(commit: str, path: str) -> str:
    resp = github.get(
        f"https://api.github.com/repos/{GITHUB_REPO}/contents/{path}?ref={commit}",
        headers={"accept": "application/vnd.github.raw+json"},
    )
    handle_github_api_error(resp)
    return resp.text


def generate_diff(dest_dir: Path) -> Path:
    diff_path = dest_dir / "diff"
    diff = open(diff_path, "wb")

    local_docs = Path("build/host/doc")
    ci_docs = dest_dir / "share" / "doc" / "ferrocene" / "html"

    for book in find_sphinx_books(local_docs):
        subprocess.run(
            [
                "diff",
                "--unified",
                "--recursive",
                local_docs / book,
                ci_docs / book,
                "--exclude=signature",
            ],
            stdout=diff,
        )

    diff.close()
    return diff_path


def find_sphinx_books(path: Path, *, root=None) -> Generator[Path, None, None]:
    root = root if root else path
    for entry in path.iterdir():
        if entry.name == "searchindex.js":
            yield entry.parent.relative_to(root)
        elif entry.is_dir():
            for found in find_sphinx_books(entry, root=root):
                yield found


def handle_github_api_error(response: requests.Response):
    if response.status_code >= 400:
        eprint(f"error: request to the GitHub API failed: {response.json()['message']}")
        eprint(f"url: {response.url}")
        exit(1)


def eprint(*args, **kwargs):
    kwargs.setdefault("file", sys.stderr)
    print(*args, **kwargs)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("pr_number", type=int)
    args = parser.parse_args()

    main(args.pr_number)
