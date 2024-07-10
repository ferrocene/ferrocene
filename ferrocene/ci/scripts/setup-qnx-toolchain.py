#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

import argparse
import tarfile
import boto3
import zstandard

TARBALL_BUCKET = "ferrocene-ci-mirrors"
TARBALL_KEY = "manual/qnx/qnx710-deployment.tar.xz"

def main():
    s3 = boto3.client('s3')
    response = s3.get_object(Bucket=TARBALL_BUCKET, Key=TARBALL_KEY)
    print(f"Got s3 object at s3://{TARBALL_BUCKET}/{TARBALL_KEY}..")
    body = response['Body']

    zstd_decompressor = zstandard.ZstdDecompressor()

    with zstd_decompressor.stream_reader(body) as stream:
        print("Began decompressing...")
        with tarfile.open(mode="r|", fileobj=stream) as tarball:
            print("Began extraction...")
            tarball.extractall(filter="data")


if __name__ == "__main__":
    main()
