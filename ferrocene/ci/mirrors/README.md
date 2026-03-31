<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Mirrored CI dependencies

These exist to reduce our dependency on external infrastructure.
Most of these are built from a dockerfile which is present in this directory, but never executed in CI.
Use the dockerfiles if you need to regenerate the packages for some reason
(e.g. if you need to recompile with a newer version of GCC).

You can cross-compile all these images by adding `--platform=linux/amd64` or
`--platform=linux/arm64` respectively. A list of shasums is kept in `hashes.txt` and checked
during the docker image build.

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
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/cmake/cmake-3.21.1-x86_64.tar.xz`

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
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/python/python-3.12.3-x86_64.tar.xz`

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
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/gdb/gdb-12.1-x86_64.tar.xz`

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
- aarch64 host, x86_64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-x86_64.tar.xz` 
- x86_64 host, aarch64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-aarch64.tar.xz`
- x86_64 host, x86_64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-x86_64.tar.xz` 

### ARM GCC

Mirrored from https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain.

https://developer.arm.com/-/media/Files/downloads/gnu/15.2.rel1/binrel/

S3 paths:
- MacOS: `s3://ferrocene-ci-mirrors/manual/arm-compiler/arm-gnu-toolchain-15.2.rel1-darwin-arm64-arm-none-eabi.pkg`
  See `ferrocene/ci/scripts/setup-darwin.sh`.
- Windows: `s3://ferrocene-ci-mirrors/manual/arm-compiler/gcc-arm-embedded.10.3.1.20251211.nupkg`
  See `ferrocene/ci/scripts/setup-windows.sh`.
