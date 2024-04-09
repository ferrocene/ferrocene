#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers


import os
import subprocess


COMMIT_SUBJECT = "Automatically push changes from ferrocene/ferrocene"
MIRRORED_MARKER = "mirrored-commit: "
MIRROR_DIR = ""  # the root, but "." does not work
ORIGIN_DIR = "ferrocene/doc/sphinx-shared-resources/"


def main():
    """
    Mirror the origin repository to the mirror repository.

    There is the origin and the mirror repository. They are specified by its
    path, supplied as an environment variable (`*_REPO_PATH`). For both there
    is a directory from/into which the changes should be taken/put (`*_DIR`).

    The script relies on the assumption that the last commit made changes to
    the `ORIGIN_DIR` in the origin repo (`ORIGIN_REPO_PATH`). This is ensured
    through the `on.paths` configuration of the workflow.
    """

    # As an input the script takes two environment variables. They specify the
    # (relative) path to the origin and mirror repository.
    origin_repo_path = os.getenv("ORIGIN_REPO_PATH")
    mirror_repo_path = os.getenv("MIRROR_REPO_PATH")
    if origin_repo_path == None or mirror_repo_path == None:
        raise Exception("script needs env vars ORIGIN_REPO_PATH and MIRROR_REPO_PATH")

    last_mirrored_commit = get_last_mirrored_commit(mirror_repo_path)

    # create patch from the origin repo and map the paths from `ORIGIN_DIR` to
    # `MIRROR_DIR`. We need to map it in order to apply it in the other repo.
    patch = run(
        ["git", "diff", last_mirrored_commit, "HEAD", "--", ORIGIN_DIR],
        origin_repo_path,
    )
    if patch == "":
        raise Exception(
            "patch cannot be empty; make sure the last commit made changes to "
            + ORIGIN_DIR
        )
    patch = patch.replace(ORIGIN_DIR, MIRROR_DIR)

    # apply patch to the mirror repo and create a commit
    run(["git", "apply", "-"], mirror_repo_path, input=patch)
    run(["git", "add", "."], mirror_repo_path)
    commit_msg = get_commit_msg(origin_repo_path)
    run(["git", "commit", "-m", commit_msg], mirror_repo_path)


def get_commit_msg(origin_repo_path: str) -> str:
    # get the hash of the commit which triggered the workflow ...
    origin_commit_hash = run(["git", "rev-parse", "HEAD"], origin_repo_path)
    # ... and construct the commit message with it
    return COMMIT_SUBJECT + "\n\n" + MIRRORED_MARKER + origin_commit_hash


def get_last_mirrored_commit(mirror_repo_path: str) -> str:
    commit_messages = run(["git", "log", "--format=%B"], mirror_repo_path)
    for line in commit_messages.splitlines():
        if line.startswith(MIRRORED_MARKER):
            hash = line.removeprefix(MIRRORED_MARKER)
            return hash
    raise Exception("could not find mirrored-commit")


def run(args: list[str], cwd: str, input: str | None = None) -> str:
    """Run the command and return stdout."""
    print(f"🏃 {' '.join(args)}")
    try:
        return subprocess.run(
            args,
            capture_output=True,
            check=True,
            cwd=cwd,
            encoding="utf-8",
            input=input,
            timeout=10,
        ).stdout
    except subprocess.CalledProcessError as e:
        print(f"{e.stderr=}")
        raise e


if __name__ == "__main__":
    main()
