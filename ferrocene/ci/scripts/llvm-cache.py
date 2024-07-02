#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import subprocess
import hashlib
import os
import io
import shutil
import pathlib
import tarfile
import sys

CACHE_BUCKET="ferrocene-ci-caches"
CACHE_PREFIX="prebuilt-llvm"


TARBALL_PATH = "llvm-cache.tar"
COMPRESSED_TARBALL_PATH = f"{TARBALL_PATH}.zst"

# Note that this *ignores* symlinks. If you need a binary that's actually a
# symlink please add to the list the binary the symlink points *to*.
KEEP_LLVM_BINARIES=[
    # Needed for the `llvm-tools` component
    "llvm-cov"
    "llvm-nm"
    "llvm-objcopy"
    "llvm-objdump"
    "llvm-profdata"
    "llvm-readobj"
    "llvm-size"
    "llvm-strip"
    "llvm-ar"
    "llvm-as"
    "llvm-dis"
    "llc"
    "opt"
    # Needed to link `rustc` with LLVM
    "llvm-config"
    # Needed by the Rust test suite
    "llvm-bcanalyzer"
    "llvm-dwarfdump"
    "FileCheck"
    # Needed to resolve symbols
    "llvm-dwp"
    # Needed to build LLD
    "llvm-tblgen"
]

def arguments():
    parser = argparse.ArgumentParser(
        prog="llvm-cache.py",
        description="Report various data about LLVM caches",
    )
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    prepare_parser = subparsers.add_parser("prepare", help="Build and cache LLVM")
    
    download_parser = subparsers.add_parser("download", help="Download the existing LLVM cache")

    s3_url_parser = subparsers.add_parser("s3-url", help="Calculate the LLVM cache URL")

    return parser.parse_args()


def run():
    args = arguments()
    try:
        ferrocene_host = os.environ["FERROCENE_HOST"]
    except:
        print("Set FERROCENE_HOST environment to a Rust triple")
        exit(1)

    # match added in 3.10
    if args.subcommand == "s3-url":
        subcommand_s3_url(ferrocene_host)
    elif args.subcommand == "download":
        subcommand_download(ferrocene_host)
    elif args.subcommand == "prepare":
        subcommand_prepare(ferrocene_host)
    else:
        print(f"Unknown command {args.subcommand}")

def subcommand_download(ferrocene_host):
    s3_url = get_s3_url(ferrocene_host)

    s3_cp_cmd = f"aws s3 cp {s3_url} - | zstd --decompress - -o {TARBALL_PATH}"
    s3_cp = subprocess.run(s3_cp_cmd, shell=True, text=True, stdout=sys.stdout)
    if s3_cp.returncode != 0:
        print(f"`{s3_cp_cmd}` did not work")
        exit(1)
    os.remove(COMPRESSED_TARBALL_PATH)
    
    # Use python tar to avoid Windows 'weirdness'
    tar = tarfile.open(TARBALL_PATH, "r")
    tar.extractall(filter="fully_trusted")
    tar.close()

    os.remove(TARBALL_PATH)

def subcommand_prepare(ferrocene_host):
    tarball = build_llvm_tarball(ferrocene_host)
    print(tarball)

def subcommand_s3_url(ferrocene_host):
    s3_url = get_s3_url(ferrocene_host)
    print(s3_url)

def build_llvm_tarball(ferrocene_host):
    """
    Build LLVM and generate a tarball we can cache with all the build artifacts.
    """
    build_cmd = f"python x.py build src/llvm-project"
    try: 
        parallelism = os.environ["LLVM_BUILD_PARALLELISM"];
        if parallelism:
            build_cmd += "-j {parallelism}"
    except:
        True
    build = subprocess.run(build_cmd, shell=True, text=True, stdout=sys.stdout)
    if build.returncode != 0:
        print(f"`{build_cmd}` did not work")
        exit(1)

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
            path.with_suffix('')
            name = path

        if name in KEEP_LLVM_BINARIES:
            continue

        f = open(os.path.join(dirname, file), "wt")
        f.write("""
            #!/usr/bin/env sh
            echo "File soft-removed by ferrocene/ci/scripts/llvm-cache.py"
            exit 1
        """)

    # Use python tar to avoid Windows 'weirdness'
    tar = tarfile.open(TARBALL_PATH, "w")
    tar.add(f"build/{ferrocene_host}/llvm")
    tar.close()

    compress_cmd = f"zstd -1 -T0 {TARBALL_PATH} -o {COMPRESSED_TARBALL_PATH}"
    compress = subprocess.run(compress_cmd, text=True, stdout=sys.stdout)
    if compress.returncode != 0:
        print(f"`{compress_cmd}` did not work")
        exit(1)

    os.remove(TARBALL_PATH)
    return COMPRESSED_TARBALL_PATH

def get_s3_url(ferrocene_host):
    cache_hash = get_llvm_cache_hash()
    cache_file = f"{CACHE_PREFIX}/{ferrocene_host}-{cache_hash}.tar.zst"
    s3_url = f"s3://{CACHE_BUCKET}/{cache_file}"
    return s3_url

def get_llvm_cache_hash():
    """
    Calculate a hash of the LLVM source code and all the files that could impact
    the LLVM build. This will be used as the cache key to avoid rebuilding LLVM
    from scratch every time.
    """
    m = hashlib.sha256()

    files = [
        "ferrocene/ci/scripts/llvm-cache.py", # __file__ is an absolute path
        "ferrocene/ci/configure.sh",
        "src/version",
    ];
    
    ls_files_cmd = "git ls-files src/bootstrap ferrocene/ci/docker-images"
    ls_files = subprocess.run(ls_files_cmd, capture_output=True, text=True)
    if ls_files.returncode != 0:
        print(f"`{ls_files_cmd}` did not work")
        exit(1)
    files += ls_files.stdout.split()

    files.sort()
    for file in files:
        # hashlib.file_digest added in 3.11
        f = open(file, mode="rb")
        buf = f.read()
        shasum = hashlib.sha256(buf)
        m.update(str.encode(shasum.hexdigest()))

    # Hashing all of the LLVM source code takes time. Instead we can simply get
    # the hash of the tree from git, saving time and achieving the same effect.
    ls_tree_cmd = "git ls-tree HEAD src/llvm-project --object-only"
    ls_tree = subprocess.run(ls_tree_cmd, capture_output=True, text=True)
    if ls_tree.returncode != 0:
        print(f"`{ls_tree_cmd}` did not work")
        exit(1)
    m.update(str.encode(ls_tree.stdout))

    return m.hexdigest()


if __name__ == "__main__":
    run()
