#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import os
import logging
import shutil
import platform
from pathlib import Path


def get_problematic_symlinks(ferrocene_host):
    """
    In the build directory, there exists several cyclic symlinks.

    We need to tear those down and rebuild them on restore, because Github Actions' upload artifact
    action doesn't understand this concept, and OOMs.
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


def subcommand_pre_upload(ferrocene_host):
    problematic_symlinks = get_problematic_symlinks(ferrocene_host)
    for location in problematic_symlinks:
        # Windows gets *extremely* confused by symlink directories
        if platform.system() == "Windows":
            logging.info(f"Removing cyclic link `{location}`")
            os.unlink(location)
        else:
            if os.path.islink(location):
                logging.info(f"Removing cyclic link `{location}`")
                os.unlink(location)
            else:
                logging.info(f"Removing cyclic directory link `{location}`")
                shutil.rmtree(location)

    for path in ["build/cache", "build/tmp"]:
        if os.path.exists(path):
            logging.info(f"Removing {path}")
            shutil.rmtree(path)
        else:
            logging.info(f"Skipped removing {path}, does not exist")

    return


def subcommand_post_download(ferrocene_host):
    problematic_symlinks = get_problematic_symlinks(ferrocene_host)
    for location in problematic_symlinks:
        target = problematic_symlinks[location]
        if os.path.exists(target):
            logging.info(f"Rebuilding cyclic link to `{target}` at `{location}`")
            parent = Path(location).parent
            if not os.path.exists(parent):
                os.makedirs(parent)
            os.symlink(target, location)
        else:
            logging.info(f"Unable to link to `{target}` at `{location}`, does not exist")

    return


def arguments():
    parser = argparse.ArgumentParser(
        description="Handle cyclic links in the build directory",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    store_parser = subparsers.add_parser("pre-upload", help="Prepare the build directory for cache upload.")

    retrieve_parser = subparsers.add_parser("post-download", help="Restore the build directory to a usable state (eg reconsitute cyclic symlinks).")

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

    match args.subcommand:
        case "pre-upload":
            subcommand_pre_upload(ferrocene_host)
        case "post-download":
            subcommand_post_download(ferrocene_host)
        case _:
            print("Unknown command, see --help")
            exit(1)


if __name__ == "__main__":
    main()
