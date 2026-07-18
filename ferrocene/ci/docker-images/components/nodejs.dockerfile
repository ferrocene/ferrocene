# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

## This builds a docker image that contains only the binaries
## of the desired nodejs toolchain at the root.
##
## This image can then later be mounted in the builder container at
## the expected location and used. This allows us to split the docker
## image into manageable parts, pulling only the bits that we actually need.
##
## Use a multi-platform build to build this image, this works without emulation since we're
## only downloading and unpacking a file and have no dependencies on the builder platform
##
## docker build --tag harbor.infra.ferrous-systems.net/ferrocene-images/nodejs:v16.20.2 --file ferrocene/ci/docker-images/components/nodejs.dockerfile --platform "linux/arm64","linux/amd64" .
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
        ca-certificates \
        xz-utils
EOF

# Install a recent NodeJS version, as the one shipped in Ubuntu 20.04 is too
# old to run the rustdoc-js test suite. Note that we cannot install the most
# recent NodeJS LTS release, as it's built with a more recent glibc version
# than the one shipped with Ubuntu 20.04.
#
# If you are reading this comment because you need to upgrade past NodeJS 16,
# you will unfortunately need to change this to build NodeJS from source.
RUN <<-EOF
    set -xe
    if [ "$TARGETPLATFORM" = "linux/amd64" ]; then
        ARCH="x64"
        SHA="874463523f26ed528634580247f403d200ba17a31adf2de98a7b124c6eb33d87"
    elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then
        ARCH="arm64"
        SHA="e88d86154d1ce53dc52fd74d79d4bfdf0b05f58c0bb2639adfa36e9378b770c4"
    else
        echo "Unsupported platform"
        exit 1
    fi
    mkdir /opt/nodejs
    PACKAGE_URL="https://nodejs.org/dist/v16.20.2/node-v16.20.2-linux-${ARCH}.tar.xz"
    curl -Lo /tmp/nodejs.tar.xz $PACKAGE_URL
    echo "${SHA} /tmp/nodejs.tar.xz" | sha256sum -c
    tar xf /tmp/nodejs.tar.xz -C /opt/nodejs --strip-components=1
    rm /tmp/nodejs.tar.xz
EOF

FROM scratch

COPY --from=builder /opt/nodejs/ /