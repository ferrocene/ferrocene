#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script primarily exists to workaround the fact that on Windows BSD/GNU tar
# are almost unusably slow. 

import argparse
import tarfile
import boto3
import zstandard
import io
import threading
import tempfile
import os
import logging

def parse_s3_url(s3_url):
    assert(s3_url.startswith("s3://"))
    [bucket, key] = s3_url.split("//")[1].split("/", 1)
    return [bucket, key]


def retrieve(s3_url, out_dir):
    """
    Retrieve a zstd compressed tarball from `s3_url` and unpack it into `out_dir`.
    """
    [bucket, key] = parse_s3_url(s3_url)

    s3 = boto3.client('s3')
    response = s3.get_object(Bucket=bucket, Key=key)
    logging.info(f"Got s3 object at `{s3_url}`...")
    body = response['Body']

    zstd_decompressor = zstandard.ZstdDecompressor()

    with zstd_decompressor.stream_reader(body) as stream:
        logging.info("Began decompression...")
        # It is important for Windows that the tarball was created with `--dereference`
        with tarfile.open(mode="r|", fileobj=stream) as tarball:
            logging.info("Began unarchiving...")
            tarball.extractall(path=out_dir, filter="data")
    logging.info(f"Done decompression and unarchiving of `{s3_url}` to `{out_dir}`.")

    return


def store(s3_url, in_dir, upload=True, cleanup=True):
    """
    Store write a zstd compressed tarball of `in_dir` to the S3 URL `s3_url`.

    Pass `upload=False` to avoid the upload to s3 during testing.
    """
    [bucket, key] = parse_s3_url(s3_url)

    zstd_compressor = zstandard.ZstdCompressor()
    
    # It proved to be quite difficult to stream a zstd compression stream
    # directly to s3 as boto3 requires a very file-like option for retries.
    with tempfile.NamedTemporaryFile(delete=False) as temporary_file:
        with zstd_compressor.stream_writer(temporary_file) as zstd_stream:
            logging.info(f"Began compression to `{temporary_file.name}`...")
            with tarfile.TarFile.open(mode='w|', fileobj=zstd_stream) as tarball:
                logging.info("Began archiving...")
                os.chdir(in_dir)
                tarball.add(".") # Always do relative to the directory passed.
        logging.info(f"Done compression and archiving to `{temporary_file.name}`.")

        if upload == True:
            logging.info(f"Beginning upload of `{temporary_file.name}` to `{s3_url}`...")
            s3 = boto3.client('s3')
            s3.upload_file(temporary_file.name, bucket, key)
            logging.info(f"Done upload to `{s3_url}`.")
        
        if cleanup == True:
            os.unlink(temporary_file.name)
            logging.info(f"Removed `{temporary_file.name}`.")
    
    return

def arguments():
    parser = argparse.ArgumentParser(
        description="Store and retrieve `.tar.xz` archives s3 items",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    store_parser = subparsers.add_parser("store", help="Store a path as a `.tar.xz` at the specified AWS S3 URL")
    store_parser.add_argument("s3_url", help="The `s3://<bucket>/<key>` url")
    store_parser.add_argument("in_dir", help="The directory to store")
    store_parser.add_argument("--upload", default=True, help="If the tarball should be actually be uploaded to s3")
    store_parser.add_argument("--cleanup", default=True, help="If the tarball should be removed when finished")

    retrieve_parser = subparsers.add_parser("retrieve", help="Retrieve a `.tar.xz` from the specified AWS S3 URL and unpack it to a path")
    retrieve_parser.add_argument("s3_url", help="The `s3://<bucket>/<key>` url")
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
            store(args.s3_url, args.in_dir, upload=args.upload)

        case "retrieve":
            retrieve(args.s3_url, args.out_dir)
        case _:
            print("Unknown command, see --help")
            exit(1)

if __name__ == "__main__":
    main()
