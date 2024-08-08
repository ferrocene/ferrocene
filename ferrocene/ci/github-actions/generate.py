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
    def __init__(self, host: bool, setup_script: Path):
        self.host = host
        self.setup_script = setup_script


TARGET_TRIPLES = {
    "x86_64-unknown-linux-gnu": TargetTriple(
        host=True,
        setup_script=None
    ),
    "aarch64-unknown-linux-gnu": TargetTriple(
        host=True,
        setup_script=None
    ),
    "x86_64-pc-windows-msvc": TargetTriple(
        host=True,
        setup_script="setup-darwin.sh"
    ),
    "aarch64-apple-darwin": TargetTriple(
        host=True,
        setup_script="setup-windows.sh"
    ),
    "armv8r-none-eabihf": TargetTriple(
        host=False,
        setup_script="setup-arm-toolchain.sh",
    ),
    "armv7r-none-eabihf": TargetTriple(
        host=False,
        setup_script="setup-arm-toolchain.sh",
    ),
}


def generate_workflow():
    template = yaml.load("ferrocene/ci/github-actions/workflow_template.yaml", Loader=yaml.Loader)


    for triple in TARGET_TRIPLES:
        triple_info = TARGET_TRIPLES[triple]
        if triple_info.host:
            logging.info(f"{triple}: {triple_info.host}")
    

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
        print(yaml.dump(generate_workflow()))
    except ScriptError as e:
        print(f"error: {e}", file=sys.stderr)
        exit(1)


if __name__ == "__main__":
    main()
