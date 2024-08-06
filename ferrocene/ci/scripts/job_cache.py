#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import cache
import os
import logging
import shutil
from pathlib import Path

CACHE_BUCKET="ferrocene-ci-caches"
CACHE_PREFIX="persist-between-jobs"

def get_problematic_symlinks(ferrocene_host):
    """
    In the build directory, there exists several cyclic symlinks.

    We need to tear those down and rebuild them on restore.
    """
    return {
        f"build/{ferrocene_host}/stage0-sysroot/lib/rustlib/rustc-src": os.getcwd(),
        f"build/{ferrocene_host}/stage0-sysroot/lib/rustlib/src": os.getcwd(),
        f"build/{ferrocene_host}/stage1/lib/rustlib/rustc-src": os.getcwd(),
        f"build/{ferrocene_host}/stage1/lib/rustlib/src": os.getcwd(),
        f"build/{ferrocene_host}/stage2/lib/rustlib/rustc-src": os.getcwd(),
        f"build/{ferrocene_host}/stage2/lib/rustlib/src": os.getcwd(),
        "build/host": f"build/{ferrocene_host}",
    }



def job_cache_url(workspace_id, job):
    return f"s3://{CACHE_BUCKET}/{CACHE_PREFIX}/{workspace_id}/{job}.tar.zst"

def subcommand_store(ferrocene_host, workspace_id, path=None, job=None):
    if path == None:
        if job == None:
            print("Set `--job` arg or set a `--path`")
            exit(1)
        path = job_cache_url(workspace_id, job)

    problematic_symlinks = get_problematic_symlinks(ferrocene_host)
    for location in problematic_symlinks:
        if os.path.exists(location):
            if os.path.islink(location):
                logging.debug(f"Removing problematic link `{location}`...")
                os.unlink(location)
            else:
                logging.debug(f"Removing problematic directory `{location}`...")
                shutil.rmtree(location)

    cache.store(path, "build", exclude=["build/metrics.json"])
    return

def subcommand_retrieve(ferrocene_host, workspace_id, path=None, job=None):
    if path == None:
        if job == None:
            print("Set `--job` arg or set a `--path`")
            exit(1)
        path = job_cache_url(workspace_id, job)

    cache.retrieve(path, ".", exclude=["build/metrics.json"])

    problematic_symlinks = get_problematic_symlinks(ferrocene_host)
    for location in problematic_symlinks:
        target = problematic_symlinks[location]
        if os.path.exists(target):
            logging.debug(f"Rebuilding problematic link to `{target}` at `{location}`...")
            parent = Path(location).parent
            if not os.path.exists(parent):
                os.makedirs(parent)
            os.symlink(target, location)
        else:
            logging.info(f"Unable to link to `{target}` at `{location}`, does not exist...")

    return

def arguments():
    parser = argparse.ArgumentParser(
        description="Store and retrieve job caches as s3 items",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    store_parser = subparsers.add_parser("store", help="Store a path as a `.tar.xz` at the specified AWS S3 URL (or local path)")
    store_parser.add_argument("--path", default=None, help="A local path or a `s3://<bucket>/<key>` url")
    store_parser.add_argument("--job", default=None, help="The job name to store")

    retrieve_parser = subparsers.add_parser("retrieve", help="Retrieve a `.tar.xz` from the specified AWS S3 URL (or local path) and unpack it to a path")
    retrieve_parser.add_argument("--path", default=None, help="A local path or a `s3://<bucket>/<key>` url")
    retrieve_parser.add_argument("--job", default=None, help="The job name to retrieve")

    return parser.parse_args()

def main():
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
        ferrocene_host = os.environ["FERROCENE_HOST"]
    except:
        print("Set FERROCENE_HOST environment to a Rust triple")
        exit(1)

    try:
        workspace_id = os.environ["CIRCLE_WORKFLOW_WORKSPACE_ID"]
    except:
        print("Set CIRCLE_WORKFLOW_WORKSPACE_ID environment to a Rust triple")
        exit(1)

    match args.subcommand:
        case "store":
            subcommand_store(ferrocene_host, workspace_id, job=args.job, path=args.path)
        case "retrieve":
            subcommand_retrieve(ferrocene_host, workspace_id, job=args.job, path=args.path)
        case _:
            print("Unknown command, see --help")
            exit(1)

if __name__ == "__main__":
    main()
