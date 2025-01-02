#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Dynamically calculate the value of the parameters defined in `workflows.yml`
# based on the rest of the source code and the current environment. The script
# tries to find a value for all the defined parameters automatically, and exits
# with an error if it can't calculate the value of one parameter.


import boto3
import datetime
import hashlib
import json
import os
import subprocess
import sys
import urllib.parse
import yaml
from typing import Callable
import llvm_cache


# Path of the YAML file to extract the needed parameters from.
CIRCLECI_CONFIGURATION = ".circleci/workflows.yml"

# Path of the directory containing all the Docker images. When a parameter
# references a Docker image, it will be looked up in this directory.
DOCKER_IMAGES_PATH = "ferrocene/ci/docker-images/"

# AWS regions we rely on.
S3_REGION = "us-east-1"
ECR_REGION = "us-east-1"

# How long should it take before an image is rebuilt.
REBUILD_IMAGES_OLDER_THAN_DAYS = 7

# QNX targets only work on x86_64 Windows, x86_64 Linux, and x86_64 Mac
# They must be excluded on, for example, aarch64 Mac
QNX_TARGETS = [
    "aarch64-unknown-nto-qnx710",
    "x86_64-pc-nto-qnx710",
]

GENERIC_BUILD_STD_TARGETS = [
    "aarch64-unknown-none",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
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
]
# x86_64-unknown-linux-gnu builds our generic cross compilation targets
# for us and is special cased somewhat. (This is used in `calculate_targets()`)
X86_64_LINUX_BUILD_STD_TARGETS_ALL = X86_64_LINUX_BUILD_STD_TARGETS + GENERIC_BUILD_STD_TARGETS + QNX_TARGETS
X86_64_LINUX_SELF_TEST_TARGETS = X86_64_LINUX_BUILD_HOSTS + AARCH64_LINUX_BUILD_HOSTS + X86_64_LINUX_BUILD_STD_TARGETS_ALL
AARCH64_LINUX_SELF_TEST_TARGETS = X86_64_LINUX_BUILD_HOSTS + AARCH64_LINUX_BUILD_HOSTS  + GENERIC_BUILD_STD_TARGETS

# Targets only built (and tested!) on Mac
AARCH64_MAC_BUILD_HOSTS = ["aarch64-apple-darwin"]
AARCH64_MAC_BUILD_STD_TARGETS = ["x86_64-apple-darwin"]
AARCH64_MAC_SELF_TEST_TARGETS = AARCH64_MAC_BUILD_HOSTS + AARCH64_MAC_BUILD_STD_TARGETS + GENERIC_BUILD_STD_TARGETS

# Tagets only built (and tested!) on Windows
X86_64_WINDOWS_BUILD_HOSTS = ["x86_64-pc-windows-msvc"]
X86_64_WINDOWS_SELF_TEST_TARGETS = X86_64_WINDOWS_BUILD_HOSTS + GENERIC_BUILD_STD_TARGETS + QNX_TARGETS

s3 = boto3.client("s3", region_name=S3_REGION)
ecr = boto3.client("ecr", region_name=ECR_REGION)


def calculate_docker_image_tag(platform_plus_image: str):
    """
    Calculates the value of parameters starting with `docker-image-tag--`.
    """
    platform, image = platform_plus_image.split("--", 1)

    path = os.path.join(DOCKER_IMAGES_PATH, image)
    if not os.path.exists(os.path.join(path, "Dockerfile")):
        raise ScriptError(f"unknown Docker image: {image}")

    all_files: list[str] = []
    for root, _, files in os.walk(path):
        for file in files:
            all_files.append(os.path.join(root, file))
    # The docker files depend on `requirements.txt` for their venv.
    all_files.append("requirements.txt")

    # This is done in two steps to guarantee a stable sorting for the files,
    # otherwise inconsistencies in the filesystem could result in different
    # hashes even though the two directories are equal.
    hash = hashlib.sha256()
    for file in sorted(all_files):
        with open(file, "rb") as f:
            hash.update(file.encode("utf-8"))
            hash.update(f.read())

    return f"{platform}-{image}-{hash.hexdigest()}"


def calculate_docker_image_rebuild(repo_plus_platform_plus_image: str) -> bool:
    """
    Calculate the value of parameters starting with `docker-image-rebuild--`
    """
    repo, platform, image = repo_plus_platform_plus_image.split("--", 2)
    try:
        image = ecr.describe_images(
            repositoryName=repo,
            imageIds=[{"imageTag": calculate_docker_image_tag(f"{platform}--{image}")}],
        )["imageDetails"][0]
    except ecr.exceptions.ImageNotFoundException:
        # Image doesn't exist, build it.
        return True

    # FIXME: .utcnow should be .now(datetime.UTC), but CI is on python 3.9
    now = datetime.datetime.utcnow().replace(tzinfo=datetime.timezone.utc)
    delta: datetime.timedelta = now - image["imagePushedAt"]
    return delta.days >= REBUILD_IMAGES_OLDER_THAN_DAYS


def calculate_docker_repository_url(repo: str) -> str:
    """
    Calculates the value of parameters starting with `docker-repository-url--`
    """
    repos = ecr.describe_repositories(repositoryNames=[repo])
    if not repos["repositories"]:
        raise ScriptError(f"ECR repository {repo} not found")

    return repos["repositories"][0]["repositoryUri"]


def calculate_llvm_rebuild(target: str):
    """
    Calculates the value of parameters starting with `llvm-rebuild--`
    """
    url: urllib.parse.ParseResult = llvm_cache.get_s3_url(target)
    assert url.scheme == "s3"

    try:
        s3.head_object(Bucket=url.netloc, Key=url.path.removeprefix("/"))
        return False
    except s3.exceptions.ClientError:
        return True


def calculate_targets(host_plus_stage: str):
    """
    Calculates the list of targets to pass.

    :param str host_plus_stage: The Rust target hosting this job, then "--", then one of `build`, `std-only`, or `self-test`
    """
    host, stage = host_plus_stage.split("--", 1)

    # The CI does not run Python 3.10 and thus `match` statements don't exist yet
    # in this universe.
    if stage == "build":
        if host == "aarch64-unknown-linux-gnu":
            targets = AARCH64_LINUX_BUILD_HOSTS
        elif host == "x86_64-unknown-linux-gnu":
            targets = X86_64_LINUX_BUILD_HOSTS
        elif host == "aarch64-apple-darwin":
            targets = AARCH64_MAC_BUILD_HOSTS + AARCH64_MAC_BUILD_STD_TARGETS # We don't currently produce x86_64 Apple host tools, but we will one day
        elif host == "x86_64-pc-windows-msvc":
            targets = X86_64_WINDOWS_BUILD_HOSTS
        else:
            raise Exception(f"Host {host} not supported at this time, please add support")
    elif stage == "std":
        if host == "x86_64-unknown-linux-gnu":
            targets = X86_64_LINUX_BUILD_STD_TARGETS_ALL
        else:
            raise Exception("Only the `x86_64-unknown-linux-gnu` currently runs the `std-only` stage.")
    elif stage == "self-test":
        if host == "aarch64-unknown-linux-gnu":
            targets = AARCH64_LINUX_SELF_TEST_TARGETS    
        elif host == "x86_64-unknown-linux-gnu":
            targets = X86_64_LINUX_SELF_TEST_TARGETS
        elif host == "aarch64-apple-darwin":
            targets = AARCH64_MAC_SELF_TEST_TARGETS
        elif host == "x86_64-pc-windows-msvc":
            targets = X86_64_WINDOWS_SELF_TEST_TARGETS
        else:
            raise Exception(f"Host {host} not supported at this time, please add support")
    else:
        raise Exception(f"Stage {stage} not known, please add support")

    return ",".join(targets)


def prepare_parameters():
    with open(CIRCLECI_CONFIGURATION) as f:
        config: dict[str, dict[str, str]] = yaml.safe_load(f)

    replacements: dict[str, Callable[[str], str]] = {
        "docker-image-tag--": calculate_docker_image_tag,
        "docker-image-rebuild--": calculate_docker_image_rebuild,
        "docker-repository-url--": calculate_docker_repository_url,
        "llvm-rebuild--": calculate_llvm_rebuild,
        "targets--": calculate_targets,
    }

    parameters: dict[str, str] = {}
    for parameter in config["parameters"].keys():
        for prefix, func in replacements.items():
            if parameter.startswith(prefix):
                # Anything after the prefix gets passed as a parameter
                parameters[parameter] = func(parameter[len(prefix) :])
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
