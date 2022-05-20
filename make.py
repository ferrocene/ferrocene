#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: Critical Section GmbH

# Convenience script to build the Ferrocene Language Specification, including
# setting up a Python virtual environment to install Sphinx into (removing the
# need to manage dependencies globally).

from pathlib import Path
import argparse
import ensurepip
import shutil
import subprocess
import venv


def build_docs(root, env, clear):
    dest = root / "build"

    if clear and dest.is_dir():
        shutil.rmtree(dest)

    subprocess.run(
        [
            env.bin("sphinx-build"),
            "-M",
            "html",
            root / "src",
            dest,
        ],
        check=False,
    )


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
    args = parser.parse_args()

    root = Path(__file__).parent.resolve()

    env = VirtualEnv(root, root / ".venv")
    build_docs(root, env, args.clear)
