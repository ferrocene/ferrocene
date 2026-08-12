<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Mirrored CI dependencies

These exist to reduce our dependency on external infrastructure.
Most of these are built from a dockerfile which is present in this directory, but never executed in CI.
Use the dockerfiles if you need to regenerate the packages for some reason
(e.g. if you need to recompile with a newer version of GCC or for a different host operating system).

You need to build all dependencies with the exception of the gcc packages from the same commit. It is not supported to rebuild dependencies that have already been uploaded and the upload steps will not overwrite existing files.

Packages are built from source mirrored in `ferrocene-ci-artifacts`. If you need to update a package, first upload the
source package and then update the version in the Makefile and rebuild.

You need to execute the full build on an x68_64 and an aarch64 host system to build the dependencies for both architectures. After the dependencies have been built and uploaded, you can execute `update-hashes` which will download all dependencies and 
generate the hashes file used in the dockerfile.

## Authentication

all upload/download commands assume you have an authenticated session that has read permissions to ferrocene-ci-artifacts and read/write permissions to ferrocene-ci-mirrors.

The tasks have been split into download-src, build, upload to avoid timing out between downloads.

## Packages

### Building all packages

You can rebuild the full dependency chain from the bootstrap gcc to the final packages in a single go by using the *-all commands. If you set `BOOTSTRAP_GCC_FROM` and/or `GCC_FROM` the builds for the gcc packages will be skipped. See the bootstrap-gcc and gcc package below for details.

Build commands:
```
make download-all-src
make build-all
make upload-all
```

### Updating hashes

run `make update-hashes` to generate a hashes file matching the dependencies. The hashes file can be generated on any hardware architecture, but dependencies for both x86_64 and aarch64 must have been built from the same commit and on both architectures. `update-hashes` will respect `GCC_FROM` and pull the gcc packages from this commit instead.

### Building individual packages

You can build individual packages as laid out below. Note that various packages depend on each other an rebuilding leaf-packages may often rebuild their dependencies.

### bootstrap-gcc (gcc 11)
The GCC in our runner OS image is too old to compile a recent GCC version, so an intermediary GCC is required. GCC 11 has been chosen because it's old enough to compile without issues, and recent enough to build the latest GCC versions.

Build commands:
```
make download-bootstrap-gcc-src
make bootstrap-gcc
make upload-bootstrap-gcc
```

Instead of rebuilding the bootstrap GCC, you can reuse an existing build by setting the environment variable "BOOTSTRAP_GCC_FROM" to a commit that has a working GCC version. Unlike the main GCC, this commit can be different for the aarch64 and x86_64 builds.

### gcc (gcc-16.1.0)

Build commands:
```
make download-gcc-src
make gcc
make upload-gcc
```

Instead of rebuilding GCC, you can reuse an existing build by setting the environment variable "GCC_FROM" to a commit that has a working GCC version. The commit to use needs to be identical for both aarch64 and x86_64, otherwise hash generation will fail.

### utils (binutils and coreutils)

Binutils and coreutils are built in the same dockerfile as recent coreutils require a more recent version of binutils than provided by the os.

Build commands:
```
make download-utils-src
make utils
make upload-utils
```

### CMake

The version packaged on our runner OS is too old. Official binary releases up to 3.31.12 work both on aarch64 and x86_64, so this is a mirrored version of the tarball downloaded from the official release page.

### Python

The packaged version is too old. Please try to keep the version in sync with the version we use in uv.

Build commands:
```
make download-python-src
make python
make upload-python
```

S3 paths:
- aarch64 host: `s3://ferrocene-ci-mirrors/manual/python/python-<version>-aarch64-<commit>.tar.xz`
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/python/python-<version>-x86_64-<commit>.tar.xz`

### GDB

We need version of GDB that doesn't fail debuginfo tests.
Upstream doesn't have very good version detection, so pin the same version that they do.

We have to build from source because the binary packages have too recent a version of glibc.

Build commands:
```
make download-gdb-src
make gdb
make upload-gdb
```

S3 paths:
- aarch64 host: `s3://ferrocene-ci-mirrors/manual/gdb/gdb-12.1-aarch64-<commit>.tar.xz`
- x86_64  host: `s3://ferrocene-ci-mirrors/manual/gdb/gdb-12.1-x86_64-<commit>.tar.xz`

### musl-cross-make

This allows cross-compiling a GCC that knows how to statically link MUSL programs, without
needing a dynamically linked MUSL libc on the host.

Build commands:
```
make download-musl-cross-src
make musl-cross
make upload-musl-cross
```
S3 paths:
- aarch64 host, aarch64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-aarch64-<commit>.tar.xz`
- aarch64 host, x86_64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-aarch64-to-x86_64-<commit>.tar.xz`
- x86_64 host, aarch64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-aarch64-<commit>.tar.xz`
- x86_64 host, x86_64 target: `s3://ferrocene-ci-mirrors/manual/musl/musl-cross-make-x86_64-to-x86_64-<commit>.tar.xz`

### ARM GCC

Mirrored from https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain.

https://developer.arm.com/-/media/Files/downloads/gnu/15.2.rel1/binrel/

S3 paths:
- MacOS: `s3://ferrocene-ci-mirrors/manual/arm-compiler/arm-gnu-toolchain-15.2.rel1-darwin-arm64-arm-none-eabi.pkg`
  See `ferrocene/ci/scripts/setup-darwin.sh`.
- Windows: `s3://ferrocene-ci-mirrors/manual/arm-compiler/gcc-arm-embedded.10.3.1.20251211.nupkg`
  See `ferrocene/ci/scripts/setup-windows.sh`.
