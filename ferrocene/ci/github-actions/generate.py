#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import os
import logging
import yaml
from pathlib import Path
from enum import Enum

class TargetTriple:
    def __init__(self, triple: str, host: bool, runner: str = None, setup_script: Path = None):
        self.triple = triple
        self.host = host
        self.runner = runner
        self.setup_script = setup_script

    def runner(self):
        self.runner

    def generate_build_job(self):
        if not self.host:
            raise ScriptError(f"{self.triple} is not a host platform.")

        job = {
            "name": f"{self.triple}-build",
            "runs_on": self.runner,
        }
        return job

    def generate_llvm_job(self):
        if not self.host:
            raise ScriptError(f"{self.triple} is not a host platform.")
            
        job = {
            "name": f"{self.triple}-build",
            "runs_on": self.runner,
        }
        return job

    def generate_dist_job(self):
        if not self.host:
            raise ScriptError(f"{self.triple} is not a host platform.")
            
        job = {
            "name": f"{self.triple}-build",
            "runs_on": self.runner,
        }
        return job


TARGET_TRIPLES = {
    "x86_64-unknown-linux-gnu": TargetTriple(
        triple="x86_64-unknown-linux-gnu",
        host=True,
        runner="ubuntu-latest"
    ),
    "aarch64-unknown-linux-gnu": TargetTriple(
        triple="aarch64-unknown-linux-gnu",
        host=True,
        runner="TODO"
    ),
    "x86_64-pc-windows-msvc": TargetTriple(
        triple="x86_64-pc-windows-msvc",
        host=True,
        runner="windows-latest",
        setup_script="setup-darwin.sh"
    ),
    "aarch64-apple-darwin": TargetTriple(
        triple="aarch64-apple-darwin",
        host=True,
        runner="TODO",
        setup_script="setup-windows.sh"
    ),
    "armv8r-none-eabihf": TargetTriple(
        triple="armv8r-none-eabihf",
        host=False,
        setup_script="setup-arm-toolchain.sh",
    ),
    "armv7r-none-eabihf": TargetTriple(
        triple="armv7r-none-eabihf",
        host=False,
        setup_script="setup-arm-toolchain.sh",
    ),
}


def generate_workflow():
    template_file = open("ferrocene/ci/github-actions/workflow_template.yaml", "r")
    template = yaml.load(template_file, Loader=yaml.Loader)

    jobs = template["jobs"]

    for target in TARGET_TRIPLES:
        target = TARGET_TRIPLES[target]
        if target.host:
            jobs[f"{target.triple}-llvm"] = target.generate_llvm_job()
            jobs[f"{target.triple}-build"] = target.generate_build_job()
            jobs[f"{target.triple}-dist"] = target.generate_dist_job()
    

    return template

class ScriptError(RuntimeError):
    pass

def arguments():
    parser = argparse.ArgumentParser(
        description="Generate GHA workflows for Ferrocene",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)

    return parser.parse_args()

def main():
    # Ensure we're using a consistent working directory
    os.chdir(os.path.abspath(os.path.join(os.path.dirname(__file__), "../../..")))

    args = arguments()

    match args.verbose:
        case 0:
            log_level = logging.INFO
        case 1:
            log_level = logging.DEBUG
        case _:
            log_level = logging.TRACE
    logging.basicConfig(format="%(asctime)s %(levelname)s: %(message)s", datefmt="%I:%M:%S %p", level=log_level)

    try:
        print(yaml.dump(generate_workflow(), Dumper=yaml.Dumper))
    except ScriptError as e:
        print(f"error: {e}", file=sys.stderr)
        exit(1)


if __name__ == "__main__":
    main()
