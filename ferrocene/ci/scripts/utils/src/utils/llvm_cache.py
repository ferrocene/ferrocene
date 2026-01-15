# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import hashlib
import subprocess
import urllib.parse
import os


CACHE_BUCKET=(os.environ.get("CACHE_BUCKET") or "ferrocene-ci-caches")
CACHE_PREFIX=(os.environ.get("CACHE_PREFIX") or "")
CACHE_DIR="prebuilt-llvm"


def get_url(ferrocene_host):
    cache_hash = get_llvm_cache_hash()
    cache_file = f"{CACHE_PREFIX}{CACHE_DIR}/{ferrocene_host}-{cache_hash}.tar.zst"
    s3_url = f"s3://{CACHE_BUCKET}/{cache_file}"
    return urllib.parse.urlparse(s3_url)


def get_llvm_cache_hash():
    """
    Calculate a hash of the LLVM source code and all the files that could impact
    the LLVM build. This will be used as the cache key to avoid rebuilding LLVM
    from scratch every time.
    """
    m = hashlib.sha256()

    files = [
        "ferrocene/ci/scripts/llvm_cache.py", # __file__ is an absolute path
        "ferrocene/ci/configure.sh",
        "src/version",
    ];

    ls_files_cmd = ["git", "ls-files", "src/bootstrap", "ferrocene/ci/docker-images"]
    ls_files = subprocess.run(ls_files_cmd, check=True, capture_output=True, text=True)
    files += ls_files.stdout.split()

    files.sort()
    for file in files:
        # hashlib.file_digest added in 3.11
        f = open(file, mode="rb")
        buf = f.read()
        shasum = hashlib.sha256(buf)
        m.update(str.encode(shasum.hexdigest()))
        f.close()

    # Hashing all of the LLVM source code takes time. Instead we can simply get
    # the hash of the tree from git, saving time and achieving the same effect.
    ls_tree_cmd = ["git", "ls-tree", "HEAD", "src/llvm-project"]
    ls_tree = subprocess.run(ls_tree_cmd, check=True, capture_output=True, text=True)
    ls_tree_shasum = ls_tree.stdout.split()[2];
    m.update(str.encode(ls_tree_shasum))

    return m.hexdigest()
