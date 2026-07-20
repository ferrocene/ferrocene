# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

## This builds a docker image that contains only the binaries
## of the desired RISC-V toolchain at the root.
##
## This image can then later be mounted in the builder container at
## the expected location and used. This allows us to split the docker
## image into manageable parts, pulling only the bits that we actually need.
##
## Use a multi-platform build to build this image, this works without emulation since we're
## only downloading and unpacking a file and have no dependencies on the builder platform
##
## docker build --tag harbor.infra.ferrous-systems.net/ferrocene-images/riscv:ubuntu2004-20240407 --file ferrocene/ci/docker-images/components/riscv-toolchain.dockerfile --platform "linux/arm64","linux/amd64" .
ARG TARGETPLATFORM
ARG BUILDPLATFORM

FROM --platform=$BUILDPLATFORM ubuntu:24.04 AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

RUN <<-EOF
    set -xe
    echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
    alias apt-install="apt-get install -yq --no-install-recommends --option Dpkg::Options::=--force-confnew"

    apt-get update

    # Needed to install AWS CLI during the build:
    apt-install \
        curl \
        ca-certificates
EOF

# Install a version of the RISC-V 64 bit toolchain that is modern, as the packaged ones on Ubuntu 20.04
# are too old to be useful (they don't know about the `z` ISA flag).
#
# We get x86_64 Linux binaries from https://www.embecosm.com/resources/tool-chain-downloads/#riscv-linux
#
# These builds are unfortunately only for x86_64. To add new targets, we'll need to use a newer Ubuntu
# image, or build our own from source via https://github.com/riscv-collab/riscv-gnu-toolchain.

RUN <<-EOF
    mkdir /opt/riscv-toolchain/
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        PACKAGE_URL="https://buildbot.embecosm.com/job/riscv64-linux-gcc-ubuntu2004/20/artifact/riscv64-embecosm-linux-gcc-ubuntu2004-20240407.tar.gz"
        SHA="ddebec169a6c3a29b6cd0133f54f93c8f7addab9b3b5fb5366fa73dda834d7b7"
        curl -Lo /tmp/riscv-toolchain.tar.gz $PACKAGE_URL
        echo "$SHA /tmp/riscv-toolchain.tar.gz" | sha256sum -c
        tar xf /tmp/riscv-toolchain.tar.gz -C /opt/riscv-toolchain --strip-components=1
        rm /tmp/riscv-toolchain.tar.gz
        /opt/riscv-toolchain/bin/riscv64-unknown-linux-gnu-gcc --version

        # echo "$SHA $PACKAGE_URL" >> /ferrocene/packages/downloads
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        echo "riscv64gc-unknown-linux-gnu toolchain setup skipped on aarch64-unknown-linux-gnu host."
    else
        echo "Unsupported platform"
        exit 1
    fi
EOF

FROM scratch

COPY --from=builder /opt/riscv-toolchain/ /