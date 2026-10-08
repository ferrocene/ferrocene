# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import functools
import hashlib
import subprocess
import urllib.parse
from utils import docker_images


CACHE_BUCKET = "ferrocene-ci-caches"
CACHE_PREFIX = "prebuilt-llvm"


def get_s3_url(ferrocene_host, override_hash: str | None = None):
    cache_hash = override_hash or get_llvm_cache_hash()
    cache_file = f"{CACHE_PREFIX}/{ferrocene_host}-{cache_hash}.tar.zst"
    s3_url = f"s3://{CACHE_BUCKET}/{cache_file}"
    return urllib.parse.urlparse(s3_url)


@functools.cache
def get_llvm_cache_hash():
    """
    Calculate a hash of the LLVM source code and all the files that could impact
    the LLVM build. This will be used as the cache key to avoid rebuilding LLVM
    from scratch every time.
    """
    m = hashlib.sha256()
    m.update(str.encode(docker_images.calculate_hash()))

    files = [
        "ferrocene/ci/scripts/llvm_cache.py",  # __file__ is an absolute path
        "ferrocene/ci/configure.sh",
        "src/version",
    ]

    ls_files_cmd = ["git", "ls-files", "src/bootstrap"]
    ls_files = subprocess.run(ls_files_cmd, check=True, capture_output=True, text=True)
    files += ls_files.stdout.split()

    files.sort()
    for file in files:
        filename = file.encode("utf-8")
        m.update(f"{len(filename)}|{filename}".encode())
        with open(file, "rb") as f:
            buf = f.read()
            m.update(f"{len(buf)}|".encode())
            m.update(buf)

    # Hashing all of the LLVM source code takes time. Instead we can simply get
    # the hash of the tree from git, saving time and achieving the same effect.
    ls_tree_cmd = ["git", "ls-tree", "HEAD", "src/llvm-project"]
    ls_tree = subprocess.run(ls_tree_cmd, check=True, capture_output=True, text=True)
    ls_tree_shasum = ls_tree.stdout.split()[2]
    m.update(str.encode(ls_tree_shasum))

    return m.hexdigest()
