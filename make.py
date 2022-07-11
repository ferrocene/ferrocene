#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

# Convenience script to build the Ferrocene Language Specification, including
# setting up a Python virtual environment to install Sphinx into (removing the
# need to manage dependencies globally).

from pathlib import Path
import argparse
import subprocess
import venv


def build_docs(root, env, clear, serve):
    dest = root / "build"

    args = ["-b", "html", "-d", dest / "doctrees", "-j", "auto"]
    if clear:
        args.append("-E")
    if serve:
        args += ["--watch", root / "exts", "--watch", root / "themes"]

    try:
        subprocess.run(
            [
                env.bin("sphinx-autobuild" if serve else "sphinx-build"),
                *args,
                root / "src",
                dest / "html",
            ],
            check=False,
        )
    except KeyboardInterrupt:
        pass

    return dest / "html"


def build_linkchecker(root):
    repo = root / ".linkchecker"
    src = repo / "src" / "tools" / "linkchecker"
    bin = src / "target" / "release" / "linkchecker"

    if not src.is_dir():
        subprocess.run(["git", "init", repo], check=True)

        def git(args):
            subprocess.run(["git", *args], cwd=repo, check=True)

        # Avoid fetching blobs unless needed by the sparse checkout
        git(["remote", "add", "origin", "https://github.com/rust-lang/rust"])
        git(["config", "remote.origin.promisor", "true"])
        git(["config", "remote.origin.partialCloneFilter", "blob:none"])

        # Checkout only the linkchecker tool rather than the whole repo
        git(["config", "core.sparsecheckout", "true"])
        with open(repo / ".git" / "info" / "sparse-checkout", "w") as f:
            f.write("/src/tools/linkchecker/")

        # Avoid fetching the whole history
        git(["fetch", "--depth=1", "origin", "master"])
        git(["checkout", "master"])

    if not bin.is_file():
        subprocess.run(["cargo", "build", "--release"], cwd=src, check=True)

    return bin


class VirtualEnv:
    def __init__(self, root, path):
        self.path = path
        self.requirements = root / "requirements.txt"
        self.installed_requirements = path / "installed-requirements.txt"

        if not self.up_to_date():
            self.create()

    def bin(self, name):
        return self.path / "bin" / name

    def up_to_date(self):
        if self.installed_requirements.exists():
            expected = self.requirements.read_bytes()
            installed = self.installed_requirements.read_bytes()
            if expected == installed:
                return True
        return False

    def create(self):
        venv.EnvBuilder(clear=True, symlinks=True, with_pip=True).create(self.path)
        subprocess.run(
            [self.bin("pip"), "install", "-r", self.requirements, "--require-hashes"],
            check=True,
        )
        self.installed_requirements.write_bytes(self.requirements.read_bytes())


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-c", "--clear", help="disable incremental builds", action="store_true"
    )
    group = parser.add_mutually_exclusive_group()
    group.add_argument(
        "-s",
        "--serve",
        help="start a local server with live reload",
        action="store_true",
    )
    group.add_argument(
        "--check-links", help="Check whether all links are valid", action="store_true"
    )
    args = parser.parse_args()

    root = Path(__file__).parent.resolve()

    env = VirtualEnv(root, root / ".venv")
    rendered = build_docs(root, env, args.clear, args.serve)

    if args.check_links:
        linkchecker = build_linkchecker(root)
        if subprocess.run([linkchecker, rendered]).returncode != 0:
            print("error: linkchecker failed")
            exit(1)
