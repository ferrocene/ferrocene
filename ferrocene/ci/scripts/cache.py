#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# dependencies = ["utils"]
#
# [tool.uv.sources]
# utils = { path = "utils", editable = true }
# ///

# This script primarily exists to workaround the fact that on Windows BSD/GNU tar
# are almost unusably slow.

import argparse
import logging
from utils import cache

def arguments():
    parser = argparse.ArgumentParser(
        description="Store and retrieve `.tar.xz` archives s3 items",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    store_parser = subparsers.add_parser("store", help="Store a path as a `.tar.xz` at the specified AWS S3 URL (or local path)")
    store_parser.add_argument("path", help="A local path or a `s3://<bucket>/<key>` url")
    store_parser.add_argument("in_dir", help="The directory to store")

    retrieve_parser = subparsers.add_parser("retrieve", help="Retrieve a `.tar.xz` from the specified AWS S3 URL (or local path) and unpack it to a path")
    retrieve_parser.add_argument("path", help="A local path or a `s3://<bucket>/<key>` url")
    retrieve_parser.add_argument("out_dir", help="The directory to expand into")

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

    match args.subcommand:
        case "store":
            cache.store(args.path, args.in_dir)

        case "retrieve":
            cache.retrieve(args.path, args.out_dir)
        case _:
            print("Unknown command, see --help")
            exit(1)

if __name__ == "__main__":
    main()
