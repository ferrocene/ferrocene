#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">= 3.12"
# dependencies = []
# ///

import argparse
import os
import logging
import shutil
import platform
import tarfile
from pathlib import Path

build_directory = Path("build")
tarball_location = Path("build.tar")

def get_problematic_symlinks(ferrocene_host):
    """
    In the build directory, there exists several cyclic symlinks.

    We need to tear those down and rebuild them on restore, because many tools 
    don't understand this concept, and OOM.
    """
    return {
        Path("build", ferrocene_host, "stage0-sysroot", "lib", "rustlib", "rustc-src"): os.getcwd(),
        Path("build", ferrocene_host, "stage0-sysroot", "lib", "rustlib", "src"): os.getcwd(),
        Path("build", ferrocene_host, "stage1", "lib", "rustlib", "rustc-src"): os.getcwd(),
        Path("build", ferrocene_host, "stage1", "lib", "rustlib", "src"): os.getcwd(),
        Path("build", ferrocene_host, "stage2", "lib", "rustlib", "rustc-src"): os.getcwd(),
        Path("build", ferrocene_host, "stage2", "lib", "rustlib", "src"): os.getcwd(),
        Path("build", "host"): Path("build", ferrocene_host),
    }


def subcommand_pre_upload(ferrocene_host):
    problematic_symlinks = get_problematic_symlinks(ferrocene_host)
    for location in problematic_symlinks:
        try:
            # Windows gets *extremely* confused by symlink directories
            if platform.system() == "Windows":
                try:
                    shutil.rmtree(location)
                    logging.info(f"Removed cyclic link `{location}`, via rmtree")
                except Exception as e:
                    os.unlink(location)
                    logging.info(f"Removed cyclic link `{location}`, via unlink")
            else:
                if os.path.islink(location):
                    os.unlink(location)
                    logging.info(f"Removed cyclic link `{location}` via `unlink`")
                else:
                    shutil.rmtree(location)
                    logging.info(f"Removed cyclic link `{location}` via `rmtree`")
        except Exception as e:
            logging.warning(f"Unable to remove {location}: {e}")

    for location in [Path("build", "cache"), Path("build", "tmp")]:
        if location.exists():
            logging.info(f"Removing {location}")
            shutil.rmtree(location)
        else:
            logging.warning(f"Skipped removing {location}, does not exist")
    return


def subcommand_post_download(ferrocene_host):
    """
    Rebuild the symlinks.
    """
    # It is important for Windows that the tarball was created with `--dereference`

    problematic_symlinks = get_problematic_symlinks(ferrocene_host)
    for location in problematic_symlinks:
        target = problematic_symlinks[location]
        if Path(target).exists():
            logging.info(f"Rebuilding cyclic link to `{target}` at `{location}`")
            parent = location.parent
            if not os.path.exists(parent):
                os.makedirs(parent)
            os.symlink(target, location, target_is_directory=True)
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
