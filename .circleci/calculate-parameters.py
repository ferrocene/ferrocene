#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Dynamically calculate the value of the parameters defined in `workflows.yml`
# based on the rest of the source code and the current environment. The script
# tries to find a value for all the defined parameters automatically, and exits
# with an error if it can't calculate the value of one parameter.

import hashlib
import json
import os
import yaml
import boto3
import sys
import datetime
import urllib.parse
import subprocess


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


s3 = boto3.client("s3", region_name=S3_REGION)
ecr = boto3.client("ecr", region_name=ECR_REGION)


def calculate_docker_image_tag(image):
    """
    Calculates the value of parameters starting with `docker-image-tag--`.
    """
    path = os.path.join(DOCKER_IMAGES_PATH, image)
    if not os.path.exists(os.path.join(path, "Dockerfile")):
        raise ScriptError(f"unknown Docker image: {image}")

    all_files = []
    for root, _, files in os.walk(path):
        for file in files:
            all_files.append(os.path.join(root, file))

    # This is done in two steps to guarantee a stable sorting for the files,
    # otherwise inconsistencies in the filesystem could result in different
    # hashes even though the two directories are equal.
    hash = hashlib.sha256()
    for file in sorted(all_files):
        with open(file, "rb") as f:
            hash.update(file.encode("utf-8"))
            hash.update(f.read())

    return f"{image}-{hash.hexdigest()}"


def calculate_docker_image_rebuild(repo_plus_image):
    """
    Calculate the value of parameters starting with `docker-image-rebuild--`
    """
    repo, image = repo_plus_image.split("--", 1)
    try:
        image = ecr.describe_images(
            repositoryName=repo,
            imageIds=[{"imageTag": calculate_docker_image_tag(image)}],
        )["imageDetails"][0]
    except ecr.exceptions.ImageNotFoundException:
        # Image doesn't exist, build it.
        return True

    now = datetime.datetime.utcnow().replace(tzinfo=datetime.timezone.utc)
    delta = now - image["imagePushedAt"]
    return delta.days >= REBUILD_IMAGES_OLDER_THAN_DAYS


def calculate_docker_repository_url(repo):
    """
    Calculates the value of parameters starting with `docker-repository-url--`
    """
    repos = ecr.describe_repositories(repositoryNames=[repo])
    if not repos["repositories"]:
        raise ScriptError(f"ECR repository {repo} not found")

    return repos["repositories"][0]["repositoryUri"]


def calculate_llvm_rebuild(target):
    """
    Calculates the value of parameters starting with `llvm-rebuild--`
    """
    url = urllib.parse.urlparse(subprocess.run(
        ["ferrocene/ci/scripts/llvm-cache.sh", "s3-url"],
        env={"FERROCENE_HOST": target},
        stdout=subprocess.PIPE,
    ).stdout.strip()).decode("utf-8")
    assert url.scheme == "s3"

    try:
        s3.head_object(Bucket=url.netloc, Key=url.path.removeprefix("/"))
        return False
    except s3.exceptions.ClientError:
        return True


def prepare_parameters():
    with open(CIRCLECI_CONFIGURATION) as f:
        config = yaml.safe_load(f)

    replacements = {
        "docker-image-tag--": calculate_docker_image_tag,
        "docker-image-rebuild--": calculate_docker_image_rebuild,
        "docker-repository-url--": calculate_docker_repository_url,
        "llvm-rebuild--": calculate_llvm_rebuild,
    }

    parameters = {}
    for parameter in config["parameters"].keys():
        for prefix, func in replacements.items():
            if parameter.startswith(prefix):
                parameters[parameter] = func(parameter[len(prefix):])
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
    os.chdir(os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

    try:
        print(json.dumps(prepare_parameters(), indent=4))
    except ScriptError as e:
        print(f"error: {e}", file=sys.stderr)
        exit(1)
