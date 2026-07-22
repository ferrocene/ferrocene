#!/home/circleci/.local/bin/uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["utils", "boto3 ~= 1.36", "pyyaml ~= 6.0"]
#
# [tool.uv.sources]
# utils = { path = "utils", editable = true }
# ///

# Dynamically calculate the value of the parameters defined in `workflows.yml`
# based on the rest of the source code and the current environment. The script
# tries to find a value for all the defined parameters automatically, and exits
# with an error if it can't calculate the value of one parameter.

import boto3
import json
import os
import sys
import urllib.parse
import yaml
from typing import Callable
from utils import llvm_cache, docker_images
from pathlib import Path

# Path of the YAML file to extract the needed parameters from.
CIRCLECI_CONFIGURATION = ".circleci/workflows.yml"

# Docker image (name, arch) pairs that are used
# If these images don't exist for the current hash at the right architecture,
# triggers an LLVM rebuild
# TODO: would be nice to eliminate this list
BASE_DOCKER_IMAGES = [
    ("emulator", "x86_64"),
    ("runner", "aarch64"),
    ("runner", "x86_64"),
]

# AWS regions we rely on.
S3_REGION = "us-east-1"
ECR_REGION = "us-east-1"

# How long should it take before an image is rebuilt.
REBUILD_IMAGES_OLDER_THAN_DAYS = 7

# QNX targets only work on x86_64 Windows, x86_64 Linux, and x86_64 Mac
# They must be excluded on, for example, aarch64 Mac
QNX71_TARGETS = [
    "aarch64-unknown-nto-qnx710",
    "x86_64-pc-nto-qnx710",
    # the QNX 8.0 targets require a different SDP (QNX toolchain) so they'll go
    # into different CI jobs (dist & self-test). we cannot list them together
    # with the QNX7.1 targets here
    # "aarch64-unknown-qnx",
    # "x86_64-pc-qnx",
]

GENERIC_BUILD_STD_TARGETS = [
    "aarch64-unknown-none",
    "aarch64-unknown-ferrocene.facade",
    "aarch64r82-unknown-none",
    "aarch64r82-unknown-none-softfloat",
    "aarch64v8r-unknown-none",
    "aarch64v8r-unknown-none-softfloat",
    "armv7r-ferrocene.facade-eabihf",
    "thumbv6m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-ferrocene.facade-eabi",
    "thumbv7em-none-eabihf",
    "thumbv7em-ferrocene.facade-eabihf",
    "armv8r-none-eabihf",
    "thumbv8m.base-none-eabi",
    "thumbv8m.main-none-eabi",
    "thumbv8m.main-none-eabihf",
    "wasm32-unknown-unknown",
    "armv7r-none-eabihf",
    "armebv7r-none-eabihf",
]

# Targets only built (and self-tested!) on Linux.
AARCH64_LINUX_BUILD_HOSTS = ["aarch64-unknown-linux-gnu"]
X86_64_LINUX_BUILD_HOSTS = ["x86_64-unknown-linux-gnu"]
X86_64_LINUX_BUILD_STD_TARGETS = [
    "riscv64gc-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "aarch64-rhivos2-linux-gnu",
    "s390x-unknown-linux-gnu",
    "powerpc64le-unknown-linux-gnu",
]
# x86_64-unknown-linux-gnu builds our generic cross compilation targets
# for us and is special cased somewhat. (This is used in `calculate_targets()`)
X86_64_LINUX_BUILD_STD_TARGETS_ALL = (
    X86_64_LINUX_BUILD_STD_TARGETS + GENERIC_BUILD_STD_TARGETS + QNX71_TARGETS
)
X86_64_LINUX_SELF_TEST_TARGETS = (
    X86_64_LINUX_BUILD_HOSTS
    + AARCH64_LINUX_BUILD_HOSTS
    + X86_64_LINUX_BUILD_STD_TARGETS_ALL
)
AARCH64_LINUX_SELF_TEST_TARGETS = (
    X86_64_LINUX_BUILD_HOSTS + AARCH64_LINUX_BUILD_HOSTS + GENERIC_BUILD_STD_TARGETS
)

# Targets only built (and tested!) on Mac
AARCH64_MAC_BUILD_HOSTS = ["aarch64-apple-darwin"]
AARCH64_MAC_BUILD_STD_TARGETS = []
AARCH64_MAC_SELF_TEST_TARGETS = (
    AARCH64_MAC_BUILD_HOSTS + AARCH64_MAC_BUILD_STD_TARGETS + GENERIC_BUILD_STD_TARGETS
)

# Tagets only built (and tested!) on Windows
X86_64_WINDOWS_BUILD_HOSTS = ["x86_64-pc-windows-msvc"]
X86_64_WINDOWS_SELF_TEST_TARGETS = (
    X86_64_WINDOWS_BUILD_HOSTS + GENERIC_BUILD_STD_TARGETS + QNX71_TARGETS
)

s3 = boto3.client("s3", region_name=S3_REGION)
ecr = boto3.client("ecr", region_name=ECR_REGION)

with open(CIRCLECI_CONFIGURATION) as f:
    config: dict[str, dict[str, str]] = yaml.safe_load(f)

full_build: dict[str, bool] = {
    "x86_64-pc-windows-msvc": os.environ["FULL_BUILD_X86_64_WINDOWS_MSVC"] == "true",
    "aarch64-apple-darwin": os.environ["FULL_BUILD_AARCH64_DARWIN"] == "true",
}


def calculate_docker_repository_url(repo: str) -> str:
    """
    Calculates the value of parameters starting with `docker-repository-url--`
    """
    repos = ecr.describe_repositories(repositoryNames=[repo])
    if not repos["repositories"]:
        raise ScriptError(f"ECR repository {repo} not found")

    return repos["repositories"][0]["repositoryUri"]


def calculate_llvm_rebuild(*dummy: str):
    """
    Calculates the value of parameters starting with `llvm-rebuild--`
    """
    not_found = 0
    for jobname in config["jobs"]:
        target = jobname.removeprefix("llvm--")
        if jobname != target and (
            target not in full_build or full_build[target] is True
        ):
            url: urllib.parse.ParseResult = llvm_cache.get_s3_url(target)
            assert url.scheme == "s3"
            try:
                s3.head_object(Bucket=url.netloc, Key=url.path.removeprefix("/"))
            except s3.exceptions.ClientError:
                print(
                    f"missing llvm artifact for {target}: {url.geturl()}",
                    file=sys.stderr,
                )
                not_found += 1
    return not_found > 0


def calculate_targets(host_plus_stage: str):
    """
    Calculates the list of targets to pass.

    :param str host_plus_stage: The Rust target hosting this job, then "--", then one of `build`, `std`, or `self-test`
    """
    host, stage = host_plus_stage.split("--", 1)

    if stage == "build":
        match host:
            case "aarch64-unknown-linux-gnu":
                targets = AARCH64_LINUX_BUILD_HOSTS
            case "x86_64-unknown-linux-gnu":
                targets = X86_64_LINUX_BUILD_HOSTS
            case "aarch64-apple-darwin":
                targets = (
                    AARCH64_MAC_BUILD_HOSTS + AARCH64_MAC_BUILD_STD_TARGETS
                )  # We don't currently produce x86_64 Apple host tools, but we will one day
            case "x86_64-pc-windows-msvc":
                targets = X86_64_WINDOWS_BUILD_HOSTS
            case _:
                raise Exception(
                    f"Host {host} not supported at this time, please add support"
                )
    elif stage == "std":
        if host == "x86_64-unknown-linux-gnu":
            targets = X86_64_LINUX_BUILD_STD_TARGETS_ALL
        else:
            raise Exception(
                "Only the `x86_64-unknown-linux-gnu` currently runs the `std-only` stage."
            )
    elif stage == "self-test":
        match host:
            case "aarch64-unknown-linux-gnu":
                targets = AARCH64_LINUX_SELF_TEST_TARGETS
            case "x86_64-unknown-linux-gnu":
                targets = X86_64_LINUX_SELF_TEST_TARGETS
            case "aarch64-apple-darwin":
                targets = AARCH64_MAC_SELF_TEST_TARGETS
            case "x86_64-pc-windows-msvc":
                targets = X86_64_WINDOWS_SELF_TEST_TARGETS
            case _:
                raise Exception(
                    f"Host {host} not supported at this time, please add support"
                )
    else:
        raise Exception(f"Stage {stage} not known, please add support")

    return ",".join(targets)


# We need `*dummy` since below in `prepare_paremeters` calls this with args.
def workflow_id(*dummy):
    var = os.environ.get("CIRCLE_WORKFLOW_ID")
    assert var is not None
    return var


# read from ferrocene/ci/awscli-version
def awscli_version(*dummy):
    return Path("ferrocene/ci/awscli-version").read_text()


# read from ferrocene/ci/qemu-version
def qemu_version(*dummy):
    return Path("ferrocene/ci/qemu-version").read_text()


def prepare_parameters():
    replacements: dict[str, Callable[[str], str | bool]] = {
        "docker-images-hash": lambda _: docker_images.calculate_hash(),
        "docker-repository-url--": calculate_docker_repository_url,
        "llvm-rebuild": calculate_llvm_rebuild,
        "targets--": calculate_targets,
        "stable-workflow-id": workflow_id,
        "awscli-version": awscli_version,
        "qemu-version": qemu_version,
    }

    parameters: dict[str, str | bool] = {}
    for parameter in config["parameters"].keys():
        for prefix, func in replacements.items():
            if parameter.startswith(prefix):
                # Anything after the prefix gets passed as a parameter
                parameters[parameter] = func(parameter[len(prefix) :])
                break
            if parameter.startswith("full-build-"):
                # ignore these parameters, they'll be passed straight on.
                break
        # In Python, the `else` is executed when the for loop finished
        # normally, without any `break` being executed. In this case, it's
        # executed whenever we don't do any replacement.
        else:
            raise ScriptError(f"unknown parameter: {parameter}")

    return parameters


class ScriptError(RuntimeError):
    pass


if __name__ == "__main__":
    # Ensure we're using a consistent working directory
    os.chdir(os.path.abspath(os.path.join(os.path.dirname(__file__), "../../..")))

    try:
        print(json.dumps(prepare_parameters(), indent=4))
    except ScriptError as e:
        print(f"error: {e}", file=sys.stderr)
        exit(1)
