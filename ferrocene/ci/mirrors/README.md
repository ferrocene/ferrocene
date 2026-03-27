<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Mirrored CI dependencies

These exist to reduce our dependency on external infrastructure.
Most of these are built from a dockerfile which is present in this directory, but never executed in CI.
Use the dockerfiles if you need to regenerate the packages for some reason
(e.g. if you need to recompile with a newer version of GCC).

You can cross-compile all these images by adding `--platform=linux/amd64` or
`--platform=linux/arm64` respectively.

## Packages

### CMake

The version packaged on Ubuntu 20.04 is too old.

Dockerfile: `Dockerfile-cmake`
Build commands:
```
docker build -f Dockerfile-cmake -t cmake .
id=$(docker create cmake)
docker cp $id:/cmake/cmake-3.21.1.tar.xz .
docker rm $id
```

S3 paths:
- aarch64 host: `s3://ferrocene-ci-mirrors/manual/cmake/cmake-3.21.1-aarch64.tar.xz`
    - sha256sum: 1c96abe1e6a0c99a0e1fb73ea025e0ebb33f047b2b06586f3cfedd9630b59b08
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/cmake/cmake-3.21.1-x86_64.tar.xz`
    - sha256sum: 808b84201a7ea2cb87da4b8b16a8b3cf41155681623dedf1a7b90616e8e7e201

### Python

The version packaged on Ubuntu 20.04 is too old.

Dockerfile: `Dockerfile-python`
Build commands:
```
docker build -f Dockerfile-python -t python .
id=$(docker create python)
docker cp $id:/python/python-3.12.3.tar.xz .
docker rm $id
```

S3 paths:
- aarch64 host: `s3://ferrocene-ci-mirrors/manual/python/python-3.12.3-aarch64.tar.xz`
    - sha256sum: 6783b65e1e8d32339b267b81f69a1bb7ac495d3734a8751f86127a639ffdbc14
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/python/python-3.12.3-x86_64.tar.xz`
    - sha256sum: 68162562f29a6a7f697f40751f9caf9d693a7ccd30f0f5a04fd817c51a2b01f0

### GDB

We need version of GDB that doesn't fail debuginfo tests.
Upstream doesn't have very good version detection, so pin the same version that they do.

We have to build from source because the binary packages have too recent a version of glibc.
Note: this uses python 3.8, not our custom python 3.12, because distutils was removed in python 3.12.

    Dockerfile: `Dockerfile-gdb`
Build commands:
```
docker build -f Dockerfile-gdb -t gdb .
id=$(docker create gdb)
docker cp $id:/gdb-12.1/gdb-12.1.tar.xz .
docker rm $id
```

S3 paths:
- aarch64 host: `s3://ferrocene-ci-mirrors/manual/gdb/gdb-12.1-aarch64.tar.xz`
    - sha256sum: fe491420cb3b510cf8a5b0bfed38d7eb328309dfebf38a39d09880c353496ab6
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/gdb/gdb-12.1-x86_64.tar.xz`
    - sha256sum: 1a6b91e949b7d3c72008957290c7afc935d771aa46fc4af450a04a3b6ab2e28a

### musl-cross-make

This allows cross-compiling a GCC that knows how to statically link MUSL programs, without
needing a dynamically linked MUSL libc on the host.

Dockerfile: `Dockerfile-musl`
Build commands:
```
docker build -f Dockerfile-musl -t musl-cross .
id=$(docker create musl-cross)
docker cp $id:/musl-cross-make/musl-cross-make-aarch64.tar.xz .
docker cp $id:/musl-cross-make/musl-cross-make-x86_64.tar.xz .
docker rm $id
```
S3 paths:
- aarch64 host, aarch64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-aarch64.tar.xz`
    - sha256sum: 0b4d71509580e030984dd4604e8398a7b1b4fe40553241aa28012357505c6b1c
- aarch64 host, x86_64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-x86_64.tar.xz` 
    - sha256sum: f02418c5da0920efb509b6ce23b3b110fdd475efeabe8b7c9b02ce8d1b73a3b8
- x86_64 host, aarch64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-aarch64.tar.xz`
    - sha256sum: d7ce8d037d6c03224bcc2edda885551ed8b953fc3e3f14e48413e26e34e95afe
- x86_64 host, x86_64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-x86_64.tar.xz` 
    - sha256sum: bcf6ad12407853e5fbbdf0a6ec1ba5056735818d46963f998a9300ac9a6f0bf0

### ARM GCC

Mirrored from https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain.

https://developer.arm.com/-/media/Files/downloads/gnu/15.2.rel1/binrel/

S3 paths:
- MacOS: `s3://ferrocene-ci-mirrors/manual/arm-compiler/arm-gnu-toolchain-15.2.rel1-darwin-arm64-arm-none-eabi.pkg`
    - sha256sum: b93712026cec9f98a5d98dfec84e8096d32be3759642381e1982c4a5d2aa020b
- Windows: See `ferrocene/ci/scripts/setup-windows.sh`.
