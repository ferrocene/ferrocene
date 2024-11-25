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
import shutil
import logging

def parse_s3_url(s3_url):
    assert(s3_url.startswith("s3://"))
    [bucket, key] = s3_url.split("//")[1].split("/", 1)
    return [bucket, key]


def retrieve(path, out_dir, exclude=[]):
    """
    Retrieve a zstd compressed tarball from `path` (an `s3://` url or local path) and unpack it into `out_dir`.
    """

    if path.startswith("s3://"):
        [bucket, key] = parse_s3_url(path)

        s3 = boto3.client("s3")
        logging.info(f"Getting s3 object at `{path}`...")
        response = s3.get_object(Bucket=bucket, Key=key)
        stream = response["Body"]
        logging.info(f"Got s3 object at `{path}`...")
    else:
        stream = open(path, "rb")
        logging.info(f"Got archive at `{path}`...")
        
    zstd_decompressor = zstandard.ZstdDecompressor()

    with zstd_decompressor.stream_reader(stream) as zstd_stream:
        logging.info("Began decompression...")
        # It is important for Windows that the tarball was created with `--dereference`
        with tarfile.open(mode="r|", fileobj=zstd_stream) as tarball:
            logging.info("Began unarchiving...")

            def exclusions(members):
                nonlocal exclude
                for tarinfo in members:
                    if tarinfo.name in exclude:
                        logging.debug(f"Excluding {tarinfo.name}...") 
                    else:
                        yield tarinfo
            
            tarball.extractall(path=out_dir, members=exclusions(tarball), filter="data")
    logging.info(f"Done decompression and unarchiving of `{path}` to `{out_dir}`.")

    return


def store(path, in_dir, exclude=[]):
    """
    Store write a zstd compressed tarball of `in_dir` to the `path` (an `s3://` url or local path) .
    """

    zstd_compressor = zstandard.ZstdCompressor()
    
    # It proved to be quite difficult to stream a zstd compression stream
    # directly to s3 as boto3 requires a very file-like option for retries.
    with tempfile.NamedTemporaryFile(delete=False) as temporary_file:
        with zstd_compressor.stream_writer(temporary_file) as zstd_stream:
            logging.info(f"Began compression to `{temporary_file.name}`...")
            with tarfile.TarFile.open(mode='w|', dereference=True, fileobj=zstd_stream) as tarball:
                logging.info(f"Began archiving `{in_dir}`...")
                
                def exclusions(tarinfo):
                    nonlocal exclude
                    if tarinfo.name in exclude:
                        logging.debug(f"Excluding {tarinfo.name}...") 
                    else:
                        return tarinfo

                tarball.add(in_dir, recursive=True, filter=exclusions) # Always do relative to the directory passed.
                tarball.close()
            zstd_stream.flush()
            zstd_stream.close()
        temporary_file.close()
        logging.info(f"Done compression and archiving to `{temporary_file.name}`.")

        if path.startswith("s3://"):
            [bucket, key] = parse_s3_url(path)
            logging.info(f"Beginning upload of `{temporary_file.name}` to `{path}`...")
            s3 = boto3.client('s3')
            s3.upload_file(temporary_file.name, bucket, key)
            logging.info(f"Done upload to `{path}`.")

            os.unlink(temporary_file.name)
            logging.info(f"Removed `{temporary_file.name}`.")
        else:
            shutil.move(temporary_file.name, path)
            logging.info(f"Moved `{temporary_file.name}` to `{path}`")
    return

def arguments():
    parser = argparse.ArgumentParser(
        description="Store and retrieve `.tar.xz` archives s3 items",
    )
    parser.add_argument('-v', '--verbose', action='count', default=0)
    subparsers = parser.add_subparsers(dest="subcommand", help="sub-command help")

    store_parser = subparsers.add_parser("store", help="Store a path as a `.tar.xz` at the specified AWS S3 URL (or local path)")
    store_parser.add_argument("path", help="A local path or a `s3://<bucket>/<key>` url")
    store_parser.add_argument("in_dir", help="The directory to store")
    store_parser.add_argument("--exclude", action="append", default=[], help="Paths to exclude")

    retrieve_parser = subparsers.add_parser("retrieve", help="Retrieve a `.tar.xz` from the specified AWS S3 URL (or local path) and unpack it to a path")
    retrieve_parser.add_argument("path", help="A local path or a `s3://<bucket>/<key>` url")
    retrieve_parser.add_argument("out_dir", help="The directory to expand into")
    retrieve_parser.add_argument("--exclude", action="append", default=[], help="Paths to exclude")

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
            store(args.path, args.in_dir, exclude=args.exclude)
        case "retrieve":
            retrieve(args.path, args.out_dir, exclude=args.exclude)
        case _:
            print("Unknown command, see --help")
            exit(1)

if __name__ == "__main__":
    main()
