# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import functools
import hashlib
import os


# Path of the directory containing all the Docker images. When a parameter
# references a Docker image, it will be looked up in this directory.
DOCKER_IMAGES_PATH = "ferrocene/ci/docker-images/"

# Paths to directories that docker images depend on in addition to those in DOCKER_IMAGES_PATH
ADDITIONAL_DOCKER_IMAGE_DEPENDENCY_DIRS = ["ferrocene/ci/mirrors"]

# Paths to files that docker images depend on in addition to those in DOCKER_IMAGES_PATH
ADDITIONAL_DOCKER_IMAGE_DEPENDENCY_FILES = [
    "ferrocene/ci/awscli-version",
    "ferrocene/ci/scripts/ensure-ubuntu-20-packages.sh",
]


@functools.cache  # prevent re-hashing the fs
def calculate_hash() -> str:
    all_files: list[str] = ADDITIONAL_DOCKER_IMAGE_DEPENDENCY_FILES
    for root, _, files in os.walk(DOCKER_IMAGES_PATH):
        all_files += [os.path.join(root, file) for file in files]
    for dep_dir in ADDITIONAL_DOCKER_IMAGE_DEPENDENCY_DIRS:
        for root, _, files in os.walk(dep_dir):
            all_files += [os.path.join(root, file) for file in files]

    # This is done in two steps to guarantee a stable sorting for the files,
    # otherwise inconsistencies in the filesystem could result in different
    # hashes even though the two directories are equal.
    hash = hashlib.sha256()
    for file in sorted(all_files):
        with open(file, "rb") as f:
            filename = file.encode("utf-8")
            hash.update(f"{len(filename)}|".encode())
            hash.update(filename)

            contents = f.read()
            hash.update(f"{len(contents)}|".encode())
            hash.update(contents)

    return hash.hexdigest()
