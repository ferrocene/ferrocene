# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

## This builds a docker image that contains only the binaries
## of the desired qnx toolchain at the root.

## You need to download and place the qnx toolchain package
# in the current folder before building.

## QNX_VERSION=qnx710-472
## aws s3 cp s3://ferrocene-ci-mirrors/manual/qnx/${QNX_VERSION}-deployment.tar.zst ${QNX_VERSION}-deployment.tar.zst
## docker build --tag harbor.infra.ferrous-systems.net/ferrocene-images/qnx:${QNX_VERSION}-linux --file qnx.dockerfile --build-arg QNX_VERSION=${QNX_VERSION} .
## docker push harbor.infra.ferrous-systems.net/ferrocene-images/qnx:${QNX_VERSION}-linux

ARG TARGETPLATFORM
ARG BUILDPLATFORM

FROM --platform=$BUILDPLATFORM ubuntu:24.04 AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG QNX_VERSION

RUN <<-EOF
    set -xe
    echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
    alias apt-install="apt-get install -yq --no-install-recommends --option Dpkg::Options::=--force-confnew"

    apt-get update

    # Needed to install AWS CLI during the build:
    apt-install \
      zstd
EOF

RUN mkdir -p /tmp/qnx
COPY $QNX_VERSION-deployment.tar.zst /tmp/qnx-deployment.tar.zst
RUN tar -C /tmp/qnx/ --strip=1 --exclude "*host/win64*" --exclude "*bat"  -xf /tmp/qnx-deployment.tar.zst

FROM scratch

COPY --from=builder /tmp/qnx/ /