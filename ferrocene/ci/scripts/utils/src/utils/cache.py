# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This module primarily exists to workaround the fact that on Windows BSD/GNU tar are almost
# unusably slow.

import tarfile
import boto3
import zstandard
import tempfile
import os
import shutil
import logging

def parse_s3_url(s3_url):
    assert(s3_url.startswith("s3://"))
    [bucket, key] = s3_url.split("//")[1].split("/", 1)
    return [bucket, key]


def exists(path):
    """
    Determine if a specific cache entry exists.
    """

    if path.startswith("s3://"):
        [bucket, key] = parse_s3_url(path)

        s3 = boto3.client("s3")
        try:
            response = s3.get_object(Bucket=bucket, Key=key)
            return True
        except s3.exceptions.NoSuchKey:
            return False
    else:
        return os.path.isfile(path)

def retrieve(path, out_dir):
    """
    Retrieve a zstd compressed tarball from `path` (an `s3://` url or local path) and unpack it into `out_dir`.
    """

    if path.startswith("s3://"):
        [bucket, key] = parse_s3_url(path)

        s3 = boto3.client("s3")
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
            tarball.extractall(path=out_dir, filter="data")
    logging.info(f"Done decompression and unarchiving of `{path}` to `{out_dir}`.")

    return


def store(path, in_dir):
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
                tarball.add(in_dir, recursive=True) # Always do relative to the directory passed.
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
