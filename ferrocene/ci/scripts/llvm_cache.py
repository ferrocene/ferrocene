#!/usr/bin/env -S uv run
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# /// script
# requires-python = ">=3.12"
# dependencies = ["utils"]
#
# [tool.uv.sources]
# utils = { path = "utils", editable = true }
# ///

import argparse
import subprocess
import os
import shutil
import pathlib
import sys
from utils import cache, llvm_cache
import logging


# Note that this *ignores* symlinks. If you need a binary that's actually a
# symlink please add to the list the binary the symlink points *to*.
KEEP_LLVM_BINARIES=[
    # Needed for the `llvm-tools` component
    "llvm-cov",
    "llvm-nm",
    "llvm-objcopy",
    "llvm-objdump",
    "llvm-profdata",
    "llvm-readobj",
    "llvm-size",
    "llvm-strip",
    "llvm-ar",
    "llvm-as",
    "llvm-dis",
    "llc",
    "opt",
    # Used by `llvm-ar`
    "llvm-ranlib",
    "llvm-dlltool",
    "llvm-lib",
    # Used by `llvm-objdump`
    "llvm-otool",
    # Used by `llvm-objcopy`, `llvm-strip`
    "llvm-bitcode-strip",
    # Used by `llvm-readobj`
    "llvm-readelf",
    # Needed to link `rustc` with LLVM
    "llvm-config",
    # Needed by the Rust test suite
    "llvm-bcanalyzer",
    "llvm-dwarfdump",
    "FileCheck",
    # Needed to resolve symbols
    "llvm-dwp",
    # Needed to build LLD
    "llvm-tblgen",
    # Needed to strip debug info in aarch64-darwin
    "llvm-install-name-tool",
]


def arguments():
    parser = argparse.ArgumentParser(
        description="Report various data about LLVM caches",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    prepare_parser = subparsers.add_parser("prepare", help="Build and cache LLVM")
    prepare_parser.add_argument("--url", help="Manually set the output `tar.zst` location")

    download_parser = subparsers.add_parser("download", help="Download the existing LLVM cache")
    download_parser.add_argument("--url", help="Manually set the input `tar.zst` location")

    download_parser = subparsers.add_parser("exists", help="Check if the LLVM cache exists")
    download_parser.add_argument("--url", help="Manually set the input `tar.zst` location")

    url_parser = subparsers.add_parser("url", help="Calculate the LLVM cache URL")
    hash_parser = subparsers.add_parser("hash", help="Calculate the LLVM cache hash")

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

    # match added in 3.10
    if args.subcommand == "url":
        subcommand_url(ferrocene_host)
    elif args.subcommand == "hash":
        subcommand_hash()
    elif args.subcommand == "download":
        subcommand_download(ferrocene_host, args.url)
    elif args.subcommand == "prepare":
        subcommand_prepare(ferrocene_host, args.url)
    elif args.subcommand == "exists":
        subcommand_exists(ferrocene_host, args.url)
    else:
        print(f"Unknown command {args.subcommand}")

def subcommand_download(ferrocene_host, url):
    if url == None:
        url = llvm_cache.get_url(ferrocene_host).geturl()

    cache.retrieve(url, ".")

def subcommand_exists(ferrocene_host, url):
    if url == None:
        url = llvm_cache.get_url(ferrocene_host).geturl()

    if cache.exists(url):
        sys.exit(0)
    else:
        sys.exit(1)

def subcommand_prepare(ferrocene_host, url):
    if url == None:
        url = llvm_cache.get_url(ferrocene_host).geturl()

    tarball = prepare_llvm_build(ferrocene_host)
    cache.store(url, tarball)

def subcommand_s3_url(ferrocene_host):
    s3_url = llvm_cache.get_url(ferrocene_host)
    print(s3_url.geturl())

def subcommand_hash():
    hash_val = llvm_cache.get_llvm_cache_hash()
    print(hash_val)

def prepare_llvm_build(ferrocene_host):
    """
    Build LLVM and generate a tarball we can cache with all the build artifacts.
    """
    build_cmd = [sys.executable, "x.py", "build", "src/llvm-project"]
    try:
        parallelism = os.environ["LLVM_BUILD_PARALLELISM"];
        if parallelism:
            build_cmd += ["-j", parallelism]
    except:
        pass
    build = subprocess.run(build_cmd, check=True, stdout=sys.stdout, stderr=sys.stderr)

    # The llvm/build directory contains a *copy* of all the binaries, plus the
    # intermediate object files and other build artifacts we don't need. To
    # save space in the cached tarball remove it.
    #
    # On Windows, we skip this pruning since it *does* need intermediate
    # object files. (Notably, `llvm/Config/llvm-config.h` and many lib objects)
    is_windows = sys.platform.startswith('win32') or sys.platform.startswith('cygwin')
    if not is_windows:
        shutil.rmtree(f"build/{ferrocene_host}/llvm/build")

        # Rustbuild is looking in `llvm/build/bin` instead of `bin` when checking
        # for an existing `llvm-config` binary. Create a symlink to make sure it
        # can still detect the existing build.
        os.makedirs(f"build/{ferrocene_host}/llvm/build")
        os.symlink(f"../bin", f"build/{ferrocene_host}/llvm/build/bin")

    # The LLVM distribution as of 2021-08-23 contains more than 1GB of
    # binaries, but we only need a small subset of them. This "deletes" the
    # binaries we don't need to avoid caching them.
    #
    # Note that this does not *actually* remove the files, it replaces them
    # with a non-executable file containing a note that the file is deleted.
    # This is because LLVM's CMake configuration explicitly checks for all the
    # binaries to be present, and if the binaries are straight up missing
    # building anything depending on LLVM (like the sanitizers) will fail.
    dirname = f"build/{ferrocene_host}/llvm/bin"
    files = os.listdir(dirname)
    for file in files:
        # Ignore symlinks, to avoid `foo` being removed if `foo` is needed but
        # a symlink called `bar` pointing to `foo` is not needed.
        if os.path.islink(file):
            continue

        name = os.path.basename(file)
        if is_windows:
            path = pathlib.Path(name)
            path = path.with_suffix('')
            name = path.as_posix()

        if name in KEEP_LLVM_BINARIES:
            logging.debug(f"Skipped {file}")
            continue
        else:
            logging.debug(f"Soft-removing {file}")
            f = open(os.path.join(dirname, file), "wt")
            f.write(f"""
                #!/usr/bin/env sh
                echo "Binary {file} soft-removed by ferrocene/ci/scripts/llvm_cache.py"
                exit 1
            """)

    return f"build/{ferrocene_host}/llvm"


if __name__ == "__main__":
    main()
