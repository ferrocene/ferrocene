#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import logging
import os
import hashlib
import subprocess
import requests
import datetime
import docker

# Path of the directory containing all the Docker images.
DOCKER_IMAGES_PATH = "ferrocene/ci/docker-images/"
REGISTRY_ADDR = "ferrocenecr.azurecr.io"

def digest(image):
    path = os.path.join(DOCKER_IMAGES_PATH, image)
    if not os.path.exists(os.path.join(path, "Dockerfile")):
        raise ScriptError(f"unknown Docker image: {image}")

    all_files: list[str] = []
    for root, _, files in os.walk(path):
        for file in files:
            all_files.append(os.path.join(root, file))
    # The docker files depend on `requirements.txt` for their venv.
    all_files.append("requirements.txt")
    # This file
    all_files.append(__file__)

    # This is done in two steps to guarantee a stable sorting for the files,
    # otherwise inconsistencies in the filesystem could result in different
    # hashes even though the two directories are equal.
    hash = hashlib.sha256()
    for file in sorted(all_files):
        with open(file, "rb") as f:
            hash.update(file.encode("utf-8"))
            hash.update(f.read())

    return hash.hexdigest()

def build_docker_client():
    docker_client = docker.from_env()
    return docker_client

def repo_for_image(image):
   return f"ferrocene/{image}"

def ensure_built(platform, image):
    docker_client = build_docker_client()
    sought_address = uri(platform, image)
    
    match platform:
        case "x86_64":
            docker_platform = "linux/amd64"
        case "aarch64":
            docker_platform = "linux/arm64/v8"
        case _:
            raise ScriptError("Unknown platform (try x86_64 or aarch64)")
    
    logging.info(f"Ensuring {sought_address} is built")
    remote_image = False
    try:
        docker_client.images.get_registry_data(sought_address)
        remote_image = True
        logging.info(f"Remote image found")
    except docker.errors.APIError as err:
        logging.info(f"Remote image not found")

    local_image = False
    try:
        # docker_client.images.get does not, in fact, discover all images, so list instead
        found_images = docker_client.images.list(filters={'reference': sought_address})
        if len(found_images) == 1:
            local_image = True
            logging.info(f"Local image found")
    except docker.errors.ImageNotFound:
        logging.info(f"Local image not found")

    match (remote_image, local_image):
        case (True, False):
            logging.info(f"Pulling")
            subprocess.run(
                ["docker", "pull", sought_address],
                check=True
            )
        case (False, False):
            logging.info(f"Building")
            # The docker python package API does not, in fact, use buildkit.
            # Instead, we are forced to use Docker via the CLI (which uses buildkit in our configuration)
            subprocess.run(
                ["docker", "build", "--platform", docker_platform, "--tag", sought_address, "--file", f"ferrocene/ci/docker-images/{image}/Dockerfile", "."],
                check=True
            )

            logging.info(f"Pushing")
            subprocess.run(
                ["docker", "push", sought_address],
                check=True
            )
        case (False, True):
            logging.info(f"Pushing")
            subprocess.run(
                ["docker", "push", sought_address],
                check=True
            )


def uri(platform, image):
    hex_digest = digest(image)
    sought_tag = f"{platform}-{hex_digest}"
    sought_repo = repo_for_image(image)
    sought_image = f"{REGISTRY_ADDR}/{sought_repo}"
    sought_address = f"{sought_image}:{sought_tag}"
    return sought_address


def arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    digest_parser = subparsers.add_parser("digest")
    digest_parser.add_argument('image')

    ensure_built_parser = subparsers.add_parser("ensure-built")
    ensure_built_parser.add_argument('platform')
    ensure_built_parser.add_argument('image')

    uri_parser = subparsers.add_parser("uri")
    uri_parser.add_argument('platform')
    uri_parser.add_argument('image')

    return parser.parse_args()

def main():
    args = arguments()

    match args.verbose:
        case 0:
            log_level = logging.INFO
        case _:
            log_level = logging.DEBUG
    logging.basicConfig(format="%(asctime)s %(levelname)s: %(message)s", datefmt="%I:%M:%S %p", level=log_level)

    match args.subcommand:
        case "digest":
            print(digest(args.image))
        case "ensure-built":
            ensure_built(args.platform, args.image)
        case "uri":
            print(uri(args.platform, args.image))
        case _:
            print("Unknown command, see --help")
            exit(1)

    

class ScriptError(RuntimeError):
    pass

if __name__ == "__main__":
    main()

